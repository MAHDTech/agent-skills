+++
title = "libraries-testing"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "opencode-acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Testing

> Check an agent implementation against the Agent Client Protocol

The [agentclientprotocol/acp-tck](https://github.com/agentclientprotocol/acp-tck)
repository provides a Test Compatibility Kit (TCK) for ACP agents. It launches
your agent as a stdio subprocess, drives it through the protocol (initialization,
session lifecycle, prompt turns, cancellation, error handling, and transport
hygiene), and reports which requirements pass, fail, don't apply, or were never
exercised.

\<Warning\>
  The TCK is experimental and does not cover the full protocol. Its
  requirements, options, and report format may change, and a passing run is not
  an official statement of ACP conformance.
</Warning>

Each requirement is reported with its spec citation and a tier: mandatory
behavior, behavior that applies only to capabilities the agent advertises,
recommended behavior, and informational probes.

See the [README](https://github.com/agentclientprotocol/acp-tck#readme) for
all options, the exact list of covered requirements, and how verdicts are
computed.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.
