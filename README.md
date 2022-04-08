# Hify

Hify is an audio player for Android TV. It is made of a server and a client, both running on a remote computer. The Android TV app then accesses the client through the provided API.

This project is under development.

**NOTE:** This application still lacks some features like play queue ordering and management
**NOTE:** This application is made for 4K TVs, it will not be rendered properly on 1080p TVs or computer screens

## Features

* Index building from files' metadata
* All common web formats supported (FLAC, MP3, M4A, OGG, ...)
* Lossless decoding
* Search tracks, albums and artists
* Support for huge collections thanks to lazy loading
* Support for Android TV's remote's inputs
* Support for keyboard navigation (mouse partially supported)
* Designed with performance and memory usage in mind, should work properly on low-end devices

Remaining features (todo list):

* Queue ordering
* Playing a track after the current one
* Playlist building
* History
* Automatic playlists

## Quickstart

The server works by analyzing the data in a given music directory, and produces an _index file_ which contains all required metadata. This file can be stored anywhere.

Clone this repository, then run the following in your terminal:

```shell
# Setup the server (once)
cd hify-server
cargo build --release

# Run the server
cargo run --release -- <path to your music directory> -i <path to the index file>

# Setup the client (once)
cd hify-client
pnpm install # Install dependencies
pnpm codegen # Generate API schema - requires the server to be running!

# Start the client
pnpm dev
```

By default the client is exposed on port `8892` and the API on port `8893`.

## A few tricks

* Long-pressing a direction button will make you go straight to the end / beginning of a page, list or grid
* Long-pressing the Play/Pause button will take you to the "Now Playing" screen or, if you're already there, to the "Search" screen
* Long-pressing the Rewind button will replay the current track or, if it's at its beginning, play the previous track instead
* Long-pressing the Forward button will play the next track

## Architecture

The server is made in Rust with Rocket and Async-GraphQL. It exposes both a GraphQL and a REST API. The former is for data fetching (e.g. list of albums, searching in the library, etc.) while the second delivers streams (e.g. pictures such as album artboxes, or audio track files).

The client is made in pure Svelte with an Apollo client to connect to the API. It uses a custom [navigation system](hify-client/src/navigable/) to handle Android TV's remote's inputs.

## License

This project is released under the [Apache-2.0](LICENSE.md) license terms.
