import * as cdk from "aws-cdk-lib";
import { CfnApp, CfnBranch, CfnDomain } from "aws-cdk-lib/aws-amplify";
import { BuildSpec } from "aws-cdk-lib/aws-codebuild";
import * as iam from "aws-cdk-lib/aws-iam";
import { config, deployEnv, projectName } from "../config/config";
import { ElbStack } from "./elb";

interface FrontendStackProps extends cdk.StackProps {
  readonly elbStack: ElbStack;
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
