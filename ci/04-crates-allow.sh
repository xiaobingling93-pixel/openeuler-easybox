#!/usr/bin/env bash
# Check if new dependencies are allowed
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ALLOW_FILE="$SCRIPT_DIR/crates.allow"

echo "==> Checking if new dependencies are allowed..."

# Get list of allowed crates from crates.allow file
if [[ ! -f "$ALLOW_FILE" ]]; then
    echo "Error: crates.allow file not found at $ALLOW_FILE" >&2
    exit 1
fi

ALLOWED_CRATES=$(grep -e "^-" "$ALLOW_FILE" | cut -d ':' -f 1 | cut -d '-' -f 2)

# Get new dependencies from Cargo.lock changes
# Handle case where origin/master doesn't exist
if ! git rev-parse --verify origin/master &>/dev/null; then
    echo "Warning: origin/master not found, skipping dependency check" >&2
    exit 0
fi

LOCK_FILES=()
readarray -t LOCK_FILES < <(git diff origin/master --name-only | grep '\.lock$')

# If no lock files changed, we're done
if [[ ${#LOCK_FILES[@]} -eq 0 ]]; then
    echo "==> No lock files changed."
    exit 0
fi

# Check if any lock file is newly created (not in origin/master)
for lock_file in "${LOCK_FILES[@]}"; do
    if ! git show origin/master:"$lock_file" &>/dev/null; then
        echo "==> New lock file detected: $lock_file, skipping dependency check."
        exit 0
    fi
done

DEPS=()
for lock_file in "${LOCK_FILES[@]}"; do
    while IFS= read -r dep; do
        DEPS+=("$dep")
    done < <(git diff origin/master "$lock_file" | grep -e "^+name =" | sed 's/^+name = "//' | sed 's/"$//')
done

# Check each dependency against allowed list
for dep in "${DEPS[@]}"; do
    if ! grep -q "^$dep$" <<< "$ALLOWED_CRATES"; then
        echo "Error: Disallowed crate found: $dep" >&2
        exit 1
    fi
done

echo "==> All dependencies are allowed."
