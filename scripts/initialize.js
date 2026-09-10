/**
 * DeFi Scars — One-Time Program Initializer
 *
 * Calls the `initialize` instruction on the deployed Anchor program
 * to create the GlobalState PDA account (owner, next_id, total_scars).
 *
 * Usage:
 *   node scripts/initialize.js
 */

const fs = require('fs');
const path = require('path');
const os = require('os');
const crypto = require('crypto');
const {
    Connection,
    PublicKey,
    Keypair,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction
} = require('@solana/web3.js');

// ── Configuration ───────────────────────────────────────────
const CONFIG = {
    SOLANA_RPC:      'https://api.devnet.solana.com',
    PROGRAM_ID:      '3EUbCw45m9gWr32QJ4muF4SiofFgggGzqTpWJEfKEhiV',
    TREASURY_WALLET: 'Cswy3cTVwwjWA7CSDjr2w3o7gRderZSNhpnCekmbjVCa',
};

async function main() {
    console.log('--- DeFi Scars Program Initializer ---');
    console.log(`RPC:         ${CONFIG.SOLANA_RPC}`);
    console.log(`Program ID:  ${CONFIG.PROGRAM_ID}`);
    console.log(`Treasury:    ${CONFIG.TREASURY_WALLET}\n`);

    // 1. Locate and load deployer keypair
    let keypairPath = path.join(os.homedir(), '.config', 'solana', 'id.json');
    
    // In WSL, check default WSL path if not on Windows
    if (!fs.existsSync(keypairPath)) {
        const wslPath = '/home/kehnmarv/.config/solana/id.json';
        if (fs.existsSync(wslPath)) keypairPath = wslPath;
    }

    if (!fs.existsSync(keypairPath)) {
        console.error(`Error: Solana keypair not found at ${keypairPath}`);
        console.error('Run: solana-keygen new --outfile ~/.config/solana/id.json');
        process.exit(1);
    }

    const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, 'utf8')));
    const payer = Keypair.fromSecretKey(secretKey);
    console.log(`Payer/Deployer: ${payer.publicKey.toBase58()}`);

    const connection = new Connection(CONFIG.SOLANA_RPC, 'confirmed');
    const programId = new PublicKey(CONFIG.PROGRAM_ID);
    const treasuryPubkey = new PublicKey(CONFIG.TREASURY_WALLET);

    // 2. Derive GlobalState PDA: seeds = [b"global"]
    const [globalStatePDA, bump] = PublicKey.findProgramAddressSync(
        [Buffer.from('global')],
        programId
    );
    console.log(`GlobalState PDA: ${globalStatePDA.toBase58()} (bump: ${bump})`);

    // 3. Check if already initialized
    const existing = await connection.getAccountInfo(globalStatePDA);
    if (existing) {
        console.log('\n[SUCCESS] GlobalState PDA already exists and is initialized!');
        return;
    }

    // 4. Compute Anchor instruction discriminator for "global:initialize"
    const disc = crypto.createHash('sha256').update('global:initialize').digest().subarray(0, 8);

    // Data layout: disc(8) + owner(32)
    const data = Buffer.concat([disc, treasuryPubkey.toBuffer()]);

    const instruction = new TransactionInstruction({
        programId,
        keys: [
            { pubkey: globalStatePDA, isSigner: false, isWritable: true },
            { pubkey: payer.publicKey, isSigner: true,  isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        data,
    });

    console.log('Sending initialize transaction to Devnet...');
    const tx = new Transaction().add(instruction);
    const sig = await sendAndConfirmTransaction(connection, tx, [payer]);

    console.log('\n======================================================');
    console.log(' Program successfully initialized!');
    console.log(` Signature:  https://explorer.solana.com/tx/${sig}?cluster=devnet`);
    console.log('======================================================\n');
}

main().catch((err) => {
    console.error('Initialization failed:', err);
    process.exit(1);
});
