import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../../devnet-wallet.json";
import ownerWallet from "../../devnet-wallet-2.json";
import {
  createSignerFromKeypair,
  PublicKey,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { burn, fetchAsset, mplCore } from "@metaplex-foundation/mpl-core";
import { base58 } from "@metaplex-foundation/umi/serializers";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

// main wallet pays the transaction fee
const payer = createSignerFromKeypair(
  umi,
  umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet)),
);

// only the current owner (here the wallet from nft_transfer.ts) can burn
const owner = createSignerFromKeypair(
  umi,
  umi.eddsa.createKeypairFromSecretKey(new Uint8Array(ownerWallet)),
);

umi.use(signerIdentity(payer));
umi.use(mplCore());

// asset address logged by nft_mint.ts
const assetAddress = publicKey("DFWRWMAHUoJvj6hXv4LjwcqwDDcXy6QD4awLPff1NHjx");

const lamports = async (address: PublicKey) =>
  (await umi.rpc.getBalance(address, { commitment: "confirmed" })).basisPoints;

const sol = (amount: bigint) => `${Number(amount) / 1e9} SOL`;

(async () => {
  try {
    const asset = await fetchAsset(umi, assetAddress);

    const before = {
      asset: await lamports(asset.publicKey),
      payer: await lamports(payer.publicKey),
      owner: await lamports(owner.publicKey),
    };

    const tx = await burn(umi, {
      asset,
      authority: owner,
    }).sendAndConfirm(umi);

    const signature = base58.deserialize(tx.signature)[0];

    const after = {
      asset: await lamports(asset.publicKey),
      payer: await lamports(payer.publicKey),
      owner: await lamports(owner.publicKey),
    };

    console.log(`burned "${asset.name}" : ${asset.publicKey}`);
    console.log(`signature ${signature}`);
    console.log(
      `asset account: ${sol(before.asset)} -> ${sol(after.asset)} (${sol(before.asset - after.asset)} released)`,
    );
    console.log(
      `payer ${payer.publicKey}: ${sol(before.payer)} -> ${sol(after.payer)}`,
    );
    console.log(
      `owner ${owner.publicKey}: ${sol(before.owner)} -> ${sol(after.owner)}`,
    );
  } catch (e) {
    console.log(`error ${e}`);
  }
})();
