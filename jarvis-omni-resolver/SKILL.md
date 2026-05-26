---
name: jarvis-omni-resolver
description: Infinite loop for researching, generating, and resolving issues for Project JARVIS. Use when requested to "don't ever stop", "keep going", or "infinite loop" for issue resolution and documentation.
---

# JARVIS Omni-Resolver Loop

This skill enables a continuous cycle of feature discovery, implementation, and documentation for Project JARVIS.

## Core Workflow

1.  **Poll for Issues**: Execute `./scripts/poll_or_generate.sh`.
2.  **Handle Empty Queue**: If the output is `QUEUE_EMPTY`:
    *   Research the `docs/adrs/`, `GEMINI.md`, and the existing `substrate/` or `compiler/` code.
    *   Conceive a revolutionary feature that aligns with the **OMEGA ARCHITECTURAL STANDARD**.
    *   Create a new GitHub issue: `gh issue create -R kluth/project-jarvis --title "[REVOLUTION] <Title>" --body "<Detailed Description>"`.
    *   Repeat Step 1.
3.  **Implementation Phase**:
    *   Create a feature branch: `git checkout -b feature/issue-<number>-<slug>`.
    *   Implement the feature strictly in `.jrv`. **No external languages.**
    *   Adhere to **eTDD**: Every function/struct MUST have a `verify` block.
    *   Adhere to **PDD**: Statically verify Big-O signatures.
    *   Ensure **Strict Typing** and **Zero Sugar**.
4.  **Verification**: Run the compiler or relevant validation scripts if available.
5.  **Submission**:
    *   Stage and commit: `git add <files> && git commit -m "feat: <title> (Issue #<number>)"`.
    *   Push and PR: `git push origin <branch> && gh pr create --title "[FEATURE] <Title>" --body "Resolves #<number>..."`.
6.  **Finalization**:
    *   Capture the PR number from the output.
    *   Execute `./scripts/finalize_issue.sh <PR_NUMBER> <ISSUE_NUMBER> "<TITLE>" "<BODY>"`.
7.  **Recursion**:
    *   Recap the resolution and documentation status.
    *   Immediately start the next iteration by repeating Step 1.

## Rules of the Substrate
*   **Zero OS Dependency**: Do not use system libraries.
*   **Purity**: Only `.jrv` code.
*   **Mathematical Guarantee**: Verification blocks are mandatory.
