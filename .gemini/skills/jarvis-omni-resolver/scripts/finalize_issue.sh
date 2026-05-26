#!/bin/bash
PR_NUMBER=$1
ISSUE_NUMBER=$2
ISSUE_TITLE=$3
ISSUE_BODY=$4

if [ -z "$PR_NUMBER" ] || [ -z "$ISSUE_NUMBER" ]; then
  echo "Error: PR_NUMBER and ISSUE_NUMBER required."
  exit 1
fi

# Merge PR
gh pr merge "$PR_NUMBER" --merge --delete-branch

# Create doc issue on jarvis-os
DOC_BODY="The feature '$ISSUE_TITLE' has been merged. (Ref: kluth/project-jarvis#$ISSUE_NUMBER)

Original Issue Body:
$ISSUE_BODY

Documentation update required for JARVIS OS."

gh issue create -R kluth/jarvis-os --title "Documentation: $ISSUE_TITLE" --body "$DOC_BODY"
