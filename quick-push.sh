#!/bin/bash
# Auto-commit and push script for Rust learning project

echo "Adding all changes to Git..."
git add .

echo "Enter commit message (or press Enter for default):"
read -r commit_message

if [ -z "$commit_message" ]; then
    commit_message="Update: $(date '+%Y-%m-%d %H:%M:%S')"
fi

echo "Committing changes..."
git commit -m "$commit_message"

echo "Pushing to GitHub..."
git push origin main

echo "✅ Changes successfully pushed to GitHub!"
