import {
  Stack,
  StackProps,
  aws_rds as rds,
  aws_secretsmanager as sm,
} from "aws-cdk-lib";
import { SubnetType } from "aws-cdk-lib/aws-ec2";

import { Construct } from "constructs";
import { currentEnvConfig, deployEnv, projectName } from "../config/config";
import { VpcStack } from "./vpc";

export interface RdsStackProps extends StackProps {
  readonly vpcStack: VpcStack;
}

/**
 * Define RDS resources.
 */
export class RdsStack extends Stack {
  /**
   * Aurora Cluster
   */
  public readonly rdsCluster: rds.DatabaseCluster;
  /**
   * RDS Secret
   */
  public readonly rdsAdminSecret: sm.Secret;

  constructor(scope: Construct, id: string, props: RdsStackProps) {
    super(scope, id, props);

    const EXCLUDE_CHARACTERS = "\"@'%$#&().,{_?<≠^>[:;`+*!]}=~|¥/\\";

    const vpc = props.vpcStack.vpc;
    const publicSubnets = vpc.selectSubnets({
      subnetType: SubnetType.PUBLIC,
    });

    /**
     * RDS Admin User Secret
     */
    this.rdsAdminSecret = new sm.Secret(this, `RdsAdminSecret`, {
      secretName: `${projectName}-${deployEnv}/rds/admin-secret`,
      description: `${projectName}-${deployEnv} RDS Admin User Secret.`,
      generateSecretString: {
        excludeCharacters: EXCLUDE_CHARACTERS,
        generateStringKey: "password",
        passwordLength: 32,
        requireEachIncludedType: true,
        secretStringTemplate: '{"username": "postgresAdmin"}',
      },
    });

    /**
     * RDS Subnet Group
     */
    const subnetGroup = new rds.SubnetGroup(this, `SubnetGroup`, {
      description: `The subnet group to be used by Aurora in ${projectName}-${deployEnv}.`,
      vpc,
      subnetGroupName: `${projectName}-${deployEnv}`,
      // Make publicly accessible for development environments.
      vpcSubnets: publicSubnets,
    });

    /**
     * RDS Parameter Group
     */
    const parameterGroupName = `${projectName}-${deployEnv}`;
    const parameterGroup = new rds.ParameterGroup(this, `ParameterGroup`, {
      engine: rds.DatabaseClusterEngine.auroraPostgres({
        version: currentEnvConfig.auroraPostgresEngineVersion,
      }),
      description: `${projectName}-${deployEnv} Parameter group for aurora-postgresql.`,
    });
    parameterGroup.bindToInstance({});
    const cfnParameterGroup = parameterGroup.node
      .defaultChild as rds.CfnDBParameterGroup;
    cfnParameterGroup.addPropertyOverride(
      "DBParameterGroupName",
      parameterGroupName
    );

    /**
     * RDS Cluster
     */
    this.rdsCluster = new rds.DatabaseCluster(this, `RdsCluster`, {
      engine: rds.DatabaseClusterEngine.auroraPostgres({
        version: currentEnvConfig.auroraPostgresEngineVersion,
      }),
      credentials: rds.Credentials.fromSecret(rdsAdminSecret),
      clusterIdentifier: `${projectName}-${deployEnv}-cluster`,
      deletionProtection: false,
      iamAuthentication: true,
      readers: [
        rds.ClusterInstance.provisioned(`Reader1`, {
          instanceIdentifier: `${projectName}-${deployEnv}-reader-1`,
          instanceType: currentEnvConfig.auroraInstanceType,
          // Make publicly accessible for development environments.
          publiclyAccessible: true,
          parameterGroup,
        }),
      ],
      storageEncrypted: true,
      subnetGroup,
      vpc,
      writer: rds.ClusterInstance.provisioned(`Writer`, {
        instanceIdentifier: `${projectName}-${deployEnv}-writer`,
        instanceType: currentEnvConfig.auroraInstanceType,
        // Make publicly accessible for development environments.
        publiclyAccessible: true,
        parameterGroup,
      }),
    });
  }
}
