# GitHub Inspired-by Copy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Present Body Motion as a standalone Horizon mod inspired by Not Enough Animations, never as an NEA port.

**Architecture:** Edit existing public copy and legal provenance text, then update GitHub repository description and v0.2.81 release notes through `gh api`. No code or binary changes.

**Tech Stack:** Markdown, plain text, GitHub CLI/API.

**Spec:** `docs/superpowers/specs/2026-11-15-github-not-port-design.md`

## Global Constraints

- Write all public GitHub-facing text in English.
- Preserve NEA inspiration credit and existing MIT-style license terms.
- Do not claim legal conclusions about derivative status.
- Do not change code, binaries, version, tag, or release assets.

## Review Focus

- README and metadata must say inspired-by, not port; verify with text search and GitHub API.
- Russian README block conflicts with English-only GitHub text; remove it.
- LICENSE and NOTICE must retain attribution and all license terms; inspect full diff.
- Release edit must preserve asset and tag; query release metadata after PATCH.

---

### Task 1: Update local public copy

**Files:**
- Modify: `README.md`
- Modify: `LICENSE`
- Modify: `NOTICE`

**Interfaces:**
- Consumes: approved spec.
- Produces: English standalone/inspired-by wording, preserving provenance and license terms.

- [ ] **Step 1: Rewrite README positioning**

Change the overview to describe Body Motion as an independently developed native Horizon mod inspired by NEA. Credit NEA as inspiration. State original implementation authorship only as supported by repository evidence. Remove Russian section.

- [ ] **Step 2: Rewrite provenance wording**

Update `LICENSE` and `NOTICE` from “port”/“based on” implications to independent project inspired by NEA. Preserve all license grant and disclaimer lines unchanged.

- [ ] **Step 3: Verify local copy**

Run `git diff --check` and inspect `git diff -- README.md LICENSE NOTICE`. Confirm no public text calls Body Motion a port and no license terms changed.

### Task 2: Update GitHub metadata and release notes

**Files:**
- Remote: `Dbdbdndbbsbs/BodyMotion` repository description.
- Remote: release `v0.2.81` body.

**Interfaces:**
- Consumes: verified local copy.
- Produces: matching English GitHub description and release notes.

- [ ] **Step 1: Update description**

Use GitHub API to set description to `Body Motion — standalone full-body player animations for Horizon, independently developed and inspired by Not Enough Animations.`

- [ ] **Step 2: Update release body**

Retain v0.2.81 feature notes; add a concise project positioning statement saying independently developed and inspired by NEA, not a port. Do not touch assets or tag.

- [ ] **Step 3: Verify GitHub API**

Query repository and release endpoints. Confirm description and body match intended English text, tag stays `v0.2.81`, asset remains attached, and release stays published.

### Task 3: Commit and publish source copy

**Files:**
- Commit: `README.md`, `LICENSE`, `NOTICE`.

- [ ] **Step 1: Commit**

Commit only the three edited text files and approved design/plan docs if appropriate, with a concise message.

- [ ] **Step 2: Push**

Push to `origin main`; verify clean status and remote commit. Do not alter existing tag.
