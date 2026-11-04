import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../../devnet-wallet.json";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { fetchAsset, mplCore, update } from "@metaplex-foundation/mpl-core";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { base58 } from "@metaplex-foundation/umi/serializers";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

// this wallet must be the asset's update authority, or the update is rejected
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));
umi.use(mplCore());
umi.use(
  irysUploader({
    address: "https://devnet.irys.xyz/",
  }),
);

// asset address logged by nft_mint.ts
const assetAddress = publicKey("DFWRWMAHUoJvj6hXv4LjwcqwDDcXy6QD4awLPff1NHjx");

// image uri logged by nft_image.ts (the photo itself doesn't change)
const image =
  "https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X";

const name = "Looking Up";

(async () => {
  try {
    // 1. upload the new off-chain metadata JSON
    const metadata = {
      name,
      description:
        "Six boys in a concrete underpass, all looking up toward the light. Black-and-white street photograph by Sidharth Punna.",
      image,
      attributes: [
        { trait_type: "Medium", value: "Photography" },
        { trait_type: "Color", value: "Black & White" },
        { trait_type: "Genre", value: "Street" },
        { trait_type: "Setting", value: "Underpass" },
        { trait_type: "Subjects", value: "6" },
        { trait_type: "Photographer", value: "Sidharth Punna" },
        { trait_type: "Edition", value: "1/1" },
      ],
      properties: {
        files: [{ uri: image, type: "image/webp" }],
        category: "image",
      },
    };
    const metadataUri = await umi.uploader.uploadJson(metadata);
    console.log(`new metadata uri: ${metadataUri}`);

    // 2. point the on-chain asset at it and rename it
    const asset = await fetchAsset(umi, assetAddress);
    console.log(`before: "${asset.name}" -> ${asset.uri}`);

    const tx = await update(umi, {
      asset,
      name,
      uri: metadataUri,
    }).sendAndConfirm(umi);

    const signature = base58.deserialize(tx.signature)[0];

    console.log(`after:  "${name}" -> ${metadataUri}`);
    console.log(`signature ${signature} , asset : ${asset.publicKey}`);
  } catch (e) {
    console.log(`error ${e}`);
  }
})();
