import {
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import wallet from "../../devnet-wallet.json";
import {
  findAssociatedTokenPda,
  getCreateAssociatedTokenIdempotentInstructionAsync,
  getTransferCheckedInstruction,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

const rpc = createSolanaRpc("https://api.devnet.solana.com");

const rpcSubscriptions = createSolanaRpcSubscriptions(
  "wss://api.devnet.solana.com",
);

const decimals = 6;
const oneToken = 1_000_000n;

// mint address logged by spl_init.ts
const mint = address("9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j");

// recipient wallet: `solana-keygen pubkey devnet-wallet-2.json`
const to = address("URBgidPaVik7T4cHNxHuaAZSJcrzBiA4ML9fsvq7SC3");

(async () => {
  try {
    const signer = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    const [fromAta] = await findAssociatedTokenPda({
      mint,
      owner: signer.address,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });
    console.log(`Your fromAta is : ${fromAta}`);

    const [toAta] = await findAssociatedTokenPda({
      mint,
      owner: to,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });
    console.log(`Your toAta is : ${toAta}`);

    // 1. the recipient has no LKUP account yet, so I create (and pay for) theirs
    const createAtaIx = await getCreateAssociatedTokenIdempotentInstructionAsync(
      {
        payer: signer,
        mint,
        owner: to,
      },
    );

    // 2. move 100 LKUP ATA -> ATA; "checked" makes the program verify the mint and decimals
    const transferTx = getTransferCheckedInstruction({
      source: fromAta,
      mint,
      destination: toAta,
      authority: signer,
      amount: 100n * oneToken,
      decimals,
    });

    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const msg = createTransactionMessage({ version: 0 });

    const msgWithPayer = setTransactionMessageFeePayerSigner(signer, msg);

    const msgWithLifetime = setTransactionMessageLifetimeUsingBlockhash(
      latestBlockhash,
      msgWithPayer,
    );

    const txMessage = appendTransactionMessageInstructions(
      [createAtaIx, transferTx],
      msgWithLifetime,
    );

    const signedTx = await signTransactionMessageWithSigners(txMessage);

    assertIsTransactionWithBlockhashLifetime(signedTx);

    const signature = getSignatureFromTransaction(signedTx);

    await sendAndConfirm(signedTx, { commitment: "confirmed" });

    console.log(`transfer txid: ${signature}`);
  } catch (error) {
    console.log(error);
  }
})();
