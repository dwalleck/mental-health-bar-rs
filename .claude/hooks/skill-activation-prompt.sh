#!/bin/bash
set -e

# Skill activation hook - calls Rust implementation
# For standalone installation (recommended):
#   Install once: ./install.sh
#   Binaries installed to ~/.claude-hooks/bin/

# Set CLAUDE_PROJECT_DIR if not already set
# This is required for the hook to find .claude/skills/skill-rules.json
if [ -z "$CLAUDE_PROJECT_DIR" ]; then
    # Get the directory containing this script, then go up two levels to project root
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    export CLAUDE_PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

# Check if standalone binary exists
if [ -f "$HOME/.claude-hooks/bin/skill-activation-prompt" ]; then
    cat | "$HOME/.claude-hooks/bin/skill-activation-prompt"
# Otherwise check for project-local build
elif [ -f "$CLAUDE_PROJECT_DIR/target/release/skill-activation-prompt" ]; then
    cat | "$CLAUDE_PROJECT_DIR/target/release/skill-activation-prompt"
else
    echo "❌ Rust hook not found!" >&2
    echo "Install with: ./install.sh" >&2
    echo "Or build locally: cargo build --release" >&2
    exit 1
fi
