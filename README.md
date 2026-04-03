
# 🪙 Piggy Bank DApp (Stellar Soroban)

A **fully permissionless decentralized Piggy Bank** built on Stellar using Soroban smart contracts.

> 💡 Save funds on-chain, allow anyone to contribute, and withdraw only as the owner — no admins, no restrictions.

---

# 🚀 Features

* 🆓 **Permissionless Creation** — Anyone can create a piggy bank
* 💰 **Open Deposits** — Anyone can deposit into any piggy bank
* 🔐 **Secure Withdrawals** — Only the owner can withdraw funds
* 📊 **Balance Tracking** — View savings anytime
* ⚡ Built with **Soroban (Rust)** smart contracts

---

# 🧠 Core Concept

Each piggy bank is uniquely identified by an `id` and contains:

* `owner` → Address of creator
* `balance` → Stored amount

### 🔑 Rules

| Action   | Permission    |
| -------- | ------------- |
| Create   | Anyone ✅      |
| Deposit  | Anyone ✅      |
| Withdraw | Owner only 🔐 |

---

# 📁 Project Structure

```bash
piggy-bank-dapp/
│
├── contracts/
│   └── piggy_bank/
│       └── src/
│           └── lib.rs
│
├── frontend/ (optional)
│
└── README.md
```

---

# ⚙️ Smart Contract Overview

### 🔨 Functions

#### 🪙 `create(owner, id)`

Creates a new piggy bank

* `owner`: Address of creator
* `id`: Unique identifier

---

#### 💰 `deposit(id, amount)`

Deposit into a piggy bank

* Anyone can call
* Increases stored balance

---

#### 🏧 `withdraw(id, caller)`

Withdraw funds

* Only owner allowed
* Resets balance to 0

---

#### 📊 `get_balance(id)`

Returns current balance

---

#### 👤 `get_owner(id)`

Returns owner of piggy bank

---

# 🔧 Setup & Installation

## 1️⃣ Install Stellar CLI

```bash
cargo install --locked soroban-cli
```

---

## 2️⃣ Build Contract

```bash
soroban contract build
```

---

## 3️⃣ Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/piggy_bank.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```

---

## 4️⃣ Invoke Contract

### Create Piggy Bank

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <USER> \
  --network testnet \
  -- create \
  --owner <USER_ADDRESS> \
  --id bank1
```

---

### Deposit

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <USER> \
  --network testnet \
  -- deposit \
  --id bank1 \
  --amount 100
```

---

### Withdraw

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER> \
  --network testnet \
  -- withdraw \
  --id bank1 \
  --caller <OWNER_ADDRESS>
```

---

# 🌐 Frontend (Optional)

You can integrate with:

* React + Stellar SDK
* Wallet: Freighter

### UI Sections

* 🪙 Create Piggy Bank
* 💰 Deposit Funds
* 🏧 Withdraw Funds
* 📊 View Balance

---

# ⚠️ Important Notes

* ❗ This contract currently tracks **internal balances only**
* ❗ It does NOT transfer real XLM or tokens yet

👉 For production:

* Integrate Stellar Asset transfers
* Add token support

---

# 🔮 Future Improvements

* ⏳ Time-locked savings
* 🎯 Goal-based piggy bank
* 👥 Multi-user piggy bank
* 💸 Real XLM/token transfers
* 📱 Mobile-friendly UI

---

# 🛡️ Security Considerations

* Ownership verified using `require_auth()`
* No admin or centralized control
* Duplicate bank IDs are prevented

---

# 📜 License

MIT License

---

# ❤️ Contributing

Pull requests are welcome!
Feel free to fork and improve 🚀



contract address CB4EKESQDWWCN5XO4OIEYPBNRFCZXZEDAEBPDUFJOCUJLUSK6SZZ67Y2
https://stellar.expert/explorer/testnet/contract/CB4EKESQDWWCN5XO4OIEYPBNRFCZXZEDAEBPDUFJOCUJLUSK6SZZ67Y2

<img width="959" height="470" alt="image" src="https://github.com/user-attachments/assets/85c83cd7-53db-4b22-bbf0-737630d34f8e" />

