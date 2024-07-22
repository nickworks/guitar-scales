# Fretboard Scales App

An app for visually exploring various scales laid out on a fretboard.

## Native build

```
cargo run
```

For a release build:

```
cargo build --release
```

## WASM build with Trunk

This project uses [trunk](https://trunkrs.dev/) to build and bundle for WASM delivery.

### Installing Trunk

```
cargo install --locked trunk
```

On Apple M1, you may also need wasm-bindgen-cli:

```
cargo install --locked wasm-bindgen-cli
```

On Mac OS X, I also had to add `~/.cargo/bin` to my PATH environment variable for trunk to work properly. Run this or add to `~/.zshrc`:

```
export PATH="$HOME/.cargo/bin:$PATH"
```

### Running Locally

```
trunk serve
```

### Building the Project

The following should build a WASM release in the `./dist` folder.

```
trunk build --release --public-url .
```
