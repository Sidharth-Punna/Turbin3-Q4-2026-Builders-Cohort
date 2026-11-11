# anchor-vault-escrow

Two Anchor programs from the Turbin3 Q4 2026 Builders Cohort, in one workspace: a SOL vault and a token escrow.

## Vault

Every user gets their own vault, made of two PDAs seeded with their key:

- `vault_state` (`["state", user]`) stores the bumps.
- `vault` (`["vault", user]`) is a system account that holds the SOL.

| Instruction | What it does |
|---|---|
| `initialize` | Creates `vault_state` and funds `vault` with its rent-exempt minimum (starter) |
| `deposit(amount)` | Moves SOL from the user into the vault (starter) |
| `withdraw(amount)` | Moves SOL back to the user, but never below the rent-exempt minimum |
| `close` | Sends everything left to the user and closes `vault_state` |

The vault is owned by the System Program, so `withdraw` and `close` move SOL with a System Program transfer that the program signs with the vault's seeds. The seeds include the user's key, so nobody can touch someone else's vault.

### Tests

[`test_initialize.rs`](programs/q3_26_vault/tests/test_initialize.rs) takes one vault through initialize, a 0.5 SOL deposit, a 0.1 SOL withdraw and close in LiteSVM, and checks the balances after each step.

![vault tests passing](docs/vault-tests.png)

## Escrow

The maker locks token A in a vault owned by the escrow PDA (`["escrow", maker, seed]`) and asks for an amount of token B. A taker sends that token B and gets the token A in the same transaction. Until then, the maker can take their tokens back.

| Instruction | What it does |
|---|---|
| `make(seed, deposit, receive, expiration)` | Creates the escrow and its vault and deposits token A (starter) |
| `refund` | Returns token A to the maker and closes the vault and escrow (starter) |
| `take` | Fails once the clock reaches `expiration`. Otherwise swaps the tokens and closes the vault and escrow, with the rent going back to the maker. |
| `update` | Not written yet |

### Tests

[`tests/mod.rs`](programs/escrowq32026/tests/mod.rs):

- `test_make_and_refund` (starter)
- `test_make_and_take`: both sides get their tokens, the vault and escrow are closed, and the maker gets the rent back
- `test_take_after_expiration_fails`: moves the clock to the expiry and checks that `take` fails with `EscrowExpired` and no tokens move

![escrow tests passing](docs/escrow-tests.png)

The diagrams in [`docs/escrow`](docs/escrow) come from the starter.

## Running it

Needs Rust 1.89, the Solana CLI and Anchor 1.1.2.

```bash
anchor build
cargo test
```

`cargo test -p q3_26_vault` or `cargo test -p escrowq32026` runs one program's tests.

Program IDs: vault `7FVNbGegQQGivf5T6Fv6d15p3SyNxWdz8NNLScrSUi5q`, escrow `Dx7f9RCcCJaLTjwVExhD5FooxCw9rP24CoUbXvCkMdUG`. The keypairs aren't committed, so on a fresh clone run `anchor keys sync` after the first `anchor build`, then build again.
