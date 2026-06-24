<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `misinformation-defence-platform` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: add stapeln.toml container definition
- feat: deploy UX Manifesto infrastructure
- feat: add CLADE.a2ml — clade taxonomy declaration
- feat: adopt contractile system
- feat: add AI Gatekeeper Protocol manifest

### Fixed

- fix(ci): pin upload-artifact to valid SHA in hypatia-scan.yml (Refs standards#48) (#30)
- fix(ci): bump a2ml/k9-validate-action pins to canonical (#29)
- fix(ci): sync hypatia-scan.yml to canonical (#28)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix(ci): adopt canonical hypatia-scan.yml (#27)
- fix(ci): Phase-2 fleet submission must not fail the security gate (#26)
- fix(ci): hypatia-scan workdir (${{ env.HOME }} resolves empty) (#25)
- fix(ci): bump erlef/setup-beam SHA for ubuntu24 runner support (#23)
- fix(deps): force-bump vulnerable transitive crates via [patch.crates-io] (#21)
- fix(security): update dependencies to fix vulnerabilities

### Changed

- refactor: migrate 6SCM → 6A2 (.scm → .a2ml format)

### Documentation

- docs(governance): CRG v2.0 STRICT audit — C (declared) -> D (honest)
- docs: substantive CRG C annotation (EXPLAINME.adoc)
- docs: add CRG Grade C header to TEST-NEEDS.md
- docs: add EXPLAINME.adoc — prove-it file backing README claims

### CI

- ci: redistribute concurrency-cancel guard to read-only check workflows (#32)
- ci: bump actions/upload-artifact SHA to current v4 (#22)
- ci: SHA-pin hyperpolymath validate-actions in dogfood-gate
- ci: deploy dogfood-gate, fix hypatia-scan, add pre-commit hooks

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
