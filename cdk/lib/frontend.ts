import * as cdk from "aws-cdk-lib";
import { CfnApp, CfnBranch, CfnDomain } from "aws-cdk-lib/aws-amplify";
import { BuildSpec } from "aws-cdk-lib/aws-codebuild";
import * as cognito from "aws-cdk-lib/aws-cognito";
import * as iam from "aws-cdk-lib/aws-iam";
import * as rum from "aws-cdk-lib/aws-rum";
import { config, deployEnv, projectName } from "../config/config";
import { CognitoStack } from "./cognito";
import { ElbStack } from "./elb";

interface FrontendStackProps extends cdk.StackProps {
  readonly elbStack: ElbStack;
  readonly cognitoStack: CognitoStack;
}

/**
 * Define resources for the frontend.
 */
export class FrontendStack extends cdk.Stack {
  /**
   * Amplify
   */
  public readonly amplify: CfnApp;
  /**
   * Green Amplify
   */
  public readonly greenAmplify: CfnApp;
  /**
   * CloudWatch RUM App Monitor
   */
  public readonly rumAppMonitor: rum.CfnAppMonitor;
  /**
   * CloudWatch RUM Green App Monitor
   */
  public readonly rumGreenAppMonitor: rum.CfnAppMonitor;

  constructor(scope: cdk.App, id: string, props: FrontendStackProps) {
    super(scope, id, props);

    // https://docs.aws.amazon.com/ja_jp/amplify/latest/userguide/monitoring-with-cloudwatch.html#ssr-logs
    const amplifyRole = new iam.Role(this, "AmplifyRole", {
      assumedBy: new iam.ServicePrincipal("amplify.amazonaws.com"),
    });
    amplifyRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
          "logs:DescribeLogGroups",
        ],
        resources: ["*"],
      })
    );

    // Cognito Identity Pool for CloudWatch RUM authentication
    const identityPool = new cognito.CfnIdentityPool(this, "RumIdentityPool", {
      identityPoolName: `${projectName}-${deployEnv}-rum-identity-pool`,
      allowUnauthenticatedIdentities: true,
    });

    // IAM role for unauthenticated users to publish RUM data
    const unauthenticatedRole = new iam.Role(this, "RumUnauthenticatedRole", {
      assumedBy: new iam.FederatedPrincipal(
        "cognito-identity.amazonaws.com",
        {
          StringEquals: {
            "cognito-identity.amazonaws.com:aud": identityPool.ref,
          },
          "ForAnyValue:StringLike": {
            "cognito-identity.amazonaws.com:amr": "unauthenticated",
          },
        },
        "sts:AssumeRoleWithWebIdentity"
      ),
      inlinePolicies: {
        RumPolicy: new iam.PolicyDocument({
          statements: [
            new iam.PolicyStatement({
              effect: iam.Effect.ALLOW,
              actions: ["rum:PutRumEvents"],
              resources: ["*"],
            }),
          ],
        }),
      },
    });

    // Attach role to identity pool
    new cognito.CfnIdentityPoolRoleAttachment(
      this,
      "RumIdentityPoolRoleAttachment",
      {
        identityPoolId: identityPool.ref,
        roles: {
          unauthenticated: unauthenticatedRole.roleArn,
        },
      }
    );

    // CloudWatch RUM App Monitor for main app
    this.rumAppMonitor = new rum.CfnAppMonitor(this, "RumAppMonitor", {
      name: `${projectName}-${deployEnv}-realshop-rum`,
      domain: `${projectName}-${deployEnv}-realshop.${config.frontendDomain}`,
      cwLogEnabled: true,
      appMonitorConfiguration: {
        allowCookies: true,
        enableXRay: true,
        excludedPages: ["/admin/*", "/api/*"],
        favoritePages: ["/", "/products", "/cart", "/checkout"],
        guestRoleArn: unauthenticatedRole.roleArn,
        identityPoolId: identityPool.ref,
        includedPages: ["*"],
        sessionSampleRate: 1.0,
        telemetries: ["errors", "performance", "http"],
      },
    });

    // CloudWatch RUM App Monitor for green deployment
    this.rumGreenAppMonitor = new rum.CfnAppMonitor(
      this,
      "RumGreenAppMonitor",
      {
        name: `${projectName}-${deployEnv}-realshop-green-rum`,
        domain: `${projectName}-${deployEnv}-realshop-green.${config.frontendDomain}`,
        cwLogEnabled: true,
        appMonitorConfiguration: {
          allowCookies: true,
          enableXRay: true,
          excludedPages: ["/admin/*", "/api/*"],
          favoritePages: ["/", "/products", "/cart", "/checkout"],
          guestRoleArn: unauthenticatedRole.roleArn,
          identityPoolId: identityPool.ref,
          includedPages: ["*"],
          sessionSampleRate: 1.0,
          telemetries: ["errors", "performance", "http"],
        },
      }
    );

    this.amplify = new CfnApp(this, "Amplify", {
      name: "realshop",
      accessToken: config.githubToken,
      iamServiceRole: amplifyRole.roleArn,
      repository: "https://github.com/penysho/ec-extension",
      environmentVariables: [
        {
          name: "AMPLIFY_MONOREPO_APP_ROOT",
          value: "frontend/realshop",
        },
        {
          name: "NEXT_PUBLIC_BACKEND_ENDPOINT",
          value: `https://${projectName}-${deployEnv}-api.${config.apiDomain}`,
        },
        {
          name: "NEXT_PUBLIC_AUTH_USER_POOL_ID",
          value: props.cognitoStack.userPool.userPoolId,
        },
        {
          name: "NEXT_PUBLIC_AUTH_USER_POOL_CLIENT_ID",
          value: props.cognitoStack.userPoolClient.userPoolClientId,
        },
        {
          name: "NEXT_PUBLIC_RUM_APP_MONITOR_ID",
          value: this.rumAppMonitor.attrId,
        },
        {
          name: "NEXT_PUBLIC_RUM_GUEST_ROLE_ARN",
          value: unauthenticatedRole.roleArn,
        },
        {
          name: "NEXT_PUBLIC_RUM_IDENTITY_POOL_ID",
          value: identityPool.ref,
        },
        {
          name: "NEXT_PUBLIC_RUM_REGION",
          value: this.region,
        },
      ],
      buildSpec: BuildSpec.fromObjectToYaml({
        version: 1,
        applications: [
          {
            appRoot: "frontend/realshop",
            frontend: {
              phases: {
                preBuild: {
                  commands: ["nvm install 22.8", "nvm use 22.8", "npm install"],
                },
                build: {
                  commands: ["npm run build"],
                },
              },
              artifacts: {
                baseDirectory: ".next",
                files: ["**/*"],
              },
              cache: {
                paths: ["node_modules/**/*"],
              },
            },
          },
        ],
      }).toBuildSpec(),
      platform: "WEB_COMPUTE",
      customRules: [
        {
          source: "/<*>",
          target: "/index.html",
          status: "404-200",
        },
      ],
    });

    const branch = new CfnBranch(this, "Branch", {
      appId: this.amplify.attrAppId,
      branchName: config.branch,
      framework: "Next.js - SSR",
      enableAutoBuild: false,
      stage: "PRODUCTION",
    });

    new CfnDomain(this, "AuthHubDomain", {
      appId: this.amplify.attrAppId,
      domainName: `${projectName}-${deployEnv}-realshop.${config.frontendDomain}`,
      enableAutoSubDomain: true,
      subDomainSettings: [
        {
          prefix: "",
          branchName: branch.branchName,
        },
      ],
    });

    this.greenAmplify = new CfnApp(this, "GreenAmplify", {
      name: "realshop-green",
      accessToken: config.githubToken,
      iamServiceRole: amplifyRole.roleArn,
      repository: "https://github.com/penysho/ec-extension",
      environmentVariables: [
        {
          name: "AMPLIFY_MONOREPO_APP_ROOT",
          value: "frontend/realshop",
        },
        {
          name: "NEXT_PUBLIC_BACKEND_ENDPOINT",
          value: `https://${projectName}-${deployEnv}-api.${config.apiDomain}:10443`,
        },
        {
          name: "NEXT_PUBLIC_AUTH_USER_POOL_ID",
          value: props.cognitoStack.userPool.userPoolId,
        },
        {
          name: "NEXT_PUBLIC_AUTH_USER_POOL_CLIENT_ID",
          value: props.cognitoStack.userPoolClient.userPoolClientId,
        },
        {
          name: "NEXT_PUBLIC_RUM_APP_MONITOR_ID",
          value: this.rumGreenAppMonitor.attrId,
        },
        {
          name: "NEXT_PUBLIC_RUM_GUEST_ROLE_ARN",
          value: unauthenticatedRole.roleArn,
        },
        {
          name: "NEXT_PUBLIC_RUM_IDENTITY_POOL_ID",
          value: identityPool.ref,
        },
        {
          name: "NEXT_PUBLIC_RUM_REGION",
          value: this.region,
        },
      ],
      buildSpec: BuildSpec.fromObjectToYaml({
        version: 1,
        applications: [
          {
            appRoot: "frontend/realshop",
            frontend: {
              phases: {
                preBuild: {
                  commands: ["nvm install 22.8", "nvm use 22.8", "npm install"],
                },
                build: {
                  commands: ["npm run build"],
                },
              },
              artifacts: {
                baseDirectory: ".next",
                files: ["**/*"],
              },
              cache: {
                paths: ["node_modules/**/*"],
              },
            },
          },
        ],
      }).toBuildSpec(),
      platform: "WEB_COMPUTE",
      customRules: [
        {
          source: "/<*>",
          target: "/index.html",
          status: "404-200",
        },
      ],
    });

    const greenBranch = new CfnBranch(this, "GreenBranch", {
      appId: this.greenAmplify.attrAppId,
      branchName: config.branch,
      framework: "Next.js - SSR",
      enableAutoBuild: false,
      stage: "PRODUCTION",
    });

    new CfnDomain(this, "GreenDomain", {
      appId: this.greenAmplify.attrAppId,
      domainName: `${projectName}-${deployEnv}-realshop-green.${config.frontendDomain}`,
      enableAutoSubDomain: true,
      subDomainSettings: [
        {
          prefix: "",
          branchName: greenBranch.branchName,
        },
      ],
    });
  }
}
