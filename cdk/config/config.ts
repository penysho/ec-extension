/**
 * Static values that do not depend on resources defined in our project are defined here and used from each stack.
 * Dynamic variables that depend on resources should be passed to each stack as props.
 */

// Define common configuration values for the project.
export const projectName: string = "sso-demo";

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
}

export const envConfig: Record<EnvCode, EnvConfig> = {
  dev: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
  },
  tst: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
  },
  prd: {
    backendImageTag: "latest",
    apiDomain: "pesh-igpjt.com",
    certificateArn:
      "arn:aws:acm:ap-northeast-1:551152530614:certificate/78e1479b-2bb2-4f89-8836-a8ff91227dfb",
    branch: "main",
  },
};

export const currentEnvConfig: EnvConfig = envConfig[deployEnv];
