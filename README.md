# Hify

Hify is an audio player for Android TV. It is made of a server and a client, both running on a remote computer. The Android TV app then accesses the client through the provided API.

This project is under development.

**NOTE:** This application is made for 1080p-scaled screens, like 4K Android TVs (which use a 4x DPI scaling). It will not be rendered properly on other screen sizes (especially smaller ones).

## Features

* Index building from files' metadata
* All common web formats supported (FLAC, MP3, M4A, OGG, ...)
* Lossless decoding
* Search tracks, albums and artists
* Play queue management
* Tracks history
* Internal rating system with support for ID3, MusicBee and Windows Media Player rating tags
* Mix generator (can also generate for a specific artist or genre, and filter tracks by score)
* Support for album cover arts in JPEG and PNG format
* Support for huge collections (perfectly fluid with a 10k music files collection)
* Support for Android TV's remote's inputs
* Support for keyboard navigation (mouse partially supported)
* Designed with performance and memory usage in mind, should work properly on low-end devices

## Quickstart

The server works by analyzing the data in a given music directory, and produces an _index file_ which contains all required metadata. This file can be stored anywhere.

Clone this repository, then run the following in your terminal:

```bash
# Setup the server (once)
cd hify-server
cargo build --release

# Run the server
cargo run --release -- ./path-to-your-music-directory
```

For the client, run:

```bash
# Setup the client (once)
cd hify-client
pnpm i   # Install dependencies
pnpm gql # Generate API schema - requires the server to be running!

# Start the client (development)
pnpm dev

# ...or build for production
pnpm build
pnpm serve
```

By default the client is exposed on port `8892` and the API on port `8893`.

## A few tricks

* Long-pressing some items will display a context menu with additional options
* Long-pressing the Play/Pause button will take you to the "Now Playing" screen or, if you're already there, to the "Search" screen
* Long-pressing the Rewind button will replay the current track or, if it's at its beginning, play the previous track instead
* Long-pressing the Forward button will play the next track

## Architecture

The server is made in Rust with Rocket and Async-GraphQL. It exposes both a GraphQL and a REST API. The former is for data fetching (e.g. list of albums, searching in the library, etc.) while the second delivers streams (e.g. pictures such as album artboxes, or audio track files).

The client is made in pure Svelte with an Apollo client to connect to the API. It uses a custom [navigation system](hify-client/src/navigable/) to handle Android TV's remote's inputs.

## License

This project is released under the [Apache-2.0](LICENSE.md) license terms.
