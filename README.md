# BlockChain — Money (RTC)

A proof-of-concept blockchain written in Rust. Implements Proof of Work consensus, Ed25519 transaction signing, balance tracking, and chain integrity validation. Currently runs as a local in-memory simulation — no networking yet.

---

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Data Structures](#data-structures)
- [Cryptography](#cryptography)
- [Blockchain Logic](#blockchain-logic)
- [Running the Project](#running-the-project)
- [Current Limitations](#current-limitations)
- [Roadmap — Phase 2: Network & Web](#roadmap--phase-2-network--web)

---

## Overview

**Currency:** RTC (Money)  
**Consensus:** Proof of Work (SHA-256)  
**Mining Difficulty:** 5 leading zeros  
**Block Reward:** 50 RTC per mined block  
**Initial Supply:** 1,000,000 RTC (seeded to genesis address)  
**Signatures:** Ed25519  

```
Money - Rust blockchain
================================
Creating wallets...
Alice: a3f2...
Bob:   7c91...
Miner: 0dbe...

Mining the first block...
Mining... nonce=173842, hash=00000a3f...
```

---

## Architecture

```
src/
├── main.rs          — CLI demo: wallets, transactions, mining, attack simulation
├── block.rs         — Block struct and SHA-256 hash calculation
├── blockchain.rs    — Chain state, mempool, balance ledger, validation
├── transaction.rs   — Transaction struct, hashing, coinbase
├── wallet.rs        — Ed25519 key generation, address derivation, signing
└── mining.rs        — Proof of Work loop and difficulty check
```

### Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `sha2` | 0.10 | SHA-256 hashing for blocks and transactions |
| `hex` | 0.4 | Hex encoding for hashes, keys, and signatures |
| `ed25519-dalek` | 2 | Ed25519 keypair generation and digital signatures |
| `rand` | 0.8 | Cryptographically secure `OsRng` for key generation |
| `serde` + `serde_json` | 1 | Struct serialization for block hashing |
| `chrono` | 0.4 | Unix timestamps on blocks |

---

## Data Structures

### Block

```rust
pub struct Block {
    pub index: u64,                     // Block height
    pub timestamp: u64,                 // Unix timestamp (seconds)
    pub transactions: Vec<Transaction>, // Transactions included in block
    pub previous_hash: String,          // SHA-256 hash of previous block
    pub nonce: u64,                     // Proof of Work nonce
    pub hash: String,                   // SHA-256 hash of this block
}
```

**Hash input:** `index || timestamp || serde_json(transactions) || previous_hash || nonce`

**Genesis block:** index 0, timestamp 0, previous_hash = 64 zeros, contains a single coinbase to `genesis_address` for 0.0 RTC.

---

### Transaction

```rust
pub struct Transaction {
    pub sender: String,     // Sender address (40-char hex)
    pub reciever: String,   // Receiver address (40-char hex)
    pub amount: f64,        // Amount in RTC
    pub signature: String,  // Ed25519 signature (128-char hex)
}
```

**Transaction hash input:** `sender || receiver || amount`

**Coinbase transactions:** `sender = "COINBASE"`, `signature = "COINBASE"` — automatically verified as valid, cannot be submitted manually.

---

### Wallet

```rust
pub struct Wallet {
    signing_key: SigningKey,       // Ed25519 private key (32 bytes)
    pub verifying_key: VerifyingKey, // Ed25519 public key (32 bytes)
    pub address: String,           // 40-char hex address
}
```

**Address derivation:** `hex( SHA-256(public_key_bytes)[0..20] )` → 40 hex characters (160-bit address space)

---

### Blockchain State

```rust
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub balances: HashMap<String, f64>,
    pub public_keys: HashMap<String, String>,
}
```

---

## Cryptography

| Component | Algorithm | Details |
|---|---|---|
| Block hashing | SHA-256 | Via `sha2` crate, output as 64-char hex string |
| Transaction hashing | SHA-256 | Same as above |
| Address derivation | SHA-256 | First 20 bytes of public key hash |
| Key generation | Ed25519 | `OsRng` — cryptographically secure entropy |
| Signing | Ed25519 | Signs SHA-256 hash of transaction data |
| Verification | Ed25519 | Reconstructs key from stored public key hex |
| Proof of Work | SHA-256 | Hash must start with `"00000"` (5 leading zeros) |

---

## Blockchain Logic

### `add_transaction`
1. Rejects COINBASE transactions submitted by users
2. Verifies Ed25519 signature against registered public key
3. Checks sender has a registered public key
4. Validates sender balance >= amount
5. Validates amount > 0
6. Pre-deducts amount from sender balance
7. Appends to mempool

### `mine_pending_transactions`
1. Creates coinbase reward transaction (50 RTC) for miner
2. Combines coinbase + mempool into block transaction list
3. Creates new Block with incremented index and current timestamp
4. Runs PoW mining loop: increments nonce until hash starts with `"00000"`
5. Credits balances for all included transactions
6. Appends block to chain and clears mempool

### `is_chain_valid`
- Recomputes each block's hash and checks it matches stored hash
- Checks each block's `previous_hash` matches the actual previous block hash
- Checks each block hash meets PoW difficulty requirement
- Returns `true` only if all checks pass

---

## Running the Project

```bash
# Clone and build
git clone https://github.com/blessedlab/BlockChain.git
cd BlockChain
cargo run
```

The demo in `main.rs` will:
1. Create three wallets (Alice, Bob, Miner)
2. Fund Alice with 500 RTC
3. Alice sends 100 RTC to Bob and 50 RTC to Miner
4. Miner mines block 1 (PoW, ~5–15s depending on hardware)
5. Bob sends 30 RTC to Alice
6. Miner mines block 2
7. Chain is printed and validated
8. Simulated attack: tampers with block 1 — chain validation detects it
9. Security test: Bob tries to send 99,999 RTC (rejected: insufficient funds)

---

## Current Limitations

| Issue | Details |
|---|---|
| No networking | Fully in-memory, single process only |
| No persistence | All state lost on exit |
| No tests | Only manual demo in `main.rs` |
| Fixed difficulty | Hardcoded at 5 (no dynamic adjustment) |
| Float balances | Uses `f64` — subject to precision errors at scale |
| No transaction fees | Only block rewards |
| No replay protection | No nonce/sequence on transactions |
| No key persistence | Private keys not saved to disk |
| Mining performance | Full SHA-256 recalculated every nonce iteration |

---

## Roadmap — Phase 2: Network & Web

The following phases will transform this MVP into a functioning networked blockchain with a web interface for wallets and explorer.

---

### Phase 2.1 — Persistence Layer

**Goal:** Survive restarts. All state written to disk.

- [ ] Integrate `sled` embedded database for block and transaction storage
- [ ] Serialize/deserialize `Block` and `Transaction` with `serde`
- [ ] Persist `balances` and `public_keys` maps on every write
- [ ] Load chain from disk on startup, validate integrity before accepting new blocks
- [ ] Export wallet keypairs to encrypted JSON keystore files (password-derived key with `argon2`)

---

### Phase 2.2 — HTTP REST API

**Goal:** Any HTTP client can interact with the node.

**Crates:** `axum` (async HTTP), `tokio` (async runtime), `serde_json`

**Endpoints:**

| Method | Path | Description |
|---|---|---|
| `GET` | `/blocks` | List all blocks |
| `GET` | `/blocks/:index` | Get block by height |
| `GET` | `/blocks/latest` | Get latest block |
| `GET` | `/transactions/mempool` | List pending transactions |
| `POST` | `/transactions` | Submit a signed transaction |
| `GET` | `/balance/:address` | Get balance for address |
| `POST` | `/wallets/register` | Register public key for address |
| `POST` | `/mine` | Trigger mining (dev/testnet only) |
| `GET` | `/chain/validate` | Validate chain integrity |
| `GET` | `/node/peers` | List connected peers |

**Example transaction submission:**
```json
POST /transactions
{
  "sender": "a3f2...",
  "reciever": "7c91...",
  "amount": 100.0,
  "signature": "3d7a..."
}
```

---

### Phase 2.3 — Peer-to-Peer Networking

**Goal:** Multiple nodes discover each other and stay in sync.

**Crates:** `tokio` (async TCP), `serde_json` (message protocol)

- [ ] Node identity: keypair-based node ID
- [ ] Peer list: static seed nodes + dynamic discovery
- [ ] TCP message protocol over `tokio::net::TcpStream`
- [ ] Message types:
  - `NewTransaction` — broadcast new tx to all peers
  - `NewBlock` — broadcast mined block to all peers
  - `GetBlocks(from_index)` — request chain sync
  - `Blocks(vec)` — respond with block range
  - `Ping` / `Pong` — keepalive
- [ ] Fork resolution: longest valid chain wins
- [ ] Mempool sync on peer connect

**Node configuration** (`node.toml`):
```toml
[node]
port = 8333
data_dir = "./data"
seed_peers = ["127.0.0.1:8334", "127.0.0.1:8335"]

[mining]
difficulty = 5
reward = 50.0
miner_address = "your_address_here"
```

---

### Phase 2.4 — Web Interface

**Goal:** Browser-based wallet and block explorer.

**Stack:** `React` + `TypeScript` frontend, communicates with the node's REST API.

#### Wallet UI
- [ ] Generate new wallet (keypair created client-side, never leaves browser)
- [ ] Import existing wallet from keystore file
- [ ] Display address and current RTC balance
- [ ] Send RTC: enter recipient address and amount, sign locally, submit to API
- [ ] Transaction history for address

#### Block Explorer
- [ ] Home: latest blocks and recent transactions
- [ ] Block detail page: all transactions, hash, timestamp, nonce
- [ ] Transaction detail page: sender, receiver, amount, signature status
- [ ] Address page: balance and full transaction history
- [ ] Chain health indicator: latest block height, last block time, mempool size

#### Design targets
- Mobile-friendly responsive layout
- Real-time updates via polling or WebSocket
- No backend login — everything is public chain data + client-side signing

---

### Phase 2.5 — Quality & Security Hardening

- [ ] Replace `f64` balances with integer arithmetic (e.g. 1 RTC = 100,000,000 units, like satoshis)
- [ ] Add transaction nonce/sequence numbers to prevent replay attacks
- [ ] Dynamic difficulty adjustment every N blocks (target ~10s block time)
- [ ] Add transaction fees (fee goes to miner)
- [ ] Unit tests for all modules (`block`, `transaction`, `wallet`, `blockchain`)
- [ ] Integration tests for full mine + validate cycle
- [ ] Fix typo: `reciever` → `receiver`
- [ ] Structured logging with `tracing` crate

---

### Milestone Summary

| Phase | Description | Status |
|---|---|---|
| 1.0 | MVP: PoW, wallets, signing, validation | ✅ Done |
| 2.1 | Disk persistence (sled + keystore) | Planned |
| 2.2 | HTTP REST API (axum + tokio) | Planned |
| 2.3 | P2P networking (TCP, block sync, fork resolution) | Planned |
| 2.4 | Web interface (React wallet + block explorer) | Planned |
| 2.5 | Hardening (integer balances, replay protection, tests, fees) | Planned |

---

> Built by Daniel Gidrewicz
