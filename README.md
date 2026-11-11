# Turbin3 Q4 2026 Builders Cohort

My work for the [Turbin3](https://turbin3.org) Q4 2026 Builders Cohort. Each assignment lives in its own folder and builds on its own.

| Assignment | Folder | What it is |
|---|---|---|
| Prerequisite, Part 1: Vault program | [`pre-req-vault/`](pre-req-vault) | Anchor vault program whose `withdraw` makes a CPI into the Turbin3 registration program. Deployed to devnet at [`CSnNzT57…Z8Y1`](https://explorer.solana.com/address/CSnNzT5768yVdNbaKH2teECCuvEVovUGaxWZtr48Z8Y1?cluster=devnet). |
| SPL token and Metaplex Core NFT (all 5 tasks, including both extensions) | [`turbine-spl-nft-26/`](turbine-spl-nft-26) | "Looking Up", built around one of my photos. The LKUP SPL token with Token Metadata is minted and transferred. A Core NFT of the photo is minted, renamed by its update authority, transferred and burned for its rent. Token mint [`9o6ZZPQ7…fa7j`](https://explorer.solana.com/address/9o6ZZPQ7yA19sd7pMGsV9EGnt1XW5cRX3vY7dcN2fa7j?cluster=devnet). |
| Vault and escrow programs | [`anchor-vault-escrow/`](anchor-vault-escrow) | A SOL vault (I wrote `withdraw` and `close`) and a token escrow (I wrote `take` with an expiry check; `update` is next) in one Anchor workspace, tested with LiteSVM. |
