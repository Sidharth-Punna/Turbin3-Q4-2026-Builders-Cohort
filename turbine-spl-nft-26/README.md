# Looking Up: my photo on Solana

Turbin3 Q4 2026 Builders Cohort: an SPL token and a Metaplex Core NFT on Solana **devnet**, both built around one of my own street photographs.

- **LKUP**, a fungible SPL token with Token Metadata, that I mint and transfer.
- **"Looking Up"**, a [Metaplex Core](https://www.metaplex.com/docs/smart-contracts/core) NFT of the photo. It runs through its full lifecycle: minted, renamed and re-described by the update authority, transferred to another wallet, and burned for its rent.

![Looking Up: six boys in an underpass looking up toward the light](https://gateway.irys.xyz/CKeLdY4XKR2Vub5bCDZsuvX7dLDrLsTjJoL8ruyGrW8X)

## Assignment checklist

| # | Task | Scripts | Proof on devnet |
|---|---|---|---|
| 1 | Mint and transfer my own SPL token | `spl:init` → `spl:metadata` → `spl:mint` → `spl:transfer` | [mint](https://explorer.solana.com/address/9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j?cluster=devnet), [transfer tx](https://explorer.solana.com/tx/jov9H7N4eXtSuUCaC1UR5p9XCnx6ogMijmvB7e4XP2mjp7D4TqQR9nNeA6j8tTVe15bUNPCBu8TTBBipxfGcuUq?cluster=devnet) |
| 2 | Mint an NFT using MPL Core | `nft:image` → `nft:metadata` → `nft:mint` | [mint tx](https://explorer.solana.com/tx/3JUeYzDep8NXcrfFezNGbgFfRZXDq18PSZP57jemMz9UyXDN58NUZvhjJGUAAfZawCvdevizSwSHHx656ya2fMAK?cluster=devnet) |
| 3 | Update the NFT's name and metadata as update authority | `nft:update` | [update tx](https://explorer.solana.com/tx/34Vsa8aLtqCLH2G8k7b8qdtfWUM5oPEJmgijKwaTCso7SZ5PD8vrPKhQKE1PZfZVAD913b8gohUyPGKn3TZMqy55?cluster=devnet) |
| 4 | *(extension)* Transfer NFT ownership between wallets | `nft:transfer` | [transfer tx](https://explorer.solana.com/tx/21GxbZjiJyvkxF2RV8otDt2QhtbcFHAUsiDUqns8qkakzZefUgvLodvCJq4EZrGMxmCKJpGJgpx5uNhMwkv1Aw1Z?cluster=devnet) |
| 5 | *(extension)* Burn the NFT and reclaim rent | `nft:burn` | [burn tx](https://explorer.solana.com/tx/2DuPMXCjd9psopmLkG3176zZJiLVhrCCzP54qoRuuRXCuvVTAJ87Nw1vLEtjC8rxzUFzVwhkRUdH4jNMNHo6sKYN?cluster=devnet) |

---

## Setup

### 1. Add your wallets

Place your devnet keypair at the project root. It's a JSON array of numbers, e.g. `[174, 23, ...]`. The transfer steps also need a second wallet to receive the tokens and the NFT:

```bash
solana-keygen new --no-bip39-passphrase -o devnet-wallet-2.json
```

```
root/
├── devnet-wallet.json     ← main wallet (payer, mint authority, NFT update authority)
└── devnet-wallet-2.json   ← receives LKUP and the NFT, signs nft:burn
```

Both files are gitignored. Fund the main wallet with devnet SOL (`solana airdrop 2 --url devnet`). The second wallet needs no SOL, because the main wallet pays every fee and every account it receives.

### 2. Install dependencies

```bash
npm install
```

### 3. Add your image

Place the image at the project root as `photo.webp`. It's gitignored, and only the Irys copy is public. Run `npm run nft:image` first: the image URI it logs is used by both the NFT metadata and the token metadata, which shows it as the token icon. To use another format, change the filename and `contentType` in `nft_image.ts` and the `type` in the metadata.

The scripts use the public devnet RPC (`https://api.devnet.solana.com`). The NFT scripts also read `SOLANA_RPC_URL` if you set it.

> Background reading:
> - [Solana token docs](https://solana.com/docs/tokens): mint accounts, token accounts, and ATAs
> - [Solana Kit](https://www.solanakit.com/): the JS SDK used for the SPL scripts
> - [Metaplex Token Metadata](https://www.metaplex.com/docs/smart-contracts/token-metadata): attaching metadata to SPL tokens
> - [Metaplex Core](https://www.metaplex.com/docs/smart-contracts/core): the NFT standard used in the NFT scripts

---

## SPL token: Looking Up (LKUP)

Uses **@solana/kit** with **@solana-program/token** and **@solana-program/system** for transactions, and **mpl-token-metadata** via UMI for the token's name, symbol and icon.

| Script | Command | What it does |
|---|---|---|
| `spl_init.ts` | `npm run spl:init` | Creates a rent-exempt account owned by the Token Program, then initializes it as a mint: 6 decimals, my wallet as mint authority, no freeze authority |
| `spl_metadata.ts` | `npm run spl:metadata` | Uploads the token JSON (name, symbol, description, photo as icon) to Irys, then creates the Token Metadata account, signed by the mint authority |
| `spl_mint.ts` | `npm run spl:mint` | Creates my associated token account (ATA) if needed and mints 1,000 LKUP into it |
| `spl_transfer.ts` | `npm run spl:transfer` | Creates the recipient's ATA if needed and sends 100 LKUP with `transferChecked` |

Run them in order. Each script has the address from the previous step pasted in as a constant.

### What happened on devnet

Mint: [`9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j`](https://explorer.solana.com/address/9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j?cluster=devnet)

| Step | Result | Transaction |
|---|---|---|
| Create mint | 6 decimals, mint authority `23MU5p…`, supply 0 | [`5zMTXoAQ…`](https://explorer.solana.com/tx/5zMTXoAQEYmoykamcj1ip4znvXQA4aFfYXQsu4czHqKSuwJx1KJTehjnuKUdhK6JjsBeFj1rBFofGd6675u2d2jk?cluster=devnet) |
| Attach metadata | Metadata PDA [`Hz6PG1pY…`](https://explorer.solana.com/address/Hz6PG1pYD7vswyY4HByDW7o9XpQAmUvcvJKVCPtKWYqV?cluster=devnet): "Looking Up" / LKUP, JSON at [`DR2WUgJu…`](https://gateway.irys.xyz/DR2WUgJuPNLqaKcGi3wDmgGFZm9JqAe7jvSi3zoc4xpy). Token standard `Fungible` | [`5Y84Ae2c…`](https://explorer.solana.com/tx/5Y84Ae2cSTwvw3ds1YezDycgxb9fkUxiba21ipEPVB46ezXtErpWdhHgvHbwpApfuqhseTxP6AMmfVe3VxWFPLAc?cluster=devnet) |
| Mint 1,000 LKUP | My ATA [`8qMTNyxP…`](https://explorer.solana.com/address/8qMTNyxPjqN6UwHJpE4fh2ibKjL46kg9e6tS9WzbWkA4?cluster=devnet) holds 1,000 | [`2A81LNNu…`](https://explorer.solana.com/tx/2A81LNNuoC8Z8FrcyBXpPUcnXdFfbRUtjUQgkKY1Q8t5RAcdfVbHjSY9f4T4dwmg5Q58KgYtRKYAfwuK995xHkss?cluster=devnet) |
| Transfer 100 LKUP | My ATA: 900, second wallet's ATA [`FXXwnigm…`](https://explorer.solana.com/address/FXXwnigmP6Vf9ySuPYkb9jWcFBUmqkhLnCh9hGuMPQuT?cluster=devnet): 100 | [`jov9H7N4…`](https://explorer.solana.com/tx/jov9H7N4eXtSuUCaC1UR5p9XCnx6ogMijmvB7e4XP2mjp7D4TqQR9nNeA6j8tTVe15bUNPCBu8TTBBipxfGcuUq?cluster=devnet) |

### Where the SOL went

These figures come from each transaction's pre/post balances. Devnet rent at the time was 5,080 lamports per byte plus a 128-byte account overhead.

| Step | Main wallet | Breakdown |
|---|---|---|
| Create mint | −0.0010768 SOL | 0.0010668 rent (82-byte mint) + 0.00001 tx fee (2 signers: wallet + new mint keypair) |
| Attach metadata | −0.0137388 SOL | 0.0037338 rent (607-byte metadata account) + **0.01 Token Metadata protocol fee** + 0.000005 tx fee |
| Mint 1,000 | −0.00149344 SOL | 0.00148844 rent (165-byte token account) + 0.000005 tx fee |
| Transfer 100 | −0.00149344 SOL | 0.00148844 rent for **the recipient's** ATA, paid by the sender + 0.000005 tx fee |

### What I learned

- **Wallets don't hold tokens; token accounts do.** The mint stores supply, decimals and authorities. Every holder needs their own 165-byte token account per mint, and the *associated* token account is a PDA derived from (owner, mint), so anyone can compute where to send.
- **Sending to a new holder costs more than the fee.** The recipient had no LKUP account, so the sender paid ~0.0015 SOL rent to create it. Using the *idempotent* create instruction makes the scripts safe to re-run.
- **Amounts are integers.** With 6 decimals, "100 LKUP" is `100_000_000` base units. `transferChecked` also passes the mint and decimals, so the program rejects a transfer built against the wrong mint or decimal count.
- **Name and symbol aren't part of SPL Token.** They live in a separate Token Metadata account (a PDA of the mint), and only the mint authority can create it. That one step cost more than everything else in this repo combined because of its 0.01 SOL protocol fee.

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

> Solana Explorer shows Core instructions as "Unknown Program". It has no built-in parser for Metaplex Core, and Core is not an Anchor program with an on-chain IDL. Optional accounts that weren't passed (collection, owner, update authority…) show up as the Core program's own address, which is the "none" placeholder.

### Where the SOL went

| Step | Main wallet | Breakdown |
|---|---|---|
| Mint | −0.00293748 SOL | 0.00142748 rent (153-byte asset) + **0.0015 Core protocol fee** + 0.00001 tx fee (2 signers) |
| Update | −0.00001008 SOL | 0.00000508 extra rent ("Looking Up" is 1 byte longer than "Underpass") + 0.000005 tx fee |
| Transfer | −0.000005 SOL | tx fee only, since the owner field is rewritten in place |
| Burn | **+0.00076724 SOL** | 0.00077724 rent reclaimed − 0.00001 tx fee (2 signers) |

After the burn the asset account still holds 0.00215532 SOL. That's 0.00065532 of rent for the 1-byte tombstone plus the 0.0015 protocol fee from minting, which a burn doesn't refund.

### What I learned

- **A Core NFT is one account.** A Token Metadata NFT needs a mint, a token account, a metadata account and a master edition. The LKUP token above already needed three accounts. A Core asset stores the owner, update authority, name and URI in a single account. Transfer is one instruction that rewrites the owner field, with no associated token accounts to create.
- **Owner and update authority are separate roles.** After the transfer the second wallet owned the NFT, but the main wallet was still the update authority and could have kept editing its metadata. Only the owner, or a delegate set up through a plugin, can transfer or burn it.
- **Updating means new JSON plus an on-chain pointer.** The image and JSON on Irys can't be edited. "Updating metadata" means uploading a new JSON and having the update authority swap the on-chain `uri`, and the `name` too if it changes. If the name gets longer, the account is resized and the extra rent is paid.
- **A burn doesn't fully close the account.** Core leaves a 1-byte tombstone so the address can never be reused for a new asset. The rest of the rent goes to the transaction **payer**, not the owner who signed. The 0.0015 SOL creation fee isn't refundable.
