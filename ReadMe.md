## This is a simple contract written in Rust and deloyed to the Soroban local network

The soroban-sdk provides a variety of types like Vec, Map, Bytes, BytesN, Symbol, that all utilize the Soroban environment's memory and native capabilities.

### Get started 

To run this project you need to have installed the Rust CLI and then setup Soroban enviroment

#### Install Rust
for mac
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
if you use windows setup from [here](https://www.rust-lang.org/tools/install)

#### Install wasm-32
```shell
rustup target add wasm32-unknown-unknown

```

#### Install Soroban CLI 

```shell
cargo install --locked --version 0.4.0 soroban-cli
```

Run this commmand to confirm that all necessary tools and packages have been setup

```shell
soroban
```


#### Install Nightly

To install the nightly Rust toolchain use rustup.

- Run these command
```shell
rustup install nightly
```

```shell
rustup target add --toolchain nightly wasm32-unknown-unknown
```

```shell
rustup component add --toolchain nightly rust-src
```


#### Install wasm-opt

To install wasm-opt, install binaryen. 
Depending on your operating system there are different ways to install it.

for Mac
```shell
brew install binaryen
```
For Linux
```shell
sudo apt install binaryen
```
For windows install options see: https://github.com/WebAssembly/binaryen/releases



#### Deploying to Soroban

- Before we can deploy, we first need to build our contract
- To build, run

```shell
cargo build --target wasm32-unknown-unknown --release
```
Once built a wasm file will be generated in your target/wasm32-unknown-unknown/release/[your project name].wasm
it contains the contract logic as well as implementations for you to be able to call it from another contract,
This is all you need. This is the only artifact needed to deploy the contract.

- Next
- If you have the soroban-cli installed, you can run contracts in a local sandbox environment.

```shell
soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/[project-name].wasm \
    --id 1 \
    --fn hello \
    --arg Orokii
```
it should return 

```shell
["Hello","Orokii"]
```
This is because our return type is a vec.
You can change the Orokii to whatever you like to see how the hello function returns differently.

#### What's more ---
  - Building optimized contracts is necessary when deploying to a network with fees or when analyzing and profiling a contract to get it as small as possible.

So let's optimize our contract to save gas and help it deploy better, Soroban only accepts contract that are withing the max size of 256KB.

- Run these command
```shell
cargo +nightly build \
    --target wasm32-unknown-unknown \
    --release \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort
```

- Use Opt-wasm to further minimize

```shell
wasm-opt -Oz \
    target/wasm32-unknown-unknown/release/[your project name].wasm \
    -o target/wasm32-unknown-unknown/release/first_project_optimized.wasm
```

###### Congratulations, you just built and deployed your first Soroban smart contract ⚡🎯🎊
