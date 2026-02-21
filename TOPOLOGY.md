<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# Misinformation Defence Platform — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              DEFENCE ANALYST            │
                        │        (Detection HUD / API Client)     │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           DEFENCE PLATFORM              │
                        │                                         │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Disinfo   │  │ Defensive         │  │
                        │  │ Detector  │  │ Multiplicity      │  │
                        │  │ (Neural)  │  │ (Strategy)        │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │           ALGORITHM SHIELD              │
                        │    (Policy Enforcement & Filtering)     │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │          DIGITAL CHANNELS               │
                        │      (Social Media, News, APIs)         │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile Automation  .machine_readable/  │
                        │  Mustfile / Deno      0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
PLATFORM CORE
  Disinfo NeSy Detector             █░░░░░░░░░  10%    Architecture stubs
  Defensive Multiplicity            █░░░░░░░░░  10%    Strategy scaffolding
  Algorithm Shield                  █░░░░░░░░░  10%    Initial policy hooks

INFRASTRUCTURE
  CI/CD Scaffolding                 ██████████ 100%    Workflow templates active
  Language Policy (CCCP)            ██████████ 100%    RSR stack verified
  .machine_readable/                ██████████ 100%    STATE tracking active

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard build/lint tasks
  0-AI-MANIFEST.a2ml                ██████████ 100%    AI entry point verified
  Security Policy                   ██████████ 100%    Vulnerability reporting stable

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ██░░░░░░░░  ~20%   Specification Phase
```

## Key Dependencies

```
Platform Spec ───► Infrastructure ───► Detector Logic ───► Channel Audit
     │                 │                   │                 │
     ▼                 ▼                   ▼                 ▼
CCCP Policy ───► CI Workflows ─────► Algorithm Shield ──► Response
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
