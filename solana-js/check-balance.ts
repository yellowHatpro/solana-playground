import { Connection, PublicKey, clusterApiUrl, LAMPORTS_PER_SOL } from "@solana/web3.js"

// Solana has different groups of validators, knows as clusters.
// 3 types of clusters: Mainnet(live prod env), Devnet(public testing and dev), Testnet(validator and stress testing)
//const connection = new Connection(clusterApiUrl("devnet"));
const addressVal = "FSoXk6if9ik34EdLUbc9p1JPZzKspdSKTbqqh8LsnrqG";
if (!PublicKey.isOnCurve(addressVal)) {
  throw Error("Wrong address");
}
const address = new PublicKey(addressVal);

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const balance = await connection.getBalance(address);
const balanceInSol = balance / LAMPORTS_PER_SOL;
console.log(`Balance of the account at ${address} is ${balanceInSol} SOL`);
console.log("✅ Finished!")

