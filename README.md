# Moon Engine
A Game library written with Rust, compiled to WebAssembly and rendered with WebGL2. Intended to be used as my final year project.

[![Crates.io](https://img.shields.io/crates/v/moon-engine)](https://crates.io/crates/moon-engine)
[![docs.rs](https://img.shields.io/docsrs/moon-engine)](https://docs.rs/moon-engine/latest/moon_engine/)

Requires node.js and npm to be installed. 
If you do not already have these installed, a manager like [nvm](https://github.com/nvm-sh/nvm/blob/master/README.md) is **HIGHLY** recommended.

## Run Demo

Download the latest package from this repo.

Install light-server using: 

```
npm install -g light-server
```

In the root directory of the downloaded folder, run 

```
light-server --serve .
```

### Controls

|Key|Control|
|:--:|--|
|`W`|Move Up|
|`A`|Move Left|
|`S`|Move Down|
|`D`|Move Right|
|`1`|Toggle `FIRE`|
|`2`|Toggle `DEFAULT`|
|`3`|Toggle `SMOKE`|

#### Notes:

- The `WASD` controls are used to move the `SMOKE` Particle System.

- The mouse `X` and `Y` are used to change the position of the `DEFAULT` Particle System.

## Instructions
### Install Rust and wasm-pack
Install Rust using [rustup](https://www.rust-lang.org/tools/install)

Check that Rust installed successfully using: 

```
rustc --version
```

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

### Ensuring npm is up-to-date
Make sure your version of npm is up-to-date with:

```
 npm install npm@latest -g
```

### Clone this repository
Either use a client/download this repo as a zip or clone using:

```
git clone https://github.com/polarvoid/moon-engine.git
```

### Final setup
Switch to the moon-engine/moon directory.

```
cd moon-engine/moon
```

Build the wasm file. The initial run might take a while. Subsequent builds should be faster.

<details open>
<summary>Note about <kbd>wasm-pack</kbd></summary>
<p>
Make sure you've clicked on the link mentioned above and have <kbd>wasm-pack</kbd> installed and ready to use.

To check whether <kbd>wasm-pack</kbd> is installed, run:

```
wasm-pack --version
```
</p>
</details>

To build:

```
wasm-pack build
```

On a non-Windows system, you will have to change the slashes used in the include_str!() macro from \\ to /. Otherwise, rustc might throw an error at you.

Switch to the www directory and install the node packages.

```
cd www
npm install
```

To run a live development server, use the command:

```
npm run start
```

To build the application, use:

```
npm run build
``` 
The files will be stored in a folder called dist/ within the www directory.


## Credits

Icons made by Icongeek26 from [www.flaticon.com](https://www.flaticon.com).
