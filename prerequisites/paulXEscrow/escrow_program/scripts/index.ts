import {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  SendOptions,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import fs from "fs";

export const NETWORK: "devnet" | "mainnet-beta" | "localnet" = "localnet";
export const getUrls = (
  network: "devnet" | "mainnet-beta" | "localnet",
  sig?: string,
  type?: "tx" | "address"
) => {
  if (network === "devnet") {
    return {
      rpc: clusterApiUrl("devnet"),
      bundlrAddress: "https://devnet.bundlr.network",
      bundlrProviderUrl: clusterApiUrl("devnet"),
      explorer: `https://explorer.solana.com/${type}/${sig}?cluster=devnet`,
    };
  } else if (network === "mainnet-beta") {
    return {
      rpc: clusterApiUrl("mainnet-beta"),
      bundlrAddress: "https://node1.bundlr.network",
      bundlrProviderUrl: clusterApiUrl("mainnet-beta"),
      explorer: `https://explorer.solana.com/${type}/${sig}`,
    };
  } else {
    return {
      rpc: "http://127.0.0.1:8899",
      bundlrAddress: "https://devnet.bundlr.network",
      bundlrProviderUrl: clusterApiUrl("devnet"),
      explorer: `https://explorer.solana.com/${type}/${sig}?cluster=custom`,
    };
  }
};

export const connection = new Connection(getUrls(NETWORK).rpc);

export const initializeKeypair = async (
  connection: Connection,
  name: string
): Promise<Keypair> => {
  const keys = JSON.parse(fs.readFileSync("keys.json", "utf8"));
  if (!keys[name]) {
    console.log("Creating new keypair");
    const signer = Keypair.generate();
    const newKeys = { ...keys, [name]: `[${signer.secretKey.toString()}]` };

    fs.writeFileSync("keys.json", JSON.stringify(newKeys));
    await airdropSolIfNeeded(signer, connection);
    return signer;
  }

  const _keys = JSON.parse(fs.readFileSync("keys.json", "utf8"));
  const secret = JSON.parse(_keys[name]);
  const secretKey = Uint8Array.from(secret);
  const keypairFromSecretKey = Keypair.fromSecretKey(secretKey);
  await airdropSolIfNeeded(keypairFromSecretKey, connection);
  console.log(`${name}:`, keypairFromSecretKey.publicKey.toBase58());
  return keypairFromSecretKey;
};

async function airdropSolIfNeeded(signer: Keypair, connection: Connection) {
  const balance = await connection.getBalance(signer.publicKey);
  console.log("Current balance is", balance / LAMPORTS_PER_SOL);

  if (balance < LAMPORTS_PER_SOL) {
    console.log("Airdropping 1 SOL...");
    const airdropSignature = await connection.requestAirdrop(
      signer.publicKey,
      LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });

    const newBalance = await connection.getBalance(signer.publicKey);
    console.log("New balance is", newBalance / LAMPORTS_PER_SOL);
  }
}

export const createAndSendV0Tx = async (
  instructions: TransactionInstruction[],
  connection: Connection,
  payer: Keypair,
  signers?: Keypair[],
  options?: SendOptions
) => {
  const latestBlockHash = await connection.getLatestBlockhash("confirmed");
  const messageV0 = new TransactionMessage({
    payerKey: payer.publicKey,
    recentBlockhash: latestBlockHash.blockhash,
    instructions,
  }).compileToV0Message();
  const transaction = new VersionedTransaction(messageV0);
  transaction.sign([payer, ...(signers as Keypair[])]);
  const signature = await connection.sendTransaction(transaction, options);
  const confirmation = await connection.confirmTransaction({
    signature,
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
  });
  if (confirmation.value.err) {
    throw new Error("Transaction not confirmed.");
  }
  return signature;
};
