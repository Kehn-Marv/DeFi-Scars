# DeFi Scars 🌍🔥

> **Because you learn way more from real losses than from YouTube tutorials.**

**DeFi Scars** is an interactive 3D globe where people pin their real crypto mistakes, the money they lost, and the hard lessons they learned. Spin the Earth, browse mistakes by location, and learn how not to get rekt in Web3.

![Three.js](https://img.shields.io/badge/Three.js-3D_Globe-black?logo=three.js)
![Electron](https://img.shields.io/badge/Electron-Desktop_App-47848F?logo=electron&logoColor=white)
![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?logo=solana&logoColor=white)

---

## 💡 Why DeFi Scars?

In crypto, social media gurus always brag when their trades win, but quietly delete their posts when they lose everything. 

**DeFi Scars changes that:**
* 🛑 **No Rewriting History:** Every lesson is permanently recorded on the Solana blockchain. No deleting mistakes when the market recovers.
* 🛡️ **Zero Spam:** Dropping a pin requires a tiny 0.01 SOL stake. This small friction keeps out bots and scammers, ensuring every story is authentic.
* 🔑 **You Own Your Story:** Your entries are tied directly to your crypto wallet—not locked in a company database.

---

## ✨ Features

* 🌐 **Interactive 3D Earth:** Spin, zoom, and fly across the globe to see mistakes pinned from Tokyo to New York.
* 🃏 **DeFi Trading Cards:** Every mistake is displayed as a trading card showing what went wrong (FOMO buy, rug pull, phishing link, bad leverage) and the lesson learned.
* 📍 **Drop Your Own Pin:** Pick your location on the map, tell what happened, attach photos, and share your advice with the world.
* 💬 **"Saved Me" Upvotes:** Found a lesson that prevented you from making a huge mistake? Upvote it so others can learn from it too.

---

## 🚀 How to Run (2 Simple Steps)

You don't need any complex blockchain tools installed to explore the app!

```bash
# 1. Install dependencies
npm install

# 2. Launch the desktop app
npm start
```

That's it! The 3D globe desktop app will open right on your screen.

---

<details>
<summary>🛠️ <b>Developer & Smart Contract Info (Click to expand)</b></summary>

<br>

### On-Chain Details (Solana Devnet)
The app interacts with an Anchor program on Solana Devnet:
* **Program ID:** `3EUbCw45m9gWr32QJ4muF4SiofFgggGzqTpWJEfKEhiV`
* **Treasury Wallet:** `Cswy3cTVwwjWA7CSDjr2w3o7gRderZSNhpnCekmbjVCa`
* **Stake Fee:** `0.01 SOL` (forwarded to the treasury on submission)

### Deploying Your Own Contract
If you ever want to deploy your own custom copy of the smart contract:
1. Open [beta.solpg.io](https://beta.solpg.io/) (Solana Playground in browser).
2. Paste [`programs/defi_scars/src/lib.rs`](programs/defi_scars/src/lib.rs) and click **Build & Deploy**.
3. Run `npm run initialize` to set up the on-chain storage account.

### Project Structure
```
.
├── programs/defi_scars/  # Solana smart contract (Rust + Anchor)
├── renderer/             # 3D Three.js Globe & Frontend UI
├── scripts/              # Setup and initialization scripts
├── main.js               # Electron Desktop app runner
└── package.json          # Project dependencies
```

</details>

---

## 📜 License
MIT
