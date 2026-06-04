<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# NSAI Disinfo Detector Architecture

## Components
- **Go Orchestrator**: NATS JetStream consumer, ONNX/Souffle pipeline.
- **Dgraph**: Knowledge graph for symbolic facts.
- **NATSS**: Message broker for scalability.
- **Podman**: Container runtime (immutable, rootless).
- **SaltStack**: Configuration management and rollback.

## Diagrams
- [System Architecture](diagrams/system.excalidraw)
- [Data Flow](diagrams/data_flow.mermaid)

## Policies
- [Security Policy](policies/security.md)
- [Rollback Procedure](policies/rollback.md)
