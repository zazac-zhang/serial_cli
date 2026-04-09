#!/bin/bash
# Migrate println! to tracing::info! and eprintln! to tracing::error!
# This script performs bulk replacements for LOG-ONLY lines
#
# Strategy: Only replace lines that are clearly logging (messages like "Opening...", "Done", etc.)
# Keep println! for: JSON output, data output, interactive prompts, progress bars

set -e

cd "$(dirname "$0")/.."

echo "=== println! to tracing migration (log messages only) ==="
echo ""

# Files to process
FILES=(
    "src/main.rs"
    "src/cli/commands.rs"
    "src/cli/interactive.rs"
    "src/lua/bindings.rs"
    "src/utils.rs"
    "src/monitoring.rs"
    "src/task/monitor.rs"
    "src/serial_core/sniffer.rs"
)

echo "Files to process:"
for f in "${FILES[@]}"; do
    echo "  - $f"
done
echo ""

# Backup strategy
if git status --porcelain | grep -q .; then
    echo "Warning: Working directory has uncommitted changes."
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo "Starting migration..."
echo ""

# Pattern 1: eprintln! for errors/warnings -> tracing::error!/tracing::warn!
echo "[1/4] Replacing eprintln! with tracing macros..."
for f in "${FILES[@]}"; do
    if [[ -f "$f" ]]; then
        # Warning messages
        sed -i '' 's/eprintln!\("Warning:[^"]*"\)/tracing::warn!(\1/g' "$f" 2>/dev/null || true
        # Error messages
        sed -i '' 's/eprintln!\("Error:[^"]*"\)/tracing::error!(\1/g' "$f" 2>/dev/null || true
        # Generic eprintln with format
        sed -i '' 's/eprintln!\(/tracing::info!(/g' "$f" 2>/dev/null || true
        echo "  - Processed: $f"
    fi
done
echo ""

# Pattern 2: Logging println! -> tracing::info!
# These patterns match common log messages (starting with action words or ending with status)
echo "[2/4] Replacing log-style println! with tracing::info!..."
for f in "${FILES[@]}"; do
    if [[ -f "$f" ]]; then
        # Common log patterns (action messages)
        sed -i '' 's/println!\("Opening[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Closing[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Starting[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Stopping[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Running[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Executing[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Successfully[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Failed[^\"]*"\)/tracing::error!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Captured[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Saving[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Saved[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Loading[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Loaded[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Connecting[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Connected[^\"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        echo "  - Processed: $f"
    fi
done
echo ""

# Pattern 3: Status messages with specific prefixes
echo "[3/4] Replacing status println! with tracing::info!..."
for f in "${FILES[@]}"; do
    if [[ -f "$f" ]]; then
        # Port status messages
        sed -i '' 's/println!\("Port [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Sent [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Received [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("No [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Use [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        sed -i '' 's/println!\("Type [^"]*"\)/tracing::info!(\1/g' "$f" 2>/dev/null || true
        echo "  - Processed: $f"
    fi
done
echo ""

# Pattern 4: Remaining eprintln! catches
echo "[4/4] Final cleanup for remaining eprintln!..."
for f in "${FILES[@]}"; do
    if [[ -f "$f" ]]; then
        sed -i '' 's/eprintln!\(/tracing::error!(/g' "$f" 2>/dev/null || true
        echo "  - Processed: $f"
    fi
done
echo ""

echo "=== Migration complete ==="
echo ""
echo "Next steps:"
echo "1. Run 'git diff' to review changes"
echo "2. Manually verify JSON outputs still use println!"
echo "3. Manually verify interactive prompts (print!(\"serial> \")) are intact"
echo "4. Manually verify progress bars and hex dumps are intact"
echo "5. Run 'cargo check' to ensure no compilation errors"
echo "6. Update main.rs logging initialization if needed"
