+++
title = "developers-grpc-api-referencebatch-management"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

# gRPC API Reference

The xAI gRPC API is a robust, high-performance gRPC interface designed for seamless integration into existing systems.

The base url for all services is at `api.x.ai`. For all services, you have to authenticate with the header `Authorization: Bearer <your xAI API key>`.

Visit [xAI API Protobuf Definitions](https://github.com/xai-org/xai-proto) to view and download our protobuf definitions.

The [xAI Python SDK](https://github.com/xai-org/xai-sdk-python) (`xai-sdk`) uses gRPC natively. Install with `pip install xai-sdk`.

## Using buf curl

Clone the proto definitions and use [buf curl](https://buf.build/docs/curl/usage) to call the API:

```bash
git clone https://github.com/xai-org/xai-proto.git
cd xai-proto
```

All `buf curl` examples below assume you run from inside the cloned `xai-proto` directory.

***
