# ASIMOV Solana Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-solana-module.svg)](https://crates.io/crates/asimov-solana-module)

[ASIMOV] module for data import from the [Solana] blockchain network.

## ✨ Features

- Imports structured data from [Solana] blocks and transactions.
- Supports [mainnet], [devnet], and [testnet] environments.
- Collects raw JSON data via the official [Solana JSON-RPC API].
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output in the form of [JSON-LD].
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-solana-module
```

## 👉 Examples

### Fetching a Devnet Block

```bash
asimov-solana-fetcher solana://devnet/100000000
```

### Fetching a Mainnet Block

```bash
asimov-solana-fetcher solana://mainnet/100000000
```

### Fetching a Testnet Block

```bash
asimov-solana-fetcher solana://testnet/100000000
```

## 📚 Reference

### Installed Binaries

- `asimov-solana-fetcher`: collects JSON data from the Solana JSON-RPC API

### `asimov-solana-fetcher`

```
Fetches block data from the Solana blockchain via the official JSON-RPC API.

Usage: asimov-solana-fetcher [OPTIONS] [URLS]...

Arguments:
  [URLS]...  One or more `solana://<network>/<slot>` URLs

Options:
  -o, --output <FORMAT>  The output format: json (default) or jsonl
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-solana-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-solana-module&text=asimov-solana-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-solana-module&title=asimov-solana-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-solana-module&t=asimov-solana-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-solana-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-solana-module)

[ASIMOV]: https://github.com/asimov-platform
[JSON-LD]: https://json-ld.org
[KNOW]: https://github.com/know-ontology
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[Rust]: https://rust-lang.org
[Solana]: https://solana.com
[Solana JSON-RPC API]: https://solana.com/docs/rpc
[mainnet]: https://solana.com/docs/core/clusters#mainnet-beta
[devnet]: https://solana.com/docs/core/clusters#devnet
[testnet]: https://solana.com/docs/core/clusters#testnet
