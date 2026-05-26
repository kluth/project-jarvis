#!/bin/bash
ISSUES=$(gh issue list -R kluth/project-jarvis --state open --limit 1 --json number,title,body)
if [ "$ISSUES" == "[]" ] || [ -z "$ISSUES" ]; then
  echo "QUEUE_EMPTY"
else
  echo "$ISSUES" | jq -c '.[0]'
fi
