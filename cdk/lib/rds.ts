import {
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_logs as logs,
  aws_rds as rds,
  aws_secretsmanager as sm,
} from "aws-cdk-lib";
import { SubnetType } from "aws-cdk-lib/aws-ec2";

import { Construct } from "constructs";
import { config, deployEnv, projectName } from "../config/config";
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
  public readonly rdsCluster: rds.IDatabaseCluster;
  /**
   * Security groups to attach to clients to make RDS accessible.
   */
  public readonly rdsClientSg: ec2.SecurityGroup;
  /**
   * RDS Admin User Secret
   */
  public readonly rdsAdminSecret: sm.Secret;
  /**
   * RDS Application User Secret
   */
  public readonly rdsApplicationSecret: sm.Secret;

  constructor(scope: Construct, id: string, props: RdsStackProps) {
    super(scope, id, props);

    const EXCLUDE_CHARACTERS = "\"@'%$#&().,{_?<≠^>[:;`+*!]}=~|¥/\\";
    const DATABASE_NAME = "ec_extension";

    const vpc = props.vpcStack.vpc;
    const publicSubnets = vpc.selectSubnets({
      subnetType: SubnetType.PUBLIC,
    });

    /**
     * Security Group for RDS client
     */
    this.rdsClientSg = new ec2.SecurityGroup(this, `RdsClientSg`, {
      vpc,
      securityGroupName: `${projectName}-${deployEnv}-rds-client`,
      description: `${projectName}-${deployEnv} RDS Client Security Group.`,
      allowAllOutbound: true,
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
        version: config.auroraPostgresEngineVersion,
      }),
      description: `${projectName}-${deployEnv} Parameter group for aurora-postgresql.`,
      parameters: {
        general_log: "1",
        slow_query_log: "1",
        long_query_time: "10",
        log_output: "FILE",
      },
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
        version: config.auroraPostgresEngineVersion,
      }),
      credentials: rds.Credentials.fromSecret(this.rdsAdminSecret),
      clusterIdentifier: `${projectName}-${deployEnv}-cluster`,
      deletionProtection: false,
      iamAuthentication: true,
      serverlessV2MaxCapacity: 1,
      serverlessV2MinCapacity: 0,
      // readers: config.createReaderInstance
      //   ? [
      //       rds.ClusterInstance.provisioned(`Reader1`, {
      //         instanceIdentifier: `${projectName}-${deployEnv}-reader-1`,
      //         instanceType: config.auroraInstanceType,
      //         // Make publicly accessible for development environments.
      //         publiclyAccessible: true,
      //         parameterGroup,
      //       }),
      //     ]
      //   : undefined,
      readers: config.createReaderInstance
        ? [
            rds.ClusterInstance.provisioned(`Reader1`, {
              instanceIdentifier: `${projectName}-${deployEnv}-reader-1`,
              instanceType: config.auroraInstanceType,
              // Make publicly accessible for development environments.
              publiclyAccessible: true,
              parameterGroup,
            }),
          ]
        : undefined,
      storageEncrypted: true,
      subnetGroup,
      vpc,
      // writer: rds.ClusterInstance.provisioned(`Writer`, {
      //   instanceIdentifier: `${projectName}-${deployEnv}-writer`,
      //   instanceType: config.auroraInstanceType,
      //   // Make publicly accessible for development environments.
      //   publiclyAccessible: true,
      //   parameterGroup,
      // }),
      writer: rds.ClusterInstance.serverlessV2(`Writer`, {
        instanceIdentifier: `${projectName}-${deployEnv}-writer`,
        // Make publicly accessible for development environments.
        publiclyAccessible: true,
        parameterGroup,
      }),
      // https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CloudwatchLogsExportConfiguration.html
      cloudwatchLogsExports: ["postgresql"],
      cloudwatchLogsRetention: logs.RetentionDays.ONE_WEEK,
    });

    this.rdsCluster.connections.allowFrom(
      this.rdsClientSg,
      ec2.Port.tcp(5432),
      "Allow access to RDS from the RDS client security group"
    );

    /**
     * RDS Application User Secret
     */
    this.rdsApplicationSecret = new sm.Secret(this, `RdsApplicationSecret`, {
      secretName: `${projectName}-${deployEnv}/rds/application-secret`,
      description: `${projectName}-${deployEnv} RDS Admin User Secret.`,
      generateSecretString: {
        excludeCharacters: EXCLUDE_CHARACTERS,
        generateStringKey: "password",
        passwordLength: 32,
        requireEachIncludedType: true,
        secretStringTemplate: `{"engine": "postgres", "username": "application", "host": "${this.rdsCluster.clusterEndpoint.hostname}", "port": "${this.rdsCluster.clusterEndpoint.port}", "dbname": "${DATABASE_NAME}"}`,
      },
    });
  }
}
