#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { currentEnvConfig, deployEnv, projectName } from "../config/config";
import { BackendStack } from "../lib/backend";
import { CiStack } from "../lib/ci";
import { CognitoStack } from "../lib/cognito";
import { ElbStack } from "../lib/elb";
import { RdsStack } from "../lib/rds";
import { VpcStack } from "../lib/vpc";

const app = new cdk.App();

const envProps = {
  account: process.env.CDK_DEFAULT_ACCOUNT,
  region: process.env.CDK_DEFAULT_REGION,
};

// Get Value from context
const backendImageTag = app.node.tryGetContext("backendImageTag");
currentEnvConfig.backendImageTag = backendImageTag;

// Define Stacks
const vpcStack = new VpcStack(app, `${projectName}-${deployEnv}-vpc`, {});

const elbStack = new ElbStack(app, `${projectName}-${deployEnv}-elb`, {
  env: envProps,
  vpcStack: vpcStack,
});

const rdsStack = new RdsStack(app, `${projectName}-${deployEnv}-rds`, {
  env: envProps,
  vpcStack: vpcStack,
});

const cognitoStack = new CognitoStack(
  app,
  `${projectName}-${deployEnv}-cognito`,
  {
    env: envProps,
  }
);

const backendStack = new BackendStack(
  app,
  `${projectName}-${deployEnv}-backend`,
  {
    env: envProps,
    vpcStack: vpcStack,
    elbStack: elbStack,
    rdsStack: rdsStack,
    cognitoStack: cognitoStack,
  }
);

new CiStack(app, `${projectName}-${deployEnv}-ci`, {
  env: envProps,
  backendStack: backendStack,
});
