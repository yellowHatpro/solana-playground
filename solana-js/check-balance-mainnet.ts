import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";


const connection = new Connection("https://api.mainnet-beta.solana.com")
const mainnetPublicKey = new PublicKey("GgJJRwLg9NzFQ97o1CJLGLp1KLSUMBwFc6eQNVEr4fbW");
const balanceInLamports = await connection.getBalance(mainnetPublicKey);
const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log(`✅ Finished! The balance for the wallet at address ${mainnetPublicKey} is ${balanceInSOL} SOL`);

