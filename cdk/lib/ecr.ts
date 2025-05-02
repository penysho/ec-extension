import * as cdk from "aws-cdk-lib";
import * as ecr from "aws-cdk-lib/aws-ecr";
import { deployEnv, projectName } from "../config/config";

export interface EcrStackProps extends cdk.StackProps {}

/**
 * Define resources for the backend.
 */
export class EcrStack extends cdk.Stack {
  /**
   * ECR
   */
  public readonly backendRepository: ecr.IRepository;
  public readonly migrationRepository: ecr.IRepository;

  public constructor(scope: cdk.App, id: string, props: EcrStackProps) {
    super(scope, id, props);

    // ECR
    this.backendRepository = new ecr.Repository(this, "BackendRepository", {
      repositoryName: `${projectName}-backend-${deployEnv}`,
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

    this.migrationRepository = new ecr.Repository(this, "MigrationRepository", {
      repositoryName: `${projectName}-backend-migration-${deployEnv}`,
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
  }
}
