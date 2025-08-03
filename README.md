# Solana Learning Playground

This repository contains multiple hands-on examples for learning how to build on the Solana blockchain using **Rust**. Each project explores a specific feature or interaction pattern using `solana-client`, `tokio`, and other foundational tools.

---

## 📦 Projects Included

### 1. `Load_a_local_json_file_keypair`
- **Purpose:** Load a local wallet (keypair) from a JSON file and print the public key.
- **Skills:** Working with file I/O, parsing JSON keypairs, and using `solana_sdk::signature::Keypair`.

### 2. `TestSOL`
- **Purpose:** Airdrop 1 SOL into a given public key and confirm the transaction.
- **Skills:** RPC client usage, token airdrop, and transaction confirmation.
- **Note:** Only works in **localnet** or **devnet**.

### 3. `SubscribingtoEvents`
- **Purpose:** Use WebSocket PubSub to listen to account changes.
- **Skills:** Real-time event subscriptions with async streams.

### 4. `connectingUsingRPC`
- **Purpose:** Connect to a local Solana node via RPC and test connectivity.
- **Skills:** Basic RPC setup with custom commitment level.

---

## ⚙️ Prerequisites

- Rust & Cargo (latest stable)
- Solana CLI tools
- Localnet or Devnet setup
- Internet connection for RPC interactions

---

## 🚀 Getting Started

```bash
git clone https://github.com/ashutoshnegi120/solana-learning.git
cd solana-learning

# Choose any project to run:
cd TestSOL
cargo run
```



> 🔐 Replace public keys in source files with your own if needed.



---

## 🙋‍♂️ Author

Maintained by **ashutoshnegi120**

GitHub: [@ashutoshnegi120](https://github.com/ashutoshnegi120)

---
