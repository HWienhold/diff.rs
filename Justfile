# List available targets
default:
    just --list

# Fetch data used in tests
test-data: test-data-info test-data-crates

# Fetch crate responses
test-data-info:
    curl -sSL https://crates.io/api/v1/crates/log -o data/log.json
    curl -sSL https://crates.io/api/v1/crates/serde -o data/serde.json
    curl -sSL https://crates.io/api/v1/crates/axum -o data/axum.json
    curl -sSL https://crates.io/api/v1/crates/reqwest -o data/reqwest.json

# Fetch crate sources
test-data-crates:
    curl -sSL https://crates.io/api/v1/crates/log/0.4.15/download -o data/log-0.4.15.crate
    curl -sSL https://crates.io/api/v1/crates/log/0.4.16/download -o data/log-0.4.16.crate
    curl -sSL https://crates.io/api/v1/crates/log/0.4.17/download -o data/log-0.4.17.crate

# Install dependencies needed to run
setup:
    rustup target add wasm32-unknown-unknown
    cargo install trunk

# Launch local debug server
serve:
    trunk serve
