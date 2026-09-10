const { app, BrowserWindow, shell, nativeImage, ipcMain } = require('electron');
const path = require('path');
const fs = require('fs');

const iconPath = path.join(__dirname, 'build', 'icon.png');

function createWindow() {
  const win = new BrowserWindow({
    width: 1360,
    height: 900,
    minWidth: 800,
    minHeight: 600,
    backgroundColor: '#070710',
    title: 'DeFi Scars',
    icon: iconPath,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  win.loadFile(path.join(__dirname, 'renderer', 'index.html'));

  // Open external links in the user's browser, not in-app.
  win.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith('http')) {
      shell.openExternal(url);
      return { action: 'deny' };
    }
    return { action: 'allow' };
  });
}

/* ─── IPC Handlers for Wallet Storage ─────────────────── */

const walletDir = () => {
  const dir = path.join(app.getPath('userData'), 'defiscars-wallet');
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
  return dir;
};

const walletPath = () => path.join(walletDir(), 'keystore.json');

// Save encrypted keystore
ipcMain.handle('wallet:save', (_event, encryptedJSON) => {
  fs.writeFileSync(walletPath(), encryptedJSON, 'utf8');
  return true;
});

// Load encrypted keystore
ipcMain.handle('wallet:load', () => {
  const p = walletPath();
  if (fs.existsSync(p)) {
    return fs.readFileSync(p, 'utf8');
  }
  return null;
});

// Check if wallet exists
ipcMain.handle('wallet:exists', () => {
  return fs.existsSync(walletPath());
});

// Delete wallet
ipcMain.handle('wallet:delete', () => {
  const p = walletPath();
  if (fs.existsSync(p)) fs.unlinkSync(p);
  return true;
});

/* ─── App Lifecycle ───────────────────────────────────── */

app.whenReady().then(() => {
  // Set the dock icon on macOS
  if (process.platform === 'darwin' && app.dock) {
    app.dock.setIcon(nativeImage.createFromPath(iconPath));
  }

  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});
