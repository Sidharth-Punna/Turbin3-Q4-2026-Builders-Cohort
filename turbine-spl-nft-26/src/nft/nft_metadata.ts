import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import wallet from "../../devnet-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(
  irysUploader({
    address: "https://devnet.irys.xyz/",
  }),
);

umi.use(signerIdentity(signer));

(async () => {
  try {
    // image uri logged by nft_image.ts
    const image =
      "https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X";

    // json schema: https://www.metaplex.com/docs/smart-contracts/core/json-schema
    // minted under a working title; nft_update.ts swaps in the final title and metadata
    const metadata = {
      name: "Underpass",
      description: "Black-and-white street photograph.",
      image,
      attributes: [
        { trait_type: "Medium", value: "Photography" },
        { trait_type: "Color", value: "Black & White" },
      ],
      properties: {
        files: [{ uri: image, type: "image/webp" }],
        category: "image",
      },
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log(`metadata uri: ${myUri} `);
  } catch (error) {
    console.log("error", error);
  }
})();
