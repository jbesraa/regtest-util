# Utility package to setup Bitcoin Regtest locally with funded wallets

The package will download bitcoin core [docker
image](https://github.com/ruimarinho/docker-bitcoin-core). After the
image is installed and the container is running, two wallets will be
generated using
[bitcoincore-rpc](https://crates.io/crates/bitcoincore-rpc).  First
wallet is named  `sender` and its initialized with 5050 bitcoins and
the second wallet is named `receiver` and its initialized with 50
bitcoins.

* Rust & Docker are required.

## Install 

```shell
cargo install regtest-util
```

## Run

```shell
regtest-util
```


### License
MIT OR Apache-2.0"
