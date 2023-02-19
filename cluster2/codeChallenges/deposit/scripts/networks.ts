import { web3 } from "@project-serum/anchor";

export enum Network {
  "https://api.devnet.solana.com" = "localnet",
  "https://api.mainnet-beta.solana.com" = "mainnetBeta",
  "http://localhost:8899" = "devnet",
}

export const getUrls = (
  network: Network,
  sig?: string,
  type?: "tx" | "address"
) => {
  if (network === Network["https://api.devnet.solana.com"]) {
    return {
      rpc: web3.clusterApiUrl("devnet"),
      bundlrAddress: "https://devnet.bundlr.network",
      bundlrProviderUrl: web3.clusterApiUrl("devnet"),
      explorer: `https://explorer.solana.com/${type}/${sig}?cluster=devnet`,
    };
  } else if (network === Network["https://api.mainnet-beta.solana.com"]) {
    return {
      rpc: web3.clusterApiUrl("mainnet-beta"),
      bundlrAddress: "https://node1.bundlr.network",
      bundlrProviderUrl: web3.clusterApiUrl("mainnet-beta"),
      explorer: `https://explorer.solana.com/${type}/${sig}`,
    };
  } else {
    return {
      rpc: "http://127.0.0.1:8899",
      bundlrAddress: "https://devnet.bundlr.network",
      bundlrProviderUrl: web3.clusterApiUrl("devnet"),
      explorer: `https://explorer.solana.com/${type}/${sig}?cluster=custom`,
    };
  }
};
