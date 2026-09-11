# DeFi Scars

A global map where every pin is a real mistake someone made in DeFi, with the lesson attached. Browse by location, by mistake type, or just spin the globe and learn from strangers' losses.

Because you learn better from real losses than from tutorials.

![Three.js](https://img.shields.io/badge/Three.js-3D_Globe-black?logo=three.js)
![Electron](https://img.shields.io/badge/Electron-Desktop_App-47848F?logo=electron&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?logo=solana&logoColor=white)

---

## The Onchain Justification

**Problem:** Social media is filled with "fake gurus" who rewrite history to look smarter than they were, or who post "I lost money panic-selling" after the market has already recovered.

**Solution:** The hash of every entry gets stored on the Solana blockchain — providing proof of integrity, not just data storage.

* **Tamper-Proof Timestamps:** When someone submits a mistake/lesson, the content (text + category + timestamp) is hashed and written to a Solana program. Anyone can verify the entry hasn't been edited or backdated after the fact.
* **Economic Spam Resistance:** A small stake-to-post mechanic (a 0.01 SOL fee) is required to pin a scar to the globe. This creates real economic friction that keeps out spam and low-effort junk, without relying on fancy AI moderation.

### Your Wallet Is Your Identity
Your entries are stored securely on Solana (the hash) and tied directly to your wallet address. The app itself is just a window into that data. If you delete the app, your data remains safely anchored to your wallet identity on-chain.

---

## Features

* **Photorealistic 3D Earth:** Built with Three.js, featuring bloom post-processing and an interactive spinning globe.
* **Location-Based Searching:** Fly the camera to any coordinate or city in the world to drop a pin.
* **DeFi-Themed Trading Cards:** Every mistake is rendered as a beautifully styled trading card.
* **Onchain Anchoring:** Connects to Solana Devnet to securely hash and anchor your lessons via an Anchor program.
* **Stake-to-Post:** Requires 0.01 SOL to commit your scar to the globe permanently.

---

## Getting Started

### Quick Start (Desktop App)

You do not need any complex blockchain tools installed to run and explore the app:

```bash
# 1. Install dependencies
npm install

# 2. Launch the desktop app
npm start
```

---

<details>
<summary>Developer and Smart Contract Details (Click to expand)</summary>

<br>

### On-Chain Details (Solana Devnet)
The app interacts with an Anchor program on Solana Devnet:
* **Program ID:** `3EUbCw45m9gWr32QJ4muF4SiofFgggGzqTpWJEfKEhiV`
* **Treasury Wallet:** `Cswy3cTVwwjWA7CSDjr2w3o7gRderZSNhpnCekmbjVCa`
* **Stake Fee:** `0.01 SOL` (forwarded to the treasury on submission)

### Deploying Your Own Contract
If you want to compile and deploy your own copy of the smart contract:
1. Open [beta.solpg.io](https://beta.solpg.io/) (Solana Playground in browser).
2. Paste `programs/defi_scars/src/lib.rs` and click **Build & Deploy**.
3. Run `npm run initialize` to set up the on-chain storage account.

### Project Structure
```
.
├── programs/defi_scars/  # Solana smart contract (Rust + Anchor)
├── renderer/             # 3D Three.js Globe and Frontend UI
├── scripts/              # Setup and initialization scripts
├── main.js               # Electron Desktop app runner
└── package.json          # Project dependencies
```

</details>

---

## License
MIT
