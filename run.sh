./build.sh
cargo build
target/debug/node-template purge-chain --dev
target/debug/node-template --dev
