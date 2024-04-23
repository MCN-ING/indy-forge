[![img](https://img.shields.io/badge/Cycle%20de%20Vie-Phase%20d%C3%A9couverte-339999)]

## About

A graphical interface for various utilities that can be useful for Indy Ledger
The app uses [egui](https://www.egui.rs/) for the GUI, as such you can run it natively on Windows, Linux and MacOS or 
for the web, and share it using Github Pages. See issue #1

## Endorser Tool

Tool for endorsing(signing) an Indy Node transactions

The [endorser-tool](https://github.com/andrewwhitehead/endorser-tool) from Andrew Whitehead was the inspiration for this
project.

## NYM Creation Tool

Tool for registering a new NYM and it's role on the Indy Ledger

## Getting started

Make sure you are using the latest version of stable rust by running `rustup update`.
Run `cargo run` to start the app.

### Native Locally

`cargo run --release`
The executable will be in `target/release/indorser`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

Sadly, for the moment, the zmq library is preventing a webassembly build. 
