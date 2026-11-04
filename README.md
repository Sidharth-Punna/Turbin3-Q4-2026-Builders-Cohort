# Looking Up: my photo as a Solana NFT

Turbin3 Q4 2026 Builders Cohort: SPL token and Metaplex Core NFT scripts on Solana **devnet**.

The NFT part takes one of my own street photographs and runs it through the full lifecycle of a [Metaplex Core](https://www.metaplex.com/docs/smart-contracts/core) asset. It gets minted, renamed and re-described by the update authority, transferred to another wallet, and burned for its rent.

![Looking Up: six boys in an underpass looking up toward the light](https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X)

---

## Setup

### 1. Add your wallets

Place your devnet keypair at the project root. It's a JSON array of numbers, e.g. `[174, 23, ...]`. The NFT transfer and burn steps also need a second wallet to receive the NFT:

```bash
solana-keygen new --no-bip39-passphrase -o devnet-wallet-2.json
```

```
root/
├── devnet-wallet.json     ← main wallet (payer, mint owner, update authority)
└── devnet-wallet-2.json   ← receives the NFT in nft:transfer, signs nft:burn
```

Both files are gitignored. Fund the main wallet with devnet SOL (`solana airdrop 2 --url devnet`). The second wallet needs no SOL, because the main wallet pays every fee.

### 2. Install dependencies

```bash
npm install
```

### 3. Add your image

Place the image at the project root as `photo.webp`. It's gitignored, and only the Irys copy is public. To use another format, change the filename and `contentType` in `nft_image.ts` and the `type` in the metadata.

All scripts default to `https://api.devnet.solana.com`. Set `SOLANA_RPC_URL` to use a different devnet RPC.

> Background reading:
> - [Solana token docs](https://solana.com/docs/tokens): mint accounts, token accounts, and ATAs
> - [Solana Kit](https://www.solanakit.com/): the JS SDK used for the SPL scripts
> - [Metaplex Token Metadata](https://www.metaplex.com/docs/smart-contracts/token-metadata): attaching metadata to SPL tokens
> - [Metaplex Core](https://www.metaplex.com/docs/smart-contracts/core): the NFT standard used in the NFT scripts

---

## NFT: "Looking Up" (Metaplex Core)

Uses **mpl-core** via UMI. The image and metadata JSON are stored on Irys.

| Script | Command | What it does |
|---|---|---|
| `nft_image.ts` | `npm run nft:image` | Uploads `photo.webp` to Irys and logs the image URI |
| `nft_metadata.ts` | `npm run nft:metadata` | Uploads the mint-time metadata JSON (working title "Underpass") and logs its URI |
| `nft_mint.ts` | `npm run nft:mint` | Creates the Core asset with that name and URI and logs the asset address |
| `nft_update.ts` | `npm run nft:update` | As update authority: uploads new metadata, then renames the asset to "Looking Up" and points it at the new URI |
| `nft_transfer.ts` | `npm run nft:transfer` | Current owner transfers the asset to the second wallet |
| `nft_burn.ts` | `npm run nft:burn` | The new owner burns the asset, and the script logs the rent that comes back |

Run them in order. Each script has the address or URI from the previous step pasted in as a constant.

### What happened on devnet

Asset: [`DFWRWMAHUoJvj6hXv4LjwcqwDDcXy6QD4awLPff1NHjx`](https://explorer.solana.com/address/DFWRWMAHUoJvj6hXv4LjwcqwDDcXy6QD4awLPff1NHjx?cluster=devnet)

| Step | Result | Transaction |
|---|---|---|
| Upload image | [`CKeLdY4X…`](https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X) (`image/webp`) | n/a |
| Upload metadata | [`5M5sXHad…`](https://gateway.irys.xyz/5M5sXHadSmm7ic6nqQp6NRsHk1h6ngQ5oQEJLUH7p2mk): "Underpass", 2 attributes | n/a |
| Mint | "Underpass", owner and update authority `23MU5p…` | [`3JUeYzDe…`](https://explorer.solana.com/tx/3JUeYzDep8NXcrfFezNGbgFfRZXDq18PSZP57jemMz9UyXDN58NUZvhjJGUAAfZawCvdevizSwSHHx656ya2fMAK?cluster=devnet) |
| Update | Renamed to "Looking Up", URI moved to [`GzNZRGaa…`](https://gateway.irys.xyz/GzNZRGaa36Bvst9riXoF7wWPCPmiqFX4VYxWxcfqTE4S) (7 attributes) | [`34Vsa8aL…`](https://explorer.solana.com/tx/34Vsa8aLtqCLH2G8k7b8qdtfWUM5oPEJmgijKwaTCso7SZ5PD8vrPKhQKE1PZfZVAD913b8gohUyPGKn3TZMqy55?cluster=devnet) |
| Transfer | Owner `23MU5p…` → `URBgid…`, update authority unchanged | [`21GxbZji…`](https://explorer.solana.com/tx/21GxbZjiJyvkxF2RV8otDt2QhtbcFHAUsiDUqns8qkakzZefUgvLodvCJq4EZrGMxmCKJpGJgpx5uNhMwkv1Aw1Z?cluster=devnet) |
| Burn | Asset shrunk to a 1-byte tombstone, rent returned to the payer | [`2DuPMXCj…`](https://explorer.solana.com/tx/2DuPMXCjd9psopmLkG3176zZJiLVhrCCzP54qoRuuRXCuvVTAJ87Nw1vLEtjC8rxzUFzVwhkRUdH4jNMNHo6sKYN?cluster=devnet) |

### Where the SOL went

These figures come from each transaction's pre/post balances. Devnet rent at the time was 5,080 lamports per byte plus a 128-byte account overhead.

| Step | Main wallet | Breakdown |
|---|---|---|
| Mint | −0.00293748 SOL | 0.00142748 rent (153-byte asset) + **0.0015 Core protocol fee** + 0.00001 tx fee (2 signers) |
| Update | −0.00001008 SOL | 0.00000508 extra rent ("Looking Up" is 1 byte longer than "Underpass") + 0.000005 tx fee |
| Transfer | −0.000005 SOL | tx fee only, since the owner field is rewritten in place |
| Burn | **+0.00076724 SOL** | 0.00077724 rent reclaimed − 0.00001 tx fee (2 signers) |

After the burn the asset account still holds 0.00215532 SOL. That's 0.00065532 of rent for the 1-byte tombstone plus the 0.0015 protocol fee from minting, which a burn doesn't refund.

### What I learned

- **A Core NFT is one account.** Token Metadata NFTs need a mint, a token account, a metadata account and a master edition. A Core asset stores the owner, update authority, name and URI in a single account. Transfer is one instruction that rewrites the owner field, with no associated token accounts to create.
- **Owner and update authority are separate roles.** After the transfer the second wallet owned the NFT, but the main wallet was still the update authority and could have kept editing its metadata. Only the owner, or a delegate set up through a plugin, can transfer or burn it.
- **Updating means new JSON plus an on-chain pointer.** The image and JSON on Irys can't be edited. "Updating metadata" means uploading a new JSON and having the update authority swap the on-chain `uri`, and the `name` too if it changes. If the name gets longer, the account is resized and the extra rent is paid.
- **A burn doesn't fully close the account.** Core leaves a 1-byte tombstone so the address can never be reused for a new asset. The rest of the rent goes to the transaction **payer**, not the owner who signed. The 0.0015 SOL creation fee isn't refundable.

---

## SPL Token

Uses **@solana/kit** and **@solana-program/token** for transactions, and **mpl-token-metadata** via UMI for on-chain metadata.

| Script | Command | What it does |
|---|---|---|
| `spl_init.ts` | `npm run spl:init` | Creates a new mint account |
| `spl_metadata.ts` | `npm run spl:metadata` | Attaches a name, symbol, and URI to the mint |
| `spl_mint.ts` | `npm run spl:mint` | Creates your associated token account and mints tokens into it |
| `spl_transfer.ts` | `npm run spl:transfer` | Sends tokens to another wallet's associated token account |

Run them in order. Each script logs the addresses or signatures you'll need to paste into the next one.
