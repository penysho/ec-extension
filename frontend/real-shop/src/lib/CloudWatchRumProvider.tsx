"use client"

import { AwsRum, AwsRumConfig } from "aws-rum-web"

const CloudWatchRumProvider = () => {
  try {
    const config: AwsRumConfig = {
      sessionSampleRate: 1,
      endpoint: "https://dataplane.rum.ap-northeast-1.amazonaws.com",
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

    const APPLICATION_ID: string = process.env.CLOUDWATCH_RUM_APPLICATION_ID!
    const APPLICATION_VERSION: string = process.env.CLOUDWATCH_RUM_VERSION!
    const APPLICATION_REGION: string = process.env.CLOUDWATCH_RUM_REGION!

    new AwsRum(APPLICATION_ID, APPLICATION_VERSION, APPLICATION_REGION, config)
  } catch (error) {
    // Ignore errors thrown during CloudWatch RUM web client initialization
    console.error(error)
  }

  return <></>
}

export default CloudWatchRumProvider
