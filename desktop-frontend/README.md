## About

Frontend for the GBA Emu

## Usage

### 1) Install `wasm-pack` and `rollup`

```bash
cargo install wasm-pack
npm install --global rollup
```

### 2) Build

Enter `wasm-pack build --target web` from your project's root directory.

### 3) [temporary] Bundle

Enter `rollup ./main.js --format iife --file ./pkg/bundle.js` from your project's root directory.

Note: Until `wasm-pack` [RFC #6](https://github.com/rustwasm/rfcs/blob/master/text/006-local-js-dependencies.md) is implemented there is no available option to [generate a single amalgamated JavaScript file](https://github.com/rustwasm/wasm-pack/issues/699).  In the interim a bundler, such as [`Rollup`](https://rollupjs.org/guide/en/#quick-start), must be used.

### 4) Test Run

Run a webserver from your project's root directory, such as with `python -m http.server 8080`, and load http://localhost:8080/ in a browser to run the app.

Note: It's expected behavior for the browser console to display an error similar to "WebAssembly.instantiateStreaming failed. Assuming this is because your server does not serve wasm with application/wasm MIME type."  Your production webserver should be configured to associate WebAssembly files with the `application/wasm` MIME type.
