;;; ==================================================
;;; STATE.scm — Misinformation Defence Platform
;;; ==================================================
;;;
;;; SPDX-License-Identifier: MIT
;;; Copyright (c) 2025 Jonathan D.A. Jewell
;;;
;;; STATEFUL CONTEXT TRACKING ENGINE
;;; Version: 2.0
;;;
;;; CRITICAL: Download this file at end of each session!
;;; At start of next conversation, upload it.
;;; Do NOT rely on ephemeral storage to persist.
;;;
;;; ==================================================

(define state
 '((metadata
   (format-version . "2.0")
   (schema-version . "2025-12-08")
   (created-at . "2025-12-08T00:00:00Z")
   (last-updated . "2025-12-08T00:00:00Z")
   (generator . "Claude/STATE-system"))

;;; ==================================================
;;; USER CONTEXT
;;; ==================================================

  (user
   (name . "Jonathan D.A. Jewell")
   (roles . ("founder" "architect" "developer"))
   (preferences
    (languages-preferred . ())  ;; To be determined
    (languages-avoid . ())
    (tools-preferred . ("GitHub" "GitHub Actions"))
    (values . ("truth" "transparency" "open-source" "security-first"))))

;;; ==================================================
;;; SESSION CONTEXT
;;; ==================================================

  (session
   (conversation-id . "create-state-scm-01DcNxCrs3x2o3qAHpjTW3GE")
   (started-at . "2025-12-08T00:00:00Z")
   (messages-used . 1)
   (messages-remaining . 99)
   (token-limit-reached . #f))

;;; ==================================================
;;; CURRENT FOCUS
;;; ==================================================

  (focus
   (current-project . "Misinformation-Defence-Platform")
   (current-phase . "pre-development-planning")
   (deadline . #f)
   (blocking-projects . ()))

;;; ==================================================
;;; CURRENT POSITION
;;; ==================================================
;;;
;;; The project is in PRE-DEVELOPMENT stage:
;;;
;;; What exists:
;;; - Repository infrastructure (LICENSE, CODE_OF_CONDUCT, SECURITY.md)
;;; - CI/CD scaffolding (GitHub Actions: CodeQL, Jekyll Pages)
;;; - Issue templates (bug report, feature request)
;;; - Dependabot configuration (incomplete)
;;;
;;; What does NOT exist yet:
;;; - Any source code
;;; - Architecture documentation
;;; - Tech stack selection
;;; - API design
;;; - Database schema
;;; - Frontend/UI design
;;;
;;; Overall Status: 0% implementation, 100% intention
;;;
;;; ==================================================

;;; ==================================================
;;; ROUTE TO MVP v1
;;; ==================================================
;;;
;;; MVP v1 Goal: A functional API that can analyze text/URLs
;;; for misinformation indicators and return a confidence score
;;; with explanatory factors.
;;;
;;; Phase 1: Foundation (Weeks 1-2)
;;; - [ ] Select tech stack (language, framework, database)
;;; - [ ] Define core API schema (OpenAPI spec)
;;; - [ ] Set up development environment
;;; - [ ] Create basic project structure
;;; - [ ] Configure CI/CD pipeline for chosen stack
;;;
;;; Phase 2: Core Detection Engine (Weeks 3-5)
;;; - [ ] Implement text preprocessing pipeline
;;; - [ ] Build claim extraction module
;;; - [ ] Create source credibility scoring system
;;; - [ ] Develop linguistic pattern analyzer
;;; - [ ] Integrate fact-checking API(s) if available
;;;
;;; Phase 3: API Layer (Week 6)
;;; - [ ] Implement REST API endpoints
;;; - [ ] Add rate limiting and authentication
;;; - [ ] Create response formatting
;;; - [ ] Write API documentation
;;;
;;; Phase 4: Testing & Hardening (Week 7)
;;; - [ ] Unit tests for all modules
;;; - [ ] Integration tests for API
;;; - [ ] Security audit
;;; - [ ] Performance benchmarking
;;;
;;; Phase 5: MVP Launch (Week 8)
;;; - [ ] Deploy to staging
;;; - [ ] Beta testing
;;; - [ ] Deploy to production
;;; - [ ] Public documentation
;;;
;;; ==================================================

;;; ==================================================
;;; KNOWN ISSUES / BLOCKERS
;;; ==================================================
;;;
;;; CRITICAL:
;;; 1. No tech stack selected - blocks all implementation
;;; 2. No architecture defined - blocks development planning
;;; 3. SECURITY.md is template-only - needs real policy
;;; 4. dependabot.yml has empty package-ecosystem
;;;
;;; HIGH:
;;; 5. No fact-checking data sources identified
;;; 6. No ML model strategy (build vs buy vs API)
;;; 7. No deployment target selected (cloud provider)
;;;
;;; MEDIUM:
;;; 8. README.md contains only title
;;; 9. No contribution guidelines (CONTRIBUTING.md)
;;; 10. No development setup documentation
;;;
;;; ==================================================

;;; ==================================================
;;; QUESTIONS FOR PROJECT OWNER
;;; ==================================================
;;;
;;; Architecture & Tech Stack:
;;; Q1: What programming language(s) do you prefer?
;;;     - Python (rich ML/NLP ecosystem)
;;;     - Rust (performance, safety)
;;;     - Go (simplicity, concurrency)
;;;     - TypeScript/Node (full-stack JS)
;;;     - Elixir (fault-tolerance, real-time)
;;;
;;; Q2: What is the target deployment environment?
;;;     - Self-hosted / On-premise
;;;     - AWS / GCP / Azure
;;;     - Vercel / Railway / Fly.io
;;;     - Docker-first / Kubernetes
;;;
;;; Q3: What database strategy?
;;;     - PostgreSQL (relational, JSONB support)
;;;     - MongoDB (document-oriented)
;;;     - SQLite (simplicity, embedded)
;;;     - Vector DB (Pinecone, Weaviate for embeddings)
;;;
;;; Detection Strategy:
;;; Q4: What is the primary detection approach?
;;;     - Rule-based heuristics (fast, explainable)
;;;     - ML classification (accuracy, training needed)
;;;     - LLM-based analysis (powerful, costly)
;;;     - Hybrid approach
;;;
;;; Q5: Will this integrate with external fact-checkers?
;;;     - Google Fact Check API
;;;     - ClaimBuster
;;;     - Full Fact API
;;;     - Custom database
;;;
;;; Scope & Users:
;;; Q6: Who is the primary user?
;;;     - End consumers (browser extension)
;;;     - Journalists / researchers
;;;     - Social media platforms (B2B API)
;;;     - Educational institutions
;;;
;;; Q7: What content types to support in MVP?
;;;     - Text only
;;;     - Text + URLs
;;;     - Images (OCR + reverse search)
;;;     - Video/audio transcripts
;;;
;;; Q8: What scale should MVP handle?
;;;     - Hobby (100 req/day)
;;;     - Small (10k req/day)
;;;     - Medium (100k req/day)
;;;     - Large (1M+ req/day)
;;;
;;; Open Source Strategy:
;;; Q9: Will the detection models/rules be open source?
;;;     - Fully open (model weights, rules, data)
;;;     - Partial (code open, models proprietary)
;;;     - API-only (closed source)
;;;
;;; Q10: Is there a monetization strategy?
;;;     - Fully free / donation-based
;;;     - Freemium (free tier + paid)
;;;     - Enterprise licensing
;;;     - Grant-funded
;;;
;;; ==================================================

;;; ==================================================
;;; PROJECT CATALOG
;;; ==================================================

  (projects
   ;; Main Platform
   ((name . "Misinformation-Defence-Platform")
    (status . "in-progress")
    (completion . 2)  ;; Only repo infrastructure exists
    (category . "ai")
    (phase . "pre-development")
    (dependencies . ())
    (blockers . ("tech-stack-selection"
                 "architecture-design"
                 "detection-strategy"))
    (next . ("Answer foundational questions"
             "Select tech stack"
             "Design system architecture"
             "Create API specification"))
    (chat-reference . #f)
    (notes . "Repository created with infrastructure only. No implementation yet."))

   ;; Sub-project: Detection Engine
   ((name . "Detection-Engine")
    (status . "blocked")
    (completion . 0)
    (category . "ai")
    (phase . "planning")
    (dependencies . ("tech-stack-selection"))
    (blockers . ("detection-strategy-undefined"))
    (next . ("Define detection approach"
             "Identify training data sources"
             "Design pipeline architecture"))
    (chat-reference . #f)
    (notes . "Core misinformation detection logic"))

   ;; Sub-project: API Layer
   ((name . "API-Layer")
    (status . "blocked")
    (completion . 0)
    (category . "infrastructure")
    (phase . "planning")
    (dependencies . ("Detection-Engine" "tech-stack-selection"))
    (blockers . ("no-api-spec"))
    (next . ("Create OpenAPI specification"
             "Define authentication strategy"
             "Design rate limiting"))
    (chat-reference . #f)
    (notes . "REST API for external consumers"))

   ;; Sub-project: Documentation Site
   ((name . "Documentation-Site")
    (status . "paused")
    (completion . 5)
    (category . "infrastructure")
    (phase . "setup")
    (dependencies . ())
    (blockers . ())
    (next . ("Configure Jekyll theme"
             "Write getting started guide"
             "Document API endpoints"))
    (chat-reference . #f)
    (notes . "Jekyll workflow exists but no content")))

;;; ==================================================
;;; LONG-TERM ROADMAP
;;; ==================================================
;;;
;;; MVP v1.0 - "Truth Detector"
;;; - Text analysis API
;;; - Basic credibility scoring
;;; - Simple web demo
;;;
;;; v1.1 - "Source Checker"
;;; - URL content extraction
;;; - Domain reputation database
;;; - Historical claim tracking
;;;
;;; v1.2 - "Pattern Recognizer"
;;; - Common misinformation pattern detection
;;; - Emotional manipulation indicators
;;; - Clickbait detection
;;;
;;; v2.0 - "Network Analyzer"
;;; - Viral spread tracking
;;; - Bot detection integration
;;; - Cross-platform correlation
;;;
;;; v2.1 - "Media Forensics"
;;; - Image manipulation detection
;;; - Reverse image search integration
;;; - Deepfake indicators (basic)
;;;
;;; v3.0 - "Intelligence Platform"
;;; - Real-time monitoring dashboards
;;; - Threat intelligence feeds
;;; - Campaign detection
;;; - API marketplace for integrations
;;;
;;; v3.1 - "Educational Tools"
;;; - Media literacy training modules
;;; - Interactive misinformation examples
;;; - School/university integrations
;;;
;;; v4.0 - "Federated Defense"
;;; - Decentralized fact-checking network
;;; - Cross-organization data sharing
;;; - Privacy-preserving analytics
;;;
;;; ==================================================

;;; ==================================================
;;; CRITICAL NEXT ACTIONS
;;; ==================================================

  (critical-next
   ("BLOCKER: Answer Q1-Q10 to unblock development"
    "Select programming language and framework"
    "Define MVP scope and detection strategy"
    "Create ARCHITECTURE.md with system design"
    "Write proper README.md with project vision"))

;;; ==================================================
;;; HISTORY
;;; ==================================================

  (history
   (snapshots
    ((timestamp . "2025-12-08T00:00:00Z")
     (event . "Initial STATE.scm creation")
     (projects
      (("Misinformation-Defence-Platform" . 2)
       ("Detection-Engine" . 0)
       ("API-Layer" . 0)
       ("Documentation-Site" . 5))))))

;;; ==================================================
;;; SESSION TRACKING
;;; ==================================================

  (files-created-this-session
   ("STATE.scm"))

  (files-modified-this-session
   ())

  (context-notes . "Initial project state capture. Repository is in pre-development with only infrastructure files. All implementation blocked on architectural decisions and tech stack selection.")))

;;; ==================================================
;;; END STATE.scm
;;; ==================================================
