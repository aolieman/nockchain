# Nockchain

**Nockchain is a lightweight blockchain for heavyweight verifiable applications.**


We believe the future of blockchains is lightweight trustless settlement of heavyweight verifiable computation. The only way to get there is by replacing verifiability-via-public-replication with verifiability-via-private-proving. Proving happens off-chain; verification is on-chain.

*Nockchain is entirely experimental and many parts are unaudited. We make no representations or guarantees as to the behavior of this software.*

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/aolieman/nockchain)


## Setup

Install `rustup` by following their instructions at: [https://rustup.rs/](https://rustup.rs/)

Ensure you have these dependencies installed if running on Debian/Ubuntu:
```
sudo apt update
sudo apt install clang llvm-dev libclang-dev
```

Copy the example environment file and rename it to `.env`:
```
cp .env_example .env
```

Install `hoonc`, the Hoon compiler:

```
make install-hoonc
```

To build the Nockchain and the wallet binaries and their required assets:

```
make build
```

## Install Wallet

After you've run the setup and build commands, install the wallet:

```
make install-nockchain-wallet
```

See the nockchain-wallet [README](./crates/nockchain-wallet/README.md) for more information.


## Install Nockchain

After you've run the setup and build commands, install Nockchain:

```
make install-nockchain
```

## Setup Keys

To generate a new key pair:

```
nockchain-wallet keygen
```

This will print a new public/private key pair + chain code to the console, as well as the seed phrase for the private key.

Now, copy the public key to the `.env` file:

```
MINING_PUBKEY=<public-key>
```

## Backup Keys

To backup your keys, run:

```
nockchain-wallet export-keys
```

This will save your keys to a file called `keys.export` in the current directory.

They can be imported later with:

```
nockchain-wallet import-keys --input keys.export
```

## Running Nodes

To run a Nockchain miner:

```
make run-nockchain
```

=======
To run a Nockchain node without mining:

```
nockchain
```

To run a Nockchain node and mine to a pubkey:

```
nockchain --mining-pubkey <your_pubkey> --mine
```

For launch, make sure you run in a fresh working directory that does not include a .data.nockchain file from testing.


## FAQ

### Can I use same pubkey if running multiple miners?

Yes, you can use the same pubkey if running multiple miners.

### How do I change the mining pubkey?

Run `nockchain-wallet keygen` to generate a new key pair and copy the new public key to the `.env` file.

### How do I import an existing key that I generated with a different tool?

If you have a **base58 encoded** public key *AND* a **base58 encoded** chain code, you can import it with:

```
nockchain-wallet import-master-pubkey --key <base58-encoded-public-key> --chain-code <base58-encoded-chain-code>
```

But you really should use the `nockchain-wallet keygen` command to generate a new key pair and use that instead.
