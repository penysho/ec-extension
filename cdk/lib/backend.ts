import * as cdk from "aws-cdk-lib";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as ecr from "aws-cdk-lib/aws-ecr";
import * as ecs from "aws-cdk-lib/aws-ecs";
import * as elasticloadbalancingv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as iam from "aws-cdk-lib/aws-iam";
import * as logs from "aws-cdk-lib/aws-logs";
import { currentEnvConfig, deployEnv, projectName } from "../config/config";
import { ElbStack } from "./elb";
import { RdsStack } from "./rds";
import { VpcStack } from "./vpc";

export interface BackendStackProps extends cdk.StackProps {
  readonly vpcStack: VpcStack;
  readonly elbStack: ElbStack;
  rdsStack: RdsStack;
}

/**
 * Define resources for the backend.
 */
export class BackendStack extends cdk.Stack {
  /**
   * ECR
   */
  public readonly repository: ecr.IRepository;
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

    const containerName = "backend";
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
            certificateArn: currentEnvConfig.certificateArn,
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
            certificateArn: currentEnvConfig.certificateArn,
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

    // ECR
    this.repository = new ecr.Repository(this, "Repository", {
      repositoryName: `${projectName}-${deployEnv}`,
      lifecycleRules: [
        {
          rulePriority: 1,
          description: "Expire images older than 3 generations",
          maxImageCount: 3,
          tagStatus: ecr.TagStatus.ANY,
        },
      ],
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      emptyOnDelete: true,
    });

    // Log Group
    const logGroup = new logs.LogGroup(this, "LogGroup", {
      retention: logs.RetentionDays.THREE_MONTHS,
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
    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
      managedPolicies: [
        {
          managedPolicyArn:
            "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy",
        },
      ],
    });

    const taskDefinition = new ecs.FargateTaskDefinition(
      this,
      "TaskDefinition",
      {
        cpu: currentEnvConfig.ecsTaskCpu,
        memoryLimitMiB: currentEnvConfig.ecsTaskMemory,
        executionRole: taskExecutionRole,
        family: `${projectName}-backend-${deployEnv}`,
      }
    );
    const container = taskDefinition.addContainer(containerName, {
      containerName,
      image: ecs.ContainerImage.fromEcrRepository(
        this.repository,
        currentEnvConfig.backendImageTag
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
      }),
    });
    container.addEnvironment("JWT_SECRET", "secret");

    // Service
    const service = new ecs.FargateService(this, "Service", {
      cluster: this.cluster,
      serviceName: `${projectName}-backend-${deployEnv}`,
      taskDefinition,
      desiredCount: currentEnvConfig.ecsServiceDesiredCount,
      deploymentController: {
        type: ecs.DeploymentControllerType.CODE_DEPLOY,
      },
      enableExecuteCommand: true,
      assignPublicIp: true,
      // Security groups that allow communication from the ALB to the container are automatically granted
      vpcSubnets: {
        // To retrieve images from ECR
        subnets: vpc.publicSubnets,
      },
    });
    this.service = service;

    // Register the service with the blue target group
    this.blueTargetGroup.addTarget(service);

    props.rdsStack.rdsCluster.connections.allowFrom(
      service,
      ec2.Port.tcp(5432),
      "Allow access to RDS from the ECS service"
    );
  }
}
