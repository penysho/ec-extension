import * as cdk from "aws-cdk-lib";
import * as elasticloadbalancingv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import { config, deployEnv, projectName } from "../config/config";
import { VpcStack } from "./vpc";

export interface ElbStackProps extends cdk.StackProps {
  readonly vpcStack: VpcStack;
}

/**
 * Define ALB resources for generic use in applications.
 */
export class ElbStack extends cdk.Stack {
  /**
   * This is the ARN of the ALB for applications.
   */
  public readonly loadBalancer: elasticloadbalancingv2.IApplicationLoadBalancer;

  public constructor(scope: cdk.App, id: string, props: ElbStackProps) {
    super(scope, id, props);

    const vpc = props.vpcStack.vpc;

    // Resources
    const loadBalancer = new elasticloadbalancingv2.ApplicationLoadBalancer(
      this,
      "LoadBalancer",
      {
        internetFacing: true,
        vpc,
        vpcSubnets: {
          subnets: vpc.publicSubnets,
        },
      }
    );
    this.loadBalancer = loadBalancer;

    const hostedZone = route53.HostedZone.fromLookup(this, "HostedZone", {
      domainName: config.apiDomain,
    });

    new route53.ARecord(this, "RecordSet", {
      recordName: `${projectName}-${deployEnv}-api`,
      zone: hostedZone,
      target: route53.RecordTarget.fromAlias(
        new targets.LoadBalancerTarget(this.loadBalancer)
      ),
      ttl: cdk.Duration.minutes(5),
    });
  }
}
