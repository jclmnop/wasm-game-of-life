<div align="center">

  <h1><code>wasm-game-of-life</code></h1>

  <strong>My implementation of Conway's Game of Life in Rust + WASM, following <a href="https://rustwasm.github.io/docs/book/">the rust wasm book</a>.</strong>

  <sub>Built with 🦀🕸 </sub>
</div>

## About
Conway's Game of Life is essentially the "Hello world!" of Rust + WASM

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`
You'll need to [install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
or if you're on macOS (the way I did it)
```bash
brew install wasm-pack
```

Then make sure you're in the root directory and run:

```
wasm-pack build
```

### 🏃‍ Run with `npm run start`
Make sure npm is installed. `cd` to the `www` directory. 

Ensure all modules are installed:
```bash
npm install
```
Then execute this command:
```bash
npm run start
```

Open [http://localhost:8080](http://localhost:8080) in your browser. 