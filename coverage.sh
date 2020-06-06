# Run cargo install grcov and install nightly (rustup install nightly) before running this script

cargo clean
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort"
cargo +nightly build
cargo +nightly test
grcov ./target/debug -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage

