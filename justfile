
default: build

build:
    cargo build --release

install:
    cargo build --release
    sudo mkdir -p /usr/local/bin
    sudo cp -f target/release/pinowm /usr/local/bin/pinowm
    sudo chmod 755 /usr/local/bin/pinowm
    sudo cp -f pinowm.desktop /usr/share/xsessions/pinowm.desktop

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

chk:
    cargo check

lint:
    cargo clippy
