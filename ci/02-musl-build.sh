#!/usr/bin/env bash
# Optional script: Build easyBox with musl target
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=ci/common_function
source "$SCRIPT_DIR/common_function"

echo "==> (Optional) Building easyBox with musl..."

# Install musl-gcc if not already installed
install_packages "musl-gcc"

# Get system architecture
arch=$(uname -m)

# Add musl target to Rust
echo "==> Adding musl target for $arch..."
rustup target add "$arch-unknown-linux-musl"

# Build with musl target (exclude login which requires PAM dynamic library)
echo "==> Building with musl target..."
export RUSTFLAGS="-C link-arg=-lm"
# Exclude 'login' and 'file' features as they require glibc libraries (PAM, libmagic) not compatible with musl static linking
# Note: Do not use --all flag as it would build all workspace members including oe_login
musl_features="base32,chage,grep,flock,hwclock,pidof,pstree,taskset,lspci,xargs,attr,free,usleep,which,column,setsid,sha256sum,killall,md5sum,sysctl,iostat,pgrep,groupadd,usermod,useradd,find,mount,umount,arp,less,logger"
cargo build --no-default-features --features "$musl_features" --target="$arch-unknown-linux-musl"

echo "==> musl build completed successfully."
