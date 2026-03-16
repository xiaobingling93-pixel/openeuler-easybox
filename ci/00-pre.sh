#!/usr/bin/env bash
# Pre-installation script: Set up build environment
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=ci/common_function
source "$SCRIPT_DIR/common_function"

contains_chinese

# Find and set fastest Rust mirror for faster downloads
echo "==> Finding fastest Rust mirror..."
rust_mirrors=(
    "https://mirrors.ustc.edu.cn/rust-static"
    # "https://mirrors.tuna.tsinghua.edu.cn/rustup"
    # "https://mirrors.sjtug.sjtu.edu.cn/rustup"
    "https://rsproxy.cn"
)

fastest_rust_mirror=$(test_fasturl "${rust_mirrors[@]}")
if [[ -n "$fastest_rust_mirror" ]]; then
    export RUSTUP_DIST_SERVER="$fastest_rust_mirror"
    export RUSTUP_UPDATE_ROOT="$fastest_rust_mirror/rustup"
    echo "==> Using fastest Rust mirror: $fastest_rust_mirror"
else
    # Fallback to USTC mirror
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    echo "==> Using default Rust mirror: $RUSTUP_DIST_SERVER"
fi

# Install required packages using common function
required_packages=("gcc" "openssl-libs" "python3-pip" "python3" "python3-devel" "musl-gcc" "clang" "glibc-static" "libgcc" "pam-devel" "file-devel" "util-linux-devel")
install_packages "${required_packages[@]}"

# Check if libclang.so exists, create symlink if needed
if [ ! -e "/usr/lib64/libclang.so" ]; then
    sofile=$(ls /usr/lib64/libclang.so* | head -1)
    sudo ln -s "$sofile" /usr/lib64/libclang.so
fi

# Check if cargo is installed
if ! command -v cargo &>/dev/null; then
    echo "==> Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustlang.sh
    sh rustlang.sh -y --default-toolchain none
    rm -rf rustlang.sh
fi

# Source cargo environment
# shellcheck source=/dev/null
source "$HOME/.cargo/env"

# Set default Rust toolchain
if ! rustup show | grep -q "1.76.0"; then
    echo "==> Installing Rust $rust_vendor..."
    rustup install "$rust_vendor"
fi
rustup default "$rust_vendor"

# Add musl target
arch=$(uname -m)
echo "==> Adding musl target for $arch..."
rustup target add "$arch-unknown-linux-musl"

# Find and set fastest cargo registry mirror with fallback
echo "==> Finding available cargo registry mirrors..."
crate_names=(
    "https://mirrors.ustc.edu.cn/crates.io-index"
    # "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
    # "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
    "https://rsproxy.cn/crates.io-index"
)

# Get available mirrors
available_mirrors=()
while IFS= read -r mirror; do
    [[ -n "$mirror" ]] && available_mirrors+=("$mirror")
done < <(get_available_cargo_mirrors "${crate_names[@]}")

if [[ ${#available_mirrors[@]} -eq 0 ]]; then
    echo "Error: No available cargo registry mirrors found!" >&2
    echo "Please check your network connection and try again." >&2
    exit 1
fi

# Use the first available mirror as primary
fastest_source="${available_mirrors[0]}"
echo "==> Primary cargo source: $fastest_source"

# Configure cargo to use available mirrors with fallback
mkdir -p ~/.cargo
cat << EOF > ~/.cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

[source.ustc]
registry = "$fastest_source"

[target.$arch-unknown-linux-musl]
rustflags = ["-C", "target-feature=-crt-static"]

[net]
git-fetch-with-cli = true
retry = 10

EOF

echo "==> Cargo configuration:"
cat ~/.cargo/config

rm -rf ~/.cargo/.package-cache

# Find fastest GitHub mirror
sources=("https://521github.com/" "https://gitclone.com/github.com/" "https://gh.api.99988866.xyz/https://github.com/" "https://github.com/")
url=$(test_fasturl "${sources[@]}")
git config --global url."${url}".insteadOf "https://github.com/"
echo "==> Using GitHub mirror: $url"

# Find fastest PyPI mirror
pipurls=("http://mirrors.aliyun.com/pypi/simple/" "https://pypi.mirrors.ustc.edu.cn/simple/" "http://pypi.sdutlinux.org/" "http://pypi.douban.com/simple/")
url=$(test_fasturl "${pipurls[@]}")

if [[ $url =~ ^https?://([^/]+) ]]; then
    domain="${BASH_REMATCH[1]}"
    pip config set global.index-url "$url"
    pip config set global.trusted-host "$domain"
    echo "==> Using PyPI mirror: $url"
fi

# Install pre-commit for local development
echo "==> Installing pre-commit and codespell..."
pip3 install --user pre-commit codespell
export PATH="$HOME/.local/bin:$PATH"

# Initialize pre-commit
echo "==> Initializing pre-commit hooks..."
git config --global init.templateDir ~/.git-template
pre-commit init-templatedir ~/.git-template
pre-commit install

echo "==> Pre-installation completed successfully."
