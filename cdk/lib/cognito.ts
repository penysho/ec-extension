import * as cdk from "aws-cdk-lib";
import * as cognito from "aws-cdk-lib/aws-cognito";
import { Construct } from "constructs";
import { deployEnv, projectName } from "../config/config";

export interface CognitoStackProps extends cdk.StackProps {}

/**
 * Define resources for the Cognito project.
 */
export class CognitoStack extends cdk.Stack {
  /**
   * User Pool
   */
  public readonly userPool: cognito.UserPool;

  constructor(scope: Construct, id: string, props: CognitoStackProps) {
    super(scope, id, props);

    this.userPool = new cognito.UserPool(this, "UserPool", {
      selfSignUpEnabled: true,
      userVerification: {
        emailSubject: "Verify your email for our app!",
        emailBody:
          "Thanks for signing up to our app! Your verification code is {####}",
        emailStyle: cognito.VerificationEmailStyle.CODE,
      },
      signInAliases: {
        email: true,
      },
    });

    this.userPool.addClient("UserPoolClient", {
      authFlows: {
        adminUserPassword: true,
        userPassword: true,
      },
    });

    this.userPool.addDomain("UserPoolDomain", {
      cognitoDomain: {
        domainPrefix: `${projectName}-${deployEnv}`,
      },
    });
  }
}
