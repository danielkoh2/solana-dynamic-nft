# solana-dynamic-nft

## 🛠 Getting Started

This repository is using [Surfpool](https://surfpool.run) as a part of its development workflow.

Surfpool provides three major upgrades to the Solana development experience:
- **Surfnet**: A local validator that runs on your machine, allowing you fork mainnet on the fly so that you always use the latest chain data when testing your programs.
- **Runbooks**: Bringing the devops best practice of `infrastructure as code` to Solana, Runbooks allow you to have secure, reproducible, and composable scripts for managing on-chain operations & deployments.
- **Surfpool Studio**: An all-local Web UI that gives new levels of introspection into your transactions.

### Installation

Install pre-built binaries:

```console
# macOS (Homebrew)
brew install txtx/taps/surfpool

# Updating surfpool for Homebrew users
brew tap txtx/taps
brew reinstall surfpool

# Linux (Snap Store)
snap install surfpool
```

Install from source:

```console
# Clone repo
git clone https://github.com/txtx/surfpool.git

# Set repo as current directory
cd surfpool

# Build
cargo surfpool-install
```

### Start a Surfnet

```console
$ surfpool start
```

## 🛠 Quickstart

### List runbooks available in this repository
```console
$ surfpool ls
Name                                    Description
deployment                              Deploy programs
```

### Start a Surfnet, automatically executing the `deployment` runbook on program recompile:
```console
$ surfpool start --watch
```

### Execute an existing runbook
```console
$ surfpool run deployment
```

# 🧩 anchor-mplxcore
### An Anchor Program for Managing MPL Core Collections & NFTs

`anchor-mplxcore` is a Solana Anchor program that provides a secure, authority-controlled workflow for interacting with Metaplex Core (MPL Core).  
It allows authorized creators to:

- Whitelist new creators
- Create Metaplex Core collections
- Mint NFTs with metadata + plugins
- Freeze and thaw assets
- Update NFT metadata

All functionality is backed by deterministic PDAs and strict authority validation.

---

## ✨ Features

### 🔐 Whitelist Enforcement

Only approved creators may initialize MPL Core collections.

- Whitelisted creators are stored in a PDA
- Only the program upgrade authority can modify the whitelist
- Prevents unauthorized collection creation

**Whitelist PDA:**
seeds = ["whitelist"]

---

### 🏛 Collection Creation

Creates a new MPL Core collection via CPI, while establishing an on-chain authority record.

- Validates creator is whitelisted
- Initializes a PDA that acts as the “collection authority”
- Stores default NFT metadata (name + URI)
- Calls `CreateCollectionV2` from MPL Core
- Plugin and external adapter support

**Collection Authority PDA:**
plugin = FreezeDelegate { frozen: true }


---

### 🔥 Thaw an NFT

Reverses freezing by updating the same plugin:

plugin = FreezeDelegate { frozen: false }


- Authorized only by the collection authority
- Uses `UpdatePluginV1` CPI

---

### ✏️ Update NFT Metadata

Allows authorized updates to an NFT’s name and URI.

- Authority-only instruction
- Uses MPL Core’s `UpdateV1` CPI
- Pulls the persistent metadata from the `collection_authority` PDA
- Ensures metadata updates remain consistent across the collection

---

## 🧬 Program PDAs

All program behavior is anchored around deterministic PDAs.

| PDA Name               | Seeds                                                   | Purpose |
|------------------------|---------------------------------------------------------|---------|
| `whitelisted_creators` | `["whitelist"]`                                         | Stores approved creators |
| `collection_authority` | `["collection_authority", collection_pubkey]`           | Authorizes mint/freeze/thaw/update operations |

These PDAs are used with `invoke_signed` to ensure only program-derived authorities can mutate MPL Core assets.

---

## 🧭 Instruction Summary

| Instruction          | Description |
|----------------------|-------------|
| `whitelist_creator`  | Adds creators to the whitelist; only upgrade authority may call |
| `create_collection`  | Initializes a new MPL Core collection + collection authority PDA |
| `mint_nft`           | Mints a frozen NFT with metadata + plugins |
| `freeze_nft`         | Freezes an asset via FreezeDelegate plugin |
| `thaw_nft`           | Thaws an asset |
| `update_nft`         | Updates NFT name and URI |

---

## 🧱 Architecture Overview
+-----------------------------+
| anchor-mplxcore |
+-----------------------------+
| whitelist_creator()
| create_collection()
| mint_nft()
| freeze_nft()
| thaw_nft()
| update_nft()
|
v
+------------------------------+
| CollectionAuthority PDA |
+------------------------------+
|
v
+------------------------------+
| MPL Core CPI API |
| (CreateV2, UpdateV1, etc.) |
+------------------------------+
|
v
+------------------------------+
| Collections / Assets |
+------------------------------+


---

## 🛠 Development

Build, test, and deploy using standard Anchor commands:

anchor build
anchor test
anchor deploy


**Requirements:**

- Rust
- Solana CLI
- Anchor
- MPL Core CPI crate (`mpl_core`)  