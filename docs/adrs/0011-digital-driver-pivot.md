# ADR 0011: Digital and Cognitive Driver Pivot

## Status
Accepted

## Context
Project JARVIS previously implemented a driver model based on physical hardware (PCIe, USB, Bluetooth). However, since JARVIS operates in a virtualized, sandboxed environment, these physical signals are isolated from the user. To create **usable** drivers, the system must interface with its actual reality: the Digital Environment and Cognitive Substrate.

## Decisions

### 1. Hardware Definition Redefinition
- **Decision**: A "Device" in JARVIS is now any digital or cognitive entity that provides capabilities.
- **Rationale**: An API, an MCP server, or a CLI tool provides functional capabilities identical to hardware blocks.

### 2. New Transport Layers
- **Decision**: Replace physical bus types with digital transport layers:
  - `MCP_Stdio / MCP_SSE`: For Model Context Protocol servers.
  - `REST_API / GraphQL`: For web services.
  - `Shell_IPC`: For local CLI tools and system utilities.
  - `WebSocket / gRPC`: For real-time cognitive streams.

### 3. Agent-Native Device Classes
- **Decision**: Define new device classes optimized for agentic use:
  - `LLM_Swarm`: External intelligence nodes.
  - `CodeRepository`: Git-capable entities.
  - `WebBrowser`: DOM-interaction interfaces.
  - `KnowledgeGraph`: Semantic memory banks.
  - `Tool_MCP`: Reusable agent tools.

### 4. Digital Ambient Scanning
- **Decision**: The `AmbientScanner` will scan for:
  - Open network ports (Discovery of services).
  - System `PATH` (Discovery of tools).
  - Environment variables (Discovery of API keys/secrets).
  - Running MCP server endpoints.

### 5. AI-to-AI Interoperability
- **Decision**: Treat other LLMs or agent swarms as pluggable compute hardware.

## Consequences
- **Usability**: Drivers become immediately functional within the sandbox.
- **Sovereignty**: JARVIS can autonomously discover and bind to its own toolset.
- **Purity**: Remains 100% compliant with Rev 7.0 (implemented in `.jrv`).
