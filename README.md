# DeFi Scars 🌍🔥

A global map where every pin is a real mistake someone made in DeFi, with the lesson attached. Browse by location, by mistake type, or just spin the globe and learn from strangers' losses. 

Because you learn better from real losses than from tutorials.

![Three.js](https://img.shields.io/badge/Three.js-r160-000000?logo=three.js)
![Electron](https://img.shields.io/badge/Electron-31-47848F?logo=electron&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?logo=solana&logoColor=white)

## The Onchain Justification

**Problem:** Social media is filled with "fake gurus" who rewrite history to look smarter than they were, or who post "I lost money panic-selling" *after* the market has already recovered. 
**Solution:** The hash of every entry gets stored on the Solana blockchain — providing proof of integrity, not just data storage.

1. **Tamper-Proof Timestamps:** When someone submits a mistake/lesson, the content (text + category + timestamp) is hashed and written to a Solana program. Anyone can verify the entry hasn't been edited or backdated after the fact.
2. **Economic Spam Resistance:** A small stake-to-post mechanic (a 0.01 SOL fee) is required to pin a scar to the globe. This creates real economic friction that keeps out spam and low-effort junk, without relying on fancy AI moderation.

### Your Wallet Is Your Identity
Your entries are stored securely on Solana (the hash) and tied directly to your wallet address. The app itself is just a window into that data. If you delete the app, your data remains safely anchored to your wallet identity on-chain.

## Features
- **Photorealistic 3D Earth:** Built with Three.js, featuring bloom post-processing and an interactive spinning globe.
- **Location-Based Searching:** Fly the camera to any coordinate or city in the world to drop a pin.
- **DeFi-Themed Trading Cards:** Every mistake is rendered as a beautifully styled trading card.
- **Onchain Anchoring:** Connects to Solana Devnet to securely hash and anchor your lessons via an Anchor program.
- **Stake-to-Post:** Requires 0.01 SOL to commit your scar to the globe permanently.

## ⚙️ On-Chain Configuration

The repository comes pre-configured with default Solana Devnet parameters in `renderer/index.html`:

| Parameter | Value | Description |
|---|---|---|
| **Network** | Solana Devnet (`api.devnet.solana.com`) | Target cluster for testing |
| **Program ID** | `3EUbCw45m9gWr32QJ4muF4SiofFgggGzqTpWJEfKEhiV` | Deployed Anchor program address |
| **Treasury Wallet** | `Cswy3cTVwwjWA7CSDjr2w3o7gRderZSNhpnCekmbjVCa` | Recipient address for scar stakes (0.01 SOL) |
| **Stake Fee** | `0.01 SOL` (10,000,000 lamports) | Anti-spam stake forwarded to treasury |

---

## 🚀 Getting Started

### Quick Start (Desktop App)
You **do not** need Rust, Solana CLI, or Anchor installed to run and test the app. The desktop app runs directly with Electron:

```bash
npm install     # Install dependencies
npm start       # Launch the 3D globe desktop app
```

---

### Smart Contract Deployment (Optional — For Developers)

If you wish to compile and deploy your own instance of the program:

#### Option A: Zero-Setup via Solana Playground (Recommended)
1. Open [beta.solpg.io](https://beta.solpg.io/) in your browser.
2. Create an Anchor project and paste [`programs/defi_scars/src/lib.rs`](programs/defi_scars/src/lib.rs).
3. Click **Build & Deploy** to Devnet directly in the browser (no local toolchains required).

#### Option B: Local Anchor CLI (Linux / macOS / WSL)
```bash
# 1. Build the Anchor program
anchor build

# 2. Deploy to Solana Devnet
anchor deploy --provider.cluster devnet
```

#### One-Time Program Initialization
Every freshly deployed Anchor program requires a one-time transaction to initialize the `global_state` PDA (`owner = treasury`, `next_id = 1`, `total_scars = 0`):

```bash
npm run initialize
```
*(Runs [`scripts/initialize.js`](scripts/initialize.js) which automatically derives the PDA and submits the initialization transaction to Devnet).*

## Project Structure
```
.
├── programs/            # Anchor/Rust smart contracts
│   └── defi_scars/
│       ├── Cargo.toml   # Rust dependencies
│       └── src/
│           └── lib.rs   # Core Solana program logic
├── scripts/             # Automation scripts
│   └── initialize.js    # One-time program initializer
├── Anchor.toml          # Anchor workspace config
├── renderer/            # Frontend (Three.js globe + UI + Solana Web3)
│   ├── index.html       # Main application view
│   └── styles.css       # DeFi-inspired styling
├── main.js              # Electron main process
├── preload.js           # Electron preload (wallet IPC bridge)
└── package.json         # Dependencies and scripts
```

## License
MIT
