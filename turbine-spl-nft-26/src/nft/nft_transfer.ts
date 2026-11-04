import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../../devnet-wallet.json";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { fetchAsset, mplCore, transfer } from "@metaplex-foundation/mpl-core";
import { base58 } from "@metaplex-foundation/umi/serializers";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

// the current owner signs the transfer
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));
umi.use(mplCore());

// asset address logged by nft_mint.ts
const assetAddress = publicKey("DFWRWMAHUoJvj6hXv4LjwcqwDDcXy6QD4awLPff1NHjx");

// recipient wallet: `solana-keygen pubkey devnet-wallet-2.json`
const newOwner = publicKey("URBgidPaVik7T4cHNxHuaAZSJcrzBiA4ML9fsvq7SC3");

(async () => {
  try {
    const asset = await fetchAsset(umi, assetAddress);
    console.log(`owner before: ${asset.owner}`);

    // Core keeps the owner inside the asset account itself, so a transfer
    // is a single field change: no token accounts to create or close
    const tx = await transfer(umi, {
      asset,
      newOwner,
    }).sendAndConfirm(umi);

    const signature = base58.deserialize(tx.signature)[0];

    console.log(`owner after:  ${newOwner}`);
    console.log(`signature ${signature} , asset : ${asset.publicKey}`);
  } catch (e) {
    console.log(`error ${e}`);
  }
})();
