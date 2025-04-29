/**
 * Static values that do not depend on resources defined in our project are defined here and used from each stack.
 * Dynamic variables that depend on resources should be passed to each stack as props.
 */

import { aws_ec2 as ec2, aws_rds as rds } from "aws-cdk-lib";

// Define common configuration values for the project.
export const projectName: string = "ec-extension";

export const envCodes = ["dev", "tst", "prd"] as const;
export type EnvCode = (typeof envCodes)[number];

const getDeployEnv = () => {
  const env = process.env.DEPLOY_ENV;
  if (envCodes.includes(env as EnvCode)) {
    return env as EnvCode;
  }
  return "tst";
};
export const deployEnv: EnvCode = getDeployEnv();

// Define different settings for each deployment environment in the project.
export interface EnvConfig {
  backendImageTag: string;
  apiDomain: string;
  certificateArn: string;
  branch: string;
  ecsTaskCpu: number;
  ecsTaskMemory: number;
  ecsServiceDesiredCount: number;
  auroraPostgresEngineVersion: rds.AuroraPostgresEngineVersion;
  auroraInstanceType: ec2.InstanceType;
}

export const envConfig: Record<EnvCode, EnvConfig> = {
  dev: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
    ecsTaskCpu: 256,
    ecsTaskMemory: 512,
    ecsServiceDesiredCount: -1,
    auroraPostgresEngineVersion: rds.AuroraPostgresEngineVersion.VER_17_1,
    auroraInstanceType: ec2.InstanceType.of(
      ec2.InstanceClass.T4G,
      ec2.InstanceSize.MEDIUM
    ),
  },
  tst: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
    ecsTaskCpu: 256,
    ecsTaskMemory: 512,
    ecsServiceDesiredCount: 1,
    auroraPostgresEngineVersion: rds.AuroraPostgresEngineVersion.VER_17_1,
    auroraInstanceType: ec2.InstanceType.of(
      ec2.InstanceClass.T4G,
      ec2.InstanceSize.MEDIUM
    ),
  },
  prd: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
    ecsTaskCpu: 256,
    ecsTaskMemory: 512,
    ecsServiceDesiredCount: 1,
    auroraPostgresEngineVersion: rds.AuroraPostgresEngineVersion.VER_17_1,
    auroraInstanceType: ec2.InstanceType.of(
      ec2.InstanceClass.T4G,
      ec2.InstanceSize.MEDIUM
    ),
  },
};

export const currentEnvConfig: EnvConfig = envConfig[deployEnv];
