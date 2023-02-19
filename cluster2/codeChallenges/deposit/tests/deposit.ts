import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { initializeKeypair } from "../scripts/initializeKeypair";
import { getUrls, Network } from "../scripts/networks";
import { Deposit } from "../target/types/deposit";
import { execSync } from "child_process";
import { BN } from "bn.js";
import { assert } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getMint,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Metaplex } from "@metaplex-foundation/js";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import fs from "fs";

describe("deposit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Deposit as Program<Deposit>;
  let depositAccount: Keypair;
  let depositAuth: Keypair;
  let pdaAuth: PublicKey;
  let pdaAuthBump: number;
  let solVault: PublicKey;
  let solVaultBump: number;
  const mint = anchor.web3.Keypair.generate();
  const usdcAuth = anchor.web3.Keypair.generate();

  execSync(
    `anchor idl init --filepath target/idl/deposit.json ${program.programId}`,
    { stdio: "inherit" }
  );

  before(async () => {
    depositAccount = await initializeKeypair(
      program.provider.connection,
      "DEPOSIT_ACCOUNT"
    );
    depositAuth = await initializeKeypair(
      program.provider.connection,
      "DEPOSIT_AUTH"
    );
    [pdaAuth, pdaAuthBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("auth"), depositAccount.publicKey.toBuffer()],
      program.programId
    );
    console.log("PDA_AUTH:", pdaAuth.toBase58());
    [solVault, solVaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("sol_vault"), pdaAuth.toBuffer()],
      program.programId
    );
    console.log("SOL_VAULT_AUTH:", solVault.toBase58());
  });

  it("Initialize", async () => {
    try {
      // Add your test here.
      const sig = await program.methods
        .initialize()
        .accounts({
          depositAuth: depositAuth.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
        })
        .signers([depositAccount, depositAuth])
        .rpc();
      console.log(
        "Your transaction signature",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      const data = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("DATA:", data);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Deposit Native", async () => {
    try {
      let beforeBalance = await program.provider.connection.getBalance(
        solVault
      );
      console.log("Before SOL:", beforeBalance);
      const sig = await program.methods
        .depositNative(new BN(0.05 * LAMPORTS_PER_SOL))
        .accounts({
          depositAccount: depositAccount.publicKey,
          depositAuth: depositAuth.publicKey,
          pdaAuth,
          solVault,
        })
        .signers([depositAuth])
        .rpc();
      console.log(
        "Your transaction signature",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      const data = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("DATA:", data);
      let afterBalance = await program.provider.connection.getBalance(solVault);
      console.log("After SOL:", afterBalance);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Withdraw Native", async () => {
    try {
      let beforeBalance = await program.provider.connection.getBalance(
        solVault
      );
      console.log("Before SOL:", beforeBalance);
      const sig = await program.methods
        .withdrawNative(new BN(0.05 * LAMPORTS_PER_SOL))
        .accounts({
          depositAccount: depositAccount.publicKey,
          depositAuth: depositAuth.publicKey,
          pdaAuth,
          solVault,
        })
        .signers([depositAuth])
        .rpc();
      console.log(
        "Your transaction signature",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      const data = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("DATA:", data);
      let afterBalance = await program.provider.connection.getBalance(solVault);
      console.log("After SOL:", afterBalance);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Create SPL Token", async () => {
    try {
      const tokenMint = await createMint(
        program.provider.connection,
        depositAuth,
        usdcAuth.publicKey,
        usdcAuth.publicKey,
        6,
        mint,
        null
      );
      const _tokenMint = await getMint(program.provider.connection, tokenMint);
      console.log(_tokenMint);
      const depositAuthUSDCTokenAccount =
        await getOrCreateAssociatedTokenAccount(
          program.provider.connection,
          depositAuth,
          mint.publicKey,
          depositAuth.publicKey,
          false
        );
      const sig = await mintToChecked(
        program.provider.connection,
        depositAuth,
        mint.publicKey,
        depositAuthUSDCTokenAccount.address,
        usdcAuth,
        200e6,
        6
      );
      console.log(
        "SPL Token Minted:",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Deposit SPL Token", async () => {
    try {
      const solVault = getAssociatedTokenAddressSync(
        mint.publicKey,
        pdaAuth,
        true
      );
      const depositorTokenAccount = getAssociatedTokenAddressSync(
        mint.publicKey,
        depositAuth.publicKey,
        false
      );
      let sig = await program.methods
        .depositSpl(new BN(25e6))
        .accounts({
          depositAccount: depositAccount.publicKey,
          pdaAuth,
          solVault,
          tokenMint: mint.publicKey,
          depositAuth: depositAuth.publicKey,
          depositorTokenAccount,
        })
        .signers([depositAuth])
        .rpc();
      console.log(
        "SPL Token Withdraw:",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      let solVaultAfterBalance =
        await program.provider.connection.getTokenAccountBalance(solVault);
      console.log("After SolVault SPL Balance:", solVaultAfterBalance);
      let depositorAfterBalance =
        await program.provider.connection.getTokenAccountBalance(
          depositorTokenAccount
        );
      console.log("After Depositor SPL Balance:", depositorAfterBalance);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Withdraw SPL Token", async () => {
    try {
      const solVault = getAssociatedTokenAddressSync(
        mint.publicKey,
        pdaAuth,
        true
      );
      const depositorTokenAccount = getAssociatedTokenAddressSync(
        mint.publicKey,
        depositAuth.publicKey,
        false
      );
      let solVaultBeforeBalance =
        await program.provider.connection.getTokenAccountBalance(solVault);
      console.log("Before SolVault SPL Balance:", solVaultBeforeBalance);
      let depositorBeforeBalance =
        await program.provider.connection.getTokenAccountBalance(
          depositorTokenAccount
        );
      console.log("Before Depositor SPL Balance:", depositorBeforeBalance);
      let sig = await program.methods
        .withdrawSpl(new BN(25e6))
        .accounts({
          depositAccount: depositAccount.publicKey,
          pdaAuth,
          solVault,
          tokenMint: mint.publicKey,
          depositAuth: depositAuth.publicKey,
          depositorTokenAccount,
        })
        .signers([depositAuth])
        .rpc();
      console.log(
        "SPL Token Deposit:",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      let solVaultAfterBalance =
        await program.provider.connection.getTokenAccountBalance(solVault);
      console.log("After SolVault SPL Balance:", solVaultAfterBalance);
      let depositorAfterBalance =
        await program.provider.connection.getTokenAccountBalance(
          depositorTokenAccount
        );
      console.log("After Depositor SPL Balance:", depositorAfterBalance);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Mint NFT & Stake", async () => {
    try {
      const metaplex = Metaplex.make(program.provider.connection);
      const mint = anchor.web3.Keypair.generate();
      const vaultTokenAccount = getAssociatedTokenAddressSync(
        mint.publicKey,
        pdaAuth,
        true
      );
      const metadata = metaplex
        .nfts()
        .pdas()
        .metadata({ mint: mint.publicKey });
      const masterEdition = metaplex
        .nfts()
        .pdas()
        .masterEdition({ mint: mint.publicKey });
      const tokenAccount = getAssociatedTokenAddressSync(
        mint.publicKey,
        depositAuth.publicKey
      );
      const tx = new anchor.web3.Transaction();
      const modifyComputeUnits =
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1000000,
        });
      tx.add(modifyComputeUnits);
      tx.add(
        await program.methods
          .mintNftAndStake(
            "Deposit NFT",
            "DEPOSIT",
            "https://arweave.net/JLPojTJRSCF23bB_EHwKnX8y7Ni8V8bOkZtNXle2PZI"
          )
          .accounts({
            depositAccount: depositAccount.publicKey,
            pdaAuth,
            metadata,
            mint: mint.publicKey,
            tokenAccount,
            vaultTokenAccount,
            masterEdition,
            depositAuth: depositAuth.publicKey,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          })
          .signers([depositAuth, mint])
          .transaction()
      );
      tx.feePayer = depositAuth.publicKey;
      const sig = await anchor.web3.sendAndConfirmTransaction(
        program.provider.connection,
        tx,
        [depositAuth, mint]
      );
      console.log(
        "Your transaction signature",
        getUrls(Network[program.provider.connection.rpcEndpoint], sig, "tx")
          .explorer
      );
      let vaultTokenAccountAfterBalance =
        await program.provider.connection.getTokenAccountBalance(
          vaultTokenAccount
        );
      console.log("After SolVault SPL Balance:", vaultTokenAccountAfterBalance);
    } catch (error) {
      console.error(error);
      fs.writeFileSync(
        "./.anchor/custom-logs.json",
        JSON.stringify(error?.logs)
      );
      assert.fail(error);
    }
  });
});
