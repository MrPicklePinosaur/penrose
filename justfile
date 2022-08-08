
default: build

build:
    cargo build --release

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

chk:
    cargo check

lint:
    cargo clippy
