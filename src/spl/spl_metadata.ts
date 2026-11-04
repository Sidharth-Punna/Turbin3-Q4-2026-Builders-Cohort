import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import wallet from "../../devnet-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import bs58 from "bs58";

// mint address logged by spl_init.ts
const mint = publicKey("9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j");

// the photo uploaded by nft_image.ts doubles as the token icon
const image =
  "https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X";

const name = "Looking Up";
const symbol = "LKUP";

const umi = createUmi("https://api.devnet.solana.com");

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));
umi.use(
  irysUploader({
    address: "https://devnet.irys.xyz/",
  }),
);

(async () => {
  try {
    // 1. off-chain JSON that wallets and explorers read for the token's icon and description
    const uri = await umi.uploader.uploadJson({
      name,
      symbol,
      description:
        "Fungible token for my black-and-white street photograph \"Looking Up\".",
      image,
    });
    console.log(`token metadata uri: ${uri}`);

    // 2. Token Metadata PDA for the mint; only the mint authority can create it
    const accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
    };

    const data: DataV2Args = {
      name,
      symbol,
      uri,
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    const args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: true,
      collectionDetails: null,
    };

    const tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    const result = await tx.sendAndConfirm(umi);
    console.log("signature: ", bs58.encode(Buffer.from(result.signature)));
  } catch (error) {
    console.log("error", error);
  }
})();
