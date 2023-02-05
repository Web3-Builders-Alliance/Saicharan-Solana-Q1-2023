import {
  AccountLayout,
  createInitializeAccountInstruction,
  createTransferInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import BN from "bn.js";
import { connection, createAndSendV0Tx, getUrls, NETWORK } from ".";
import { ESCROW_ACCOUNT_DATA_LAYOUT, EscrowLayout } from "./layout";

export const initEscrow = async (
  initializerAccount: Keypair,
  initializerXTokenAccountPubkey: PublicKey,
  amountXTokensToSendToEscrow: number,
  initializerReceivingTokenAccountPubkey: PublicKey,
  expectedAmount: number,
  escrowProgramIdString: string
) => {
  const XTokenMintAccount = await connection.getParsedAccountInfo(
    initializerXTokenAccountPubkey
  );
  const XTokenMintAccountPubkey = new PublicKey(
    // @ts-ignore
    XTokenMintAccount.value!.data.parsed.info.mint
  );

  const tempTokenAccount = Keypair.generate();
  const createTempTokenAccountIx = SystemProgram.createAccount({
    programId: TOKEN_PROGRAM_ID,
    space: AccountLayout.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span,
      "finalized"
    ),
    fromPubkey: initializerAccount.publicKey,
    newAccountPubkey: tempTokenAccount.publicKey,
  });
  const initTempAccountIx = createInitializeAccountInstruction(
    tempTokenAccount.publicKey,
    XTokenMintAccountPubkey,
    initializerAccount.publicKey,
    TOKEN_PROGRAM_ID
  );
  const transferXTokensToTempAccIx = createTransferInstruction(
    initializerXTokenAccountPubkey,
    tempTokenAccount.publicKey,
    initializerAccount.publicKey,
    amountXTokensToSendToEscrow,
    [],
    TOKEN_PROGRAM_ID
  );

  const escrowAccount = Keypair.generate();
  const escrowProgramId = new PublicKey(escrowProgramIdString);

  const createEscrowAccountIx = SystemProgram.createAccount({
    space: ESCROW_ACCOUNT_DATA_LAYOUT.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
      ESCROW_ACCOUNT_DATA_LAYOUT.span
    ),
    fromPubkey: initializerAccount.publicKey,
    newAccountPubkey: escrowAccount.publicKey,
    programId: escrowProgramId,
  });

  const initEscrowIx = new TransactionInstruction({
    programId: escrowProgramId,
    keys: [
      {
        pubkey: initializerAccount.publicKey,
        isSigner: true,
        isWritable: false,
      },
      { pubkey: tempTokenAccount.publicKey, isSigner: false, isWritable: true },
      {
        pubkey: initializerReceivingTokenAccountPubkey,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: escrowAccount.publicKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: Buffer.from(
      Uint8Array.of(0, ...new BN(expectedAmount).toArray("le", 8))
    ),
  });

  const tx = new Transaction().add(
    createTempTokenAccountIx,
    initTempAccountIx,
    transferXTokensToTempAccIx,
    createEscrowAccountIx,
    initEscrowIx
  );
  const sig = await sendAndConfirmTransaction(connection, tx, [
    initializerAccount,
    tempTokenAccount,
    escrowAccount,
  ]);
  console.log("Signature:", getUrls(NETWORK, sig, "tx").explorer);

  await new Promise((resolve) => setTimeout(resolve, 1000));

  const encodedEscrowState = (await connection.getAccountInfo(
    escrowAccount.publicKey,
    "finalized"
  ))!.data;
  const decodedEscrowState = ESCROW_ACCOUNT_DATA_LAYOUT.decode(
    encodedEscrowState
  ) as EscrowLayout;
  return {
    escrowAccountPubkey: escrowAccount.publicKey.toBase58(),
    isInitialized: !!decodedEscrowState.isInitialized,
    initializerAccountPubkey: new PublicKey(
      decodedEscrowState.initializerPubkey
    ).toBase58(),
    XTokenTempAccountPubkey: new PublicKey(
      decodedEscrowState.initializerTempTokenAccountPubkey
    ).toBase58(),
    initializerYTokenAccount: new PublicKey(
      decodedEscrowState.initializerReceivingTokenAccountPubkey
    ).toBase58(),
    expectedAmount: new BN(
      decodedEscrowState.expectedAmount,
      10,
      "le"
    ).toNumber(),
  };
};
