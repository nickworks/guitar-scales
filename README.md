## Dependency: Trunk

This project uses [trunk](https://trunkrs.dev/) to build and bundle for WASM delivery.

### Installing Trunk

`cargo install --locked trunk`

On Apple M1, you may also need wasm-bindgen-cli:

`cargo install --locked wasm-bindgen-cli`

### Building the Project

1. `trunk build`
2. Serve `dist` folder via web-server
3. Open in browser
