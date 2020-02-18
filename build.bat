@echo off
rustup default nightly
xargo build --release --target x86_64-pc-windows-msvc
rustup default stable
