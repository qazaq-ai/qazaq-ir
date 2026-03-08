#!/bin/bash
set -e
echo "🚀 Starting release process for Qazaq IR..."
VERSION=$(grep -m 1 '^version = ' Cargo.toml | awk -F '"' '{print $2}')
echo "📦 Version found: v$VERSION"
git add .
git commit -m "chore: release v$VERSION" || true
git push origin main
echo "🏷️ Creating and pushing git tag v$VERSION..."
git tag "v$VERSION"
git push origin "v$VERSION"
echo "✅ Release v$VERSION pushed successfully! GitHub will now build the release."