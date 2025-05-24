"use client"

import { AwsRum, AwsRumConfig } from "aws-rum-web"

/**
 * CloudWatch RUM Web Client Provider
 * Initializes Real User Monitoring for the application
 */
const CloudWatchRumProvider = () => {
  if (typeof window === "undefined") {
    return <></>
  }

  try {
    // Environment variables from CDK configuration
    const appMonitorId = process.env.NEXT_PUBLIC_RUM_APP_MONITOR_ID
    const guestRoleArn = process.env.NEXT_PUBLIC_RUM_GUEST_ROLE_ARN
    const identityPoolId = process.env.NEXT_PUBLIC_RUM_IDENTITY_POOL_ID
    const region = process.env.NEXT_PUBLIC_RUM_REGION || "ap-northeast-1"

    // Skip initialization if required environment variables are missing
    if (!appMonitorId || !identityPoolId || !guestRoleArn) {
      console.warn("CloudWatch RUM: Missing required environment variables")
      return <></>
    }

    const config: AwsRumConfig = {
      sessionSampleRate: 1.0,
      identityPoolId: identityPoolId,
      endpoint: `https://dataplane.rum.${region}.amazonaws.com`,
      // guestRoleArn: guestRoleArn,
      telemetries: [
        "performance",
        "errors",
        [
          "http",
          {
            // https://github.com/aws-observability/aws-rum-web/blob/main/docs/configuration.md#http
            addXRayTraceIdHeader: true,
          },
        ],
      ],
      allowCookies: true,
      enableXRay: true,
      signing: true, // If you have a public resource policy and wish to send unsigned requests please set this to false
    }

    const APPLICATION_VERSION = "1.0.0"

    new AwsRum(appMonitorId, APPLICATION_VERSION, region, config)
  } catch (error) {
    // Ignore errors thrown during CloudWatch RUM web client initialization
    console.error("CloudWatch RUM initialization error:", error)
  }

  return <></>
}

export default CloudWatchRumProvider
