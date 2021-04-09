# Run cargo install grcov and install nightly (rustup install nightly) before running this script

LD_LIBRARY_PATH=$(rustc --print sysroot)/lib

cargo clean
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
# on MacOS:
# cargo +nightly build --target x86_64-apple-darwin
# cargo +nightly test --target x86_64-apple-darwin
cargo +nightly build
cargo +nightly test
grcov ./target/debug -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage

