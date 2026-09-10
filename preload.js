/**
 * DeFi Scars — Preload Script
 * Exposes a secure bridge (window.scarsAPI) to the renderer for
 * wallet keystore I/O. All other features (Solana web3, IPFS, crypto)
 * run entirely in the renderer via CDN + Web APIs.
 */
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('scarsAPI', {
  // Wallet keystore (encrypted Solana keypair)
  saveWallet:   (data) => ipcRenderer.invoke('wallet:save', data),
  loadWallet:   ()     => ipcRenderer.invoke('wallet:load'),
  walletExists: ()     => ipcRenderer.invoke('wallet:exists'),
  deleteWallet: ()     => ipcRenderer.invoke('wallet:delete'),
});
