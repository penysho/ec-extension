import * as cdk from "aws-cdk-lib";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as elasticloadbalancingv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as iam from "aws-cdk-lib/aws-iam";
import * as logs from "aws-cdk-lib/aws-logs";
import { config, deployEnv, projectName } from "../config/config";
import { CognitoStack } from "./cognito";
import { EcrStack } from "./ecr";
import { ElbStack } from "./elb";
import { RdsStack } from "./rds";
import { VpcStack } from "./vpc";

export interface BackendStackProps extends cdk.StackProps {
  readonly vpcStack: VpcStack;
  readonly elbStack: ElbStack;
  readonly rdsStack: RdsStack;
  readonly cognitoStack: CognitoStack;
  readonly ecrStack: EcrStack;
}

/**
 * Define resources for the backend.
 */
export class BackendStack extends cdk.Stack {
  /**
   * ECS Cluster
   */
  public readonly cluster: ecs.ICluster;
  /**
   * ECS Service
   */
  public readonly service: ecs.IBaseService;
  /**
   * Listener ARN for port 80 used by ALB in applications.
   */
  public readonly elb80Listener: elasticloadbalancingv2.IApplicationListener;
  /**
   * Listener ARN for port 443 used by ALB in applications.
   */
  public readonly elb443Listener: elasticloadbalancingv2.IApplicationListener;
  /**
   * This is the group ID of the security group for the ALB target of applications.
   */
  public readonly greenListener: elasticloadbalancingv2.IApplicationListener;
  /**
   * Blue Target Group
   */
  public readonly blueTargetGroup: elasticloadbalancingv2.IApplicationTargetGroup;
  /**
   * Green Target Group
   */
  public readonly greenTargetGroup: elasticloadbalancingv2.IApplicationTargetGroup;

  public constructor(scope: cdk.App, id: string, props: BackendStackProps) {
    super(scope, id, props);

    const vpc = props.vpcStack.vpc;

    const containerPort = 8080;

    // Resources

    // Listeners
    this.elb443Listener = new elasticloadbalancingv2.ApplicationListener(
      this,
      "Elb443Listener",
      {
        loadBalancer: props.elbStack.loadBalancer,
        // This creates a security group that allows access from the public
        open: true,
        defaultAction: elasticloadbalancingv2.ListenerAction.fixedResponse(
          403,
          { contentType: "text/plain" }
        ),
        port: 443,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTPS,
        certificates: [
          {
            certificateArn: config.certificateArn,
          },
        ],
      }
    );

    this.elb80Listener = new elasticloadbalancingv2.ApplicationListener(
      this,
      "Elb80Listener",
      {
        loadBalancer: props.elbStack.loadBalancer,
        open: false,
        defaultAction: elasticloadbalancingv2.ListenerAction.fixedResponse(
          403,
          { contentType: "text/plain" }
        ),
        port: 80,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTP,
      }
    );

    this.greenListener = new elasticloadbalancingv2.ApplicationListener(
      this,
      "GreenListener",
      {
        loadBalancer: props.elbStack.loadBalancer,
        open: false,
        port: 10443,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTPS,
        defaultAction: elasticloadbalancingv2.ListenerAction.fixedResponse(
          403,
          { contentType: "text/plain" }
        ),
        certificates: [
          {
            certificateArn: config.certificateArn,
          },
        ],
      }
    );

    // Target Groups
    this.blueTargetGroup = new elasticloadbalancingv2.ApplicationTargetGroup(
      this,
      "BlueTargetGroup",
      {
        vpc,
        port: containerPort,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTP,
        targetType: elasticloadbalancingv2.TargetType.IP,
        healthCheck: {
          path: "/health",
          port: containerPort.toString(),
        },
      }
    );
    this.elb443Listener.addAction(`${projectName}-${deployEnv}-blue`, {
      priority: 1,
      conditions: [
        elasticloadbalancingv2.ListenerCondition.pathPatterns(["*"]),
      ],
      action: elasticloadbalancingv2.ListenerAction.forward([
        this.blueTargetGroup,
      ]),
    });

    this.greenTargetGroup = new elasticloadbalancingv2.ApplicationTargetGroup(
      this,
      "GreenTargetGroup",
      {
        vpc,
        port: containerPort,
        protocol: elasticloadbalancingv2.ApplicationProtocol.HTTP,
        targetType: elasticloadbalancingv2.TargetType.IP,
        healthCheck: {
          path: "/health",
          port: containerPort.toString(),
        },
      }
    );
    this.greenListener.addAction(`${projectName}-${deployEnv}-green`, {
      priority: 1,
      conditions: [
        elasticloadbalancingv2.ListenerCondition.pathPatterns(["*"]),
      ],
      action: elasticloadbalancingv2.ListenerAction.forward([
        this.greenTargetGroup,
      ]),
    });

    // Cluster
    this.cluster = new ecs.Cluster(this, "Cluster", {
      vpc,
      clusterName: `${projectName}-${deployEnv}`,
    });

    // Log Group
    const logGroup = new logs.LogGroup(this, "LogGroup", {
      retention: logs.RetentionDays.THREE_MONTHS,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    const metricFilterForServerError = new logs.CfnMetricFilter(
      this,
      "MetricFilterForServerError",
      {
        filterName: "server-error",
        filterPattern: "?ERROR ?error ?Error",
        logGroupName: logGroup.logGroupName,
        metricTransformations: [
          {
            metricValue: "1",
            metricNamespace: `${projectName}-${deployEnv}`,
            metricName: `${projectName}-${deployEnv}-server-error`,
          },
        ],
      }
    );
    metricFilterForServerError.cfnOptions.deletionPolicy =
      cdk.CfnDeletionPolicy.DELETE;

    // Task definition

    // https://aws-otel.github.io/docs/setup/ecs/create-iam-role
    const executionRole = new iam.Role(this, "ExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      managedPolicies: [
        {
          managedPolicyArn:
            "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy",
        },
        {
          managedPolicyArn: "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess",
        },
        {
          managedPolicyArn: "arn:aws:iam::aws:policy/AmazonSSMReadOnlyAccess",
        },
      ],
    });

    // https://aws-otel.github.io/docs/setup/permissions#create-iam-policy
    const taskRole = new iam.Role(this, "TaskRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      managedPolicies: [
        {
          managedPolicyArn: "arn:aws:iam::aws:policy/AWSXrayWriteOnlyAccess",
        },
      ],
    });
    taskRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "logs:PutLogEvents",
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:DescribeLogStreams",
          "logs:DescribeLogGroups",
          "logs:PutRetentionPolicy",
          "ssm:GetParameters",
        ],
        resources: ["*"],
        effect: iam.Effect.ALLOW,
      })
    );

    const taskDefinition = new ecs.FargateTaskDefinition(
      this,
      "TaskDefinition",
      {
        cpu: config.ecsTaskCpu,
        memoryLimitMiB: config.ecsTaskMemory,
        executionRole,
        taskRole,
        family: `${projectName}-backend-${deployEnv}`,
      }
    );
    const backendContainer = taskDefinition.addContainer("backend", {
      containerName: "backend",
      image: ecs.ContainerImage.fromEcrRepository(
        props.ecrStack.backendRepository,
        config.backendImageTag
      ),
      essential: true,
      portMappings: [
        {
          containerPort: containerPort,
          hostPort: containerPort,
          protocol: ecs.Protocol.TCP,
        },
      ],
      logging: ecs.LogDrivers.awsLogs({
        logGroup,
        streamPrefix: "ecs",
        mode: ecs.AwsLogDriverMode.NON_BLOCKING,
      }),
    });
    backendContainer.addEnvironment("RUST_LOG", config.appConfig.rustLog);
    backendContainer.addEnvironment("STORE_URL", config.appConfig.storeUrl);
    backendContainer.addEnvironment(
      "ACCESS_TOKEN",
      config.appConfig.accessToken
    );
    backendContainer.addEnvironment(
      "COGNITO_USER_POOL_ID",
      props.cognitoStack.userPool.userPoolId
    );
    backendContainer.addEnvironment(
      "COGNITO_CLIENT_ID",
      props.cognitoStack.userPoolClient.userPoolClientId
    );
    backendContainer.addEnvironment(
      "COGNITO_REGION",
      props.env?.region ?? "ap-northeast-1"
    );
    backendContainer.addEnvironment(
      "COGNITO_JWKS_URI",
      `https://cognito-idp.${props.env?.region}.amazonaws.com/${props.cognitoStack.userPool.userPoolId}/.well-known/jwks.json`
    );
    backendContainer.addEnvironment(
      "DATABASE_URL",
      `postgres://${props.rdsStack.rdsAdminSecret
        .secretValueFromJson("username")
        .unsafeUnwrap()}:${props.rdsStack.rdsAdminSecret
        .secretValueFromJson("password")
        .unsafeUnwrap()}@${
        props.rdsStack.rdsCluster.clusterEndpoint.hostname
      }:${props.rdsStack.rdsCluster.clusterEndpoint.port}/postgres`
    );
    // // When using the X-Ray daemon
    // backendContainer.addEnvironment(
    //   "AWS_XRAY_DAEMON_ADDRESS",
    //   "127.0.0.1:2000"
    // );
    backendContainer.addEnvironment(
      "OPENTELEMETRY_ENDPOINT",
      "http://localhost:4318/v1/traces"
    );

    const migrationContainer = taskDefinition.addContainer("migration", {
      containerName: "migration",
      image: ecs.ContainerImage.fromEcrRepository(
        props.ecrStack.migrationRepository,
        config.backendImageTag
      ),
      essential: false,
      logging: ecs.LogDrivers.awsLogs({
        logGroup,
        streamPrefix: "ecs",
        mode: ecs.AwsLogDriverMode.NON_BLOCKING,
      }),
      command: [
        "/app/target/release/migration",
        config.executeMigration ? "up" : "status",
      ],
    });
    migrationContainer.addEnvironment(
      "DATABASE_URL",
      `postgres://${props.rdsStack.rdsAdminSecret
        .secretValueFromJson("username")
        .unsafeUnwrap()}:${props.rdsStack.rdsAdminSecret
        .secretValueFromJson("password")
        .unsafeUnwrap()}@${
        props.rdsStack.rdsCluster.clusterEndpoint.hostname
      }:${props.rdsStack.rdsCluster.clusterEndpoint.port}/postgres`
    );

    // // https://docs.aws.amazon.com/ja_jp/xray/latest/devguide/xray-daemon-ecs.html#xray-daemon-ecs-image
    // // When using the X-Ray daemon
    // const xrayDaemonContainer = taskDefinition.addContainer("xray-daemon", {
    //   containerName: "xray-daemon",
    //   image: ecs.ContainerImage.fromRegistry("amazon/aws-xray-daemon"),
    //   essential: false,
    //   portMappings: [
    //     {
    //       containerPort: 2000,
    //       hostPort: 2000,
    //       protocol: ecs.Protocol.UDP,
    //     },
    //   ],
    //   logging: ecs.LogDrivers.awsLogs({
    //     logGroup,
    //     streamPrefix: "ecs",
    //     mode: ecs.AwsLogDriverMode.NON_BLOCKING,
    //   }),
    // });

    taskDefinition.addContainer("aws-otel-collector", {
      containerName: "aws-otel-collector",
      image: ecs.ContainerImage.fromRegistry(
        "public.ecr.aws/aws-observability/aws-otel-collector:v0.43.2"
      ),
      essential: true,
      command: ["--config", "/etc/ecs/ecs-cloudwatch-xray.yaml"],
      logging: ecs.LogDrivers.awsLogs({
        logGroup,
        streamPrefix: "ecs",
        mode: ecs.AwsLogDriverMode.NON_BLOCKING,
      }),
    });

    // Service
    const service = new ecs.FargateService(this, "Service", {
      cluster: this.cluster,
      serviceName: `${projectName}-backend-${deployEnv}`,
      taskDefinition,
      desiredCount: config.ecsServiceDesiredCount,
      deploymentController: {
        type: ecs.DeploymentControllerType.CODE_DEPLOY,
      },
      enableExecuteCommand: true,
      assignPublicIp: true,
      // Security groups that allow communication from the ALB to the container are automatically granted
      securityGroups: [props.rdsStack.rdsClientSg],
      vpcSubnets: {
        // To retrieve images from ECR
        subnets: vpc.publicSubnets,
      },
    });
    this.service = service;

    // Register the service with the blue target group
    this.blueTargetGroup.addTarget(service);
  }
}
