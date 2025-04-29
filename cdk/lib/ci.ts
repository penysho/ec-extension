import * as cdk from "aws-cdk-lib";
import * as codedeploy from "aws-cdk-lib/aws-codedeploy";
import { Construct } from "constructs";
import { deployEnv, projectName } from "../config/config";
import { BackendStack } from "./backend";

interface CiStackProps extends cdk.StackProps {
  readonly backendStack: BackendStack;
}

/**
 * Define resources for the CI project.
 */
export class CiStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: CiStackProps) {
    super(scope, id, props);

    // CodeDeploy
    const application = new codedeploy.EcsApplication(
      this,
      "CodeDeployApplication",
      {
        applicationName: `${projectName}-${deployEnv}`,
      }
    );

    // CodeDeploy DeploymentGroup
    new codedeploy.EcsDeploymentGroup(this, "DeploymentGroup", {
      application,
      autoRollback: {
        // CodeDeploy will automatically roll back if the 8-hour approval period times out and the deployment stops
        stoppedDeployment: true,
      },
      blueGreenDeploymentConfig: {
        // The deployment will wait for approval for up to 8 hours before stopping the deployment
        deploymentApprovalWaitTime: cdk.Duration.hours(8),
        terminationWaitTime: cdk.Duration.minutes(30),
        blueTargetGroup: props.backendStack.blueTargetGroup,
        greenTargetGroup: props.backendStack.greenTargetGroup,
        listener: props.backendStack.elb443Listener,
        testListener: props.backendStack.greenListener,
      },
      deploymentConfig: codedeploy.EcsDeploymentConfig.ALL_AT_ONCE,
      deploymentGroupName: `${projectName}-${deployEnv}`,
      service: props.backendStack.service,
    });
  }
}
