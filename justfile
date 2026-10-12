# justfile for noip-client: build a static aarch64 binary for Termux on Android

target := "aarch64-unknown-linux-musl"
bin := "noip-client"
out := "target" / target / "release" / bin

# Phone connection (override with: just phone_host=100.x.x.x deploy)
phone_user := "u0_a253"
phone_host := "192.168.1.13"
phone_port := "8022"

# List available recipes
default:
    @just --list

# Run a command inside the Nix shell
[private]
nix cmd:
    nix-shell --run '{{cmd}}'

# Add the Rust target if it is missing
setup:
    just nix 'rustup target add {{target}}'

# Build the release binary for Termux
build: setup
    just nix 'cargo build --release --target {{target}}'

# Check the binary is an aarch64 static executable
verify: build
    file {{out}}
    @file {{out}} | grep -q 'ARM aarch64' || (echo "error: not an aarch64 binary" && exit 1)
    @file {{out}} | grep -q 'statically linked' || (echo "error: not statically linked" && exit 1)
    @echo "OK: {{out}}"

# Copy the binary and .env to the phone over SSH
deploy: verify
    scp -P {{phone_port}} {{out}} {{phone_user}}@{{phone_host}}:~/{{bin}}
    @test -f .env && scp -P {{phone_port}} .env {{phone_user}}@{{phone_host}}:~/.env || echo "warning: no .env to copy"

# Remove build artifacts for the Termux target
clean:
    rm -rf target/{{target}}

# Run cargo fmt and clippy on the host
lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
