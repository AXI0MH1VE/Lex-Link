#!/bin/bash
# AXIOM HIVE - GitHub Publishing Script
set -e
echo "[AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]"
echo "Publishing to GitHub..."
git init
git config user.name "Alexis Adams"
git config user.email "devdollzai@gmail.com"
git add .
git commit -m "🚀 AXIOM HIVE / LEX-Ω v1.0.0 [SUBSTRATE: ALEXIS ADAMS]"
if command -v gh &> /dev/null; then
    gh repo create LexLink --public --source=. --remote=origin --push --description "AXIOM HIVE / LEX-Ω - Coding-only, human-in-the-loop. C=0"
    echo "✅ Published to: https://github.com/AXI0MH1VE/LexLink"
else
    echo "⚠️  Install gh: brew install gh && gh auth login"
fi
