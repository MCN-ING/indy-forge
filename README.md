[![img](https://img.shields.io/badge/Cycle%20de%20Vie-Phase%20d%C3%A9couverte-339999)]

# Indy Forge

[![img](https://img.shields.io/badge/Version-0.4.0-blue)](

## About

A graphical interface for various utilities that can be useful for Indy Ledger
The app uses [egui](https://www.egui.rs/) for the GUI, as such you can run it natively on Windows, Linux and MacOS ~~or
for the web, and share it using Github Pages.~~ See [issue #1](https://github.com/MCN-ING/indy-forge/issues/1)

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
The executable will be in `target/release/indyforge`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

Sadly, for the moment, the zmq library is preventing a webassembly build.

## À propos

Une interface graphique pour divers utilitaires qui peuvent être utiles pour Indy Ledger.
L'application utilise [egui](https://www.egui.rs/) pour l'interface graphique, vous pouvez donc l'exécuter nativement
sur Windows, Linux et MacOS ~~ou
pour le web, et le partager en utilisant Github Pages.~~
Voir [problème #1](https://github.com/MCN-ING/indy-forge/issues/1).

## Outil de signature

Outil pour signer des transactions de nœud Indy.

L'[outil de](https://github.com/andrewwhitehead/endorser-tool) d'Andrew Whitehead a inspiré ce
projet.

## Outil de création de NYM

Outil pour enregistrer un nouveau NYM et son rôle sur Indy Ledger.

## Pour commencer

Assurez-vous d'utiliser la dernière version de Rust stable en exécutant `rustup update`.
Exécutez `cargo run` pour démarrer l'application.

### Natif Localement

`cargo run --release`
L'exécutable se trouvera dans `target/release/indyforge`.

Sur Linux, vous devez d'abord exécuter :

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

Sur Fedora Rawhide, vous devez exécuter :

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Localement

Malheureusement, pour le moment, la bibliothèque zmq empêche une compilation en WebAssembly.
