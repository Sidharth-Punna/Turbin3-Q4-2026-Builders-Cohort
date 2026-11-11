use {
    anchor_lang::{
        prelude::msg,
        solana_program::clock::Clock,
        solana_program::instruction::{error::InstructionError, Instruction},
        solana_program::program_pack::Pack,
        system_program::ID as SYSTEM_PROGRAM_ID,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    anchor_spl::{
        associated_token::{self, ID as ASSOCIATED_TOKEN_PROGRAM_ID},
        token::spl_token,
    },
    escrowq32026::error::ErrorCode,
    litesvm::types::TransactionResult,
    litesvm::LiteSVM,
    litesvm_token::{
        spl_token::ID as TOKEN_PROGRAM_ID, CreateAssociatedTokenAccount, CreateMint, MintTo,
    },
    solana_keypair::Keypair,
    solana_message::Message,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::{Transaction, TransactionError},
};

// Setup function to initialize LiteSVM and create a payer keypair
fn setup() -> (LiteSVM, Keypair) {
    let program_id = escrowq32026::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/escrowq32026.so"
    ));
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // Return the LiteSVM instance and payer keypair
    (svm, payer)
}

#[test]
fn test_make_and_refund() {
    // Setup the test environment by initializing LiteSVM and creating a payer keypair
    let (mut program, payer) = setup();

    // Get the maker's public key from the payer keypair
    let maker = payer.pubkey();

    // Create two mints (Mint A and Mint B) with 6 decimal places and the maker as the authority
    // This done using litesvm-token's CreateMint utility which creates the mint in the LiteSVM environment
    let mint_a = CreateMint::new(&mut program, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();
    msg!("Mint A: {}\n", mint_a);

    let mint_b = CreateMint::new(&mut program, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();
    msg!("Mint B: {}\n", mint_b);

    // Create the maker's associated token account for Mint A
    // This is done using litesvm-token's CreateAssociatedTokenAccount utility
    let maker_ata_a = CreateAssociatedTokenAccount::new(&mut program, &payer, &mint_a)
        .owner(&maker)
        .send()
        .unwrap();
    msg!("Maker ATA A: {}\n", maker_ata_a);

    // Derive the PDA for the escrow account using the maker's public key and a seed value
    let escrow = Pubkey::find_program_address(
        &[b"escrow", maker.as_ref(), &123u64.to_le_bytes()],
        &escrowq32026::id(),
    )
    .0;
    msg!("Escrow PDA: {}\n", escrow);

    // Derive the PDA for the vault associated token account using the escrow PDA and Mint A
    let vault = associated_token::get_associated_token_address(&escrow, &mint_a);
    msg!("Vault PDA: {}\n", vault);

    // Mint 1,000 tokens (with 6 decimal places) of Mint A to the maker's associated token account
    MintTo::new(&mut program, &payer, &mint_a, &maker_ata_a, 1000_000_000)
        .send()
        .unwrap();

    // Create the "Make" instruction to deposit tokens into the escrow
    let make_ix = Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Make {
            maker: maker,
            mint_a: mint_a,
            mint_b: mint_b,
            maker_ata_a: maker_ata_a,
            escrow: escrow,
            vault: vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Make {
            deposit: 10_000_000,
            seed: 123u64,
            receive: 10_000_000,
            expiration: 17780206209,
        }
        .data(),
    };

    // Create and send the transaction containing the "Make" instruction
    let message = Message::new(&[make_ix], Some(&payer.pubkey()));
    let recent_blockhash = program.latest_blockhash();

    let transaction = Transaction::new(&[&payer], message, recent_blockhash);

    // Send the transaction and capture the result
    let tx = program.send_transaction(transaction).unwrap();

    // Log transaction details
    msg!("\n\nMake transaction sucessfull");
    msg!("CUs Consumed: {}", tx.compute_units_consumed);
    msg!("Tx Signature: {}", tx.signature);

    // Verify the vault account and escrow account data after the "Make" instruction
    let vault_account = program.get_account(&vault).unwrap();
    let vault_data = spl_token::state::Account::unpack(&vault_account.data).unwrap();
    assert_eq!(vault_data.amount, 10_000_000);
    assert_eq!(vault_data.owner, escrow);
    assert_eq!(vault_data.mint, mint_a);

    let escrow_account = program.get_account(&escrow).unwrap();
    let escrow_data =
        escrowq32026::state::Escrow::try_deserialize(&mut escrow_account.data.as_ref()).unwrap();
    assert_eq!(escrow_data.seed, 123u64);
    assert_eq!(escrow_data.maker, maker);
    assert_eq!(escrow_data.mint_a, mint_a);
    assert_eq!(escrow_data.mint_b, mint_b);
    assert_eq!(escrow_data.receive, 10_000_000);

    set_time(&mut program, escrow_data.expiration);

    // Create the "Refund" instruction to refund tokens back to the maker
    let refund_ix = Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Refund {
            maker: maker,
            mint_a: mint_a,
            maker_ata_a: maker_ata_a,
            escrow: escrow,
            vault: vault,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Refund {}.data(),
    };

    // Create and send the transaction containing the "Refund" instruction
    let message = Message::new(&[refund_ix], Some(&payer.pubkey()));
    let recent_blockhash = program.latest_blockhash();

    let transaction = Transaction::new(&[&payer], message, recent_blockhash);

    // Send the transaction and capture the result
    let tx = program.send_transaction(transaction).unwrap();

    // Log transaction details
    msg!("\n\nRefund transaction sucessful");
    msg!("CUs Consumed: {}", tx.compute_units_consumed);
    msg!("Tx Signature: {}", tx.signature);
    assert!(program.get_account(&escrow).is_none());
    assert!(program.get_account(&vault).is_none());
}

const OFFER_SEED: u64 = 7;
const OFFERED_A: u64 = 10_000_000;
const ASKED_B: u64 = 25_000_000;
const TAKER_B: u64 = 100_000_000;
const WINDOW: i64 = 3_600;

struct Offer {
    maker: Keypair,
    taker: Keypair,
    mint_a: Pubkey,
    mint_b: Pubkey,
    maker_ata_a: Pubkey,
    escrow: Pubkey,
    vault: Pubkey,
    taker_ata_b: Pubkey,
}

#[allow(clippy::result_large_err)]
fn send(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction) -> TransactionResult {
    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let transaction = Transaction::new(&[payer], message, svm.latest_blockhash());
    svm.send_transaction(transaction)
}

fn token_amount(svm: &LiteSVM, token_account: &Pubkey) -> u64 {
    let account = svm.get_account(token_account).unwrap();
    spl_token::state::Account::unpack(&account.data)
        .unwrap()
        .amount
}

fn now(svm: &LiteSVM) -> i64 {
    svm.get_sysvar::<Clock>().unix_timestamp
}

fn set_time(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

fn program_error(code: ErrorCode) -> TransactionError {
    TransactionError::InstructionError(0, InstructionError::Custom(code.into()))
}

fn new_offer(svm: &mut LiteSVM, maker: Keypair) -> Offer {
    let mint_a = CreateMint::new(svm, &maker)
        .decimals(6)
        .authority(&maker.pubkey())
        .send()
        .unwrap();
    let mint_b = CreateMint::new(svm, &maker)
        .decimals(6)
        .authority(&maker.pubkey())
        .send()
        .unwrap();

    let maker_ata_a = CreateAssociatedTokenAccount::new(svm, &maker, &mint_a)
        .owner(&maker.pubkey())
        .send()
        .unwrap();
    MintTo::new(svm, &maker, &mint_a, &maker_ata_a, OFFERED_A)
        .send()
        .unwrap();

    let escrow = Pubkey::find_program_address(
        &[
            b"escrow",
            maker.pubkey().as_ref(),
            &OFFER_SEED.to_le_bytes(),
        ],
        &escrowq32026::id(),
    )
    .0;
    let vault = associated_token::get_associated_token_address(&escrow, &mint_a);

    let taker = Keypair::new();
    svm.airdrop(&taker.pubkey(), 1_000_000_000).unwrap();
    let taker_ata_b = CreateAssociatedTokenAccount::new(svm, &taker, &mint_b)
        .owner(&taker.pubkey())
        .send()
        .unwrap();
    MintTo::new(svm, &maker, &mint_b, &taker_ata_b, TAKER_B)
        .send()
        .unwrap();

    Offer {
        maker,
        taker,
        mint_a,
        mint_b,
        maker_ata_a,
        escrow,
        vault,
        taker_ata_b,
    }
}

fn make_ix(offer: &Offer, expiration: i64) -> Instruction {
    Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Make {
            maker: offer.maker.pubkey(),
            mint_a: offer.mint_a,
            mint_b: offer.mint_b,
            maker_ata_a: offer.maker_ata_a,
            escrow: offer.escrow,
            vault: offer.vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Make {
            seed: OFFER_SEED,
            deposit: OFFERED_A,
            receive: ASKED_B,
            expiration,
        }
        .data(),
    }
}

fn open_offer(svm: &mut LiteSVM, maker: Keypair, expiration: i64) -> Offer {
    let offer = new_offer(svm, maker);
    send(svm, &offer.maker, make_ix(&offer, expiration)).unwrap();
    offer
}

fn take_ix(offer: &Offer) -> Instruction {
    let taker = offer.taker.pubkey();
    let maker = offer.maker.pubkey();
    Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Take {
            taker,
            maker,
            mint_a: offer.mint_a,
            mint_b: offer.mint_b,
            taker_ata_a: associated_token::get_associated_token_address(&taker, &offer.mint_a),
            taker_ata_b: offer.taker_ata_b,
            maker_ata_b: associated_token::get_associated_token_address(&maker, &offer.mint_b),
            escrow: offer.escrow,
            vault: offer.vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Take {}.data(),
    }
}

fn refund_ix(offer: &Offer) -> Instruction {
    Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Refund {
            maker: offer.maker.pubkey(),
            mint_a: offer.mint_a,
            maker_ata_a: offer.maker_ata_a,
            escrow: offer.escrow,
            vault: offer.vault,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Refund {}.data(),
    }
}

#[test]
fn test_make_and_take() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);

    let maker = offer.maker.pubkey();
    let taker_ata_a =
        associated_token::get_associated_token_address(&offer.taker.pubkey(), &offer.mint_a);
    let maker_ata_b = associated_token::get_associated_token_address(&maker, &offer.mint_b);
    assert!(svm.get_account(&taker_ata_a).is_none());
    assert!(svm.get_account(&maker_ata_b).is_none());

    let maker_lamports = svm.get_balance(&maker).unwrap();
    let rent_returned =
        svm.get_balance(&offer.escrow).unwrap() + svm.get_balance(&offer.vault).unwrap();

    set_time(&mut svm, expiration - 1);
    let tx = send(&mut svm, &offer.taker, take_ix(&offer)).unwrap();
    msg!("\n\nTake transaction successful");
    msg!("CUs Consumed: {}", tx.compute_units_consumed);

    assert_eq!(token_amount(&svm, &taker_ata_a), OFFERED_A);
    assert_eq!(token_amount(&svm, &maker_ata_b), ASKED_B);
    assert_eq!(token_amount(&svm, &offer.taker_ata_b), TAKER_B - ASKED_B);

    assert!(svm.get_account(&offer.escrow).is_none());
    assert!(svm.get_account(&offer.vault).is_none());
    assert_eq!(
        svm.get_balance(&maker).unwrap(),
        maker_lamports + rent_returned
    );
}

#[test]
fn test_make_with_past_expiration_fails() {
    let (mut svm, maker) = setup();
    let offer = new_offer(&mut svm, maker);
    let expiration = now(&svm);

    let failed = send(&mut svm, &offer.maker, make_ix(&offer, expiration)).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::ExpirationInPast));

    assert!(svm.get_account(&offer.escrow).is_none());
    assert_eq!(token_amount(&svm, &offer.maker_ata_a), OFFERED_A);
}

#[test]
fn test_take_after_expiration_fails() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);

    set_time(&mut svm, expiration);
    let failed = send(&mut svm, &offer.taker, take_ix(&offer)).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::EscrowExpired));

    assert_eq!(token_amount(&svm, &offer.vault), OFFERED_A);
    assert_eq!(token_amount(&svm, &offer.taker_ata_b), TAKER_B);
    assert!(svm.get_account(&offer.escrow).is_some());
}

#[test]
fn test_refund_only_after_expiration() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);

    set_time(&mut svm, expiration - 1);
    let failed = send(&mut svm, &offer.maker, refund_ix(&offer)).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::EscrowNotExpired));
    assert_eq!(token_amount(&svm, &offer.vault), OFFERED_A);

    set_time(&mut svm, expiration);
    svm.expire_blockhash();
    send(&mut svm, &offer.maker, refund_ix(&offer)).unwrap();

    assert_eq!(token_amount(&svm, &offer.maker_ata_a), OFFERED_A);
    assert!(svm.get_account(&offer.escrow).is_none());
    assert!(svm.get_account(&offer.vault).is_none());
}

fn update_ix(offer: &Offer, signer: Pubkey, receive: u64, expiration: i64) -> Instruction {
    Instruction {
        program_id: escrowq32026::id(),
        accounts: escrowq32026::accounts::Update {
            maker: signer,
            escrow: offer.escrow,
        }
        .to_account_metas(None),
        data: escrowq32026::instruction::Update {
            receive,
            expiration,
        }
        .data(),
    }
}

fn escrow_state(svm: &LiteSVM, offer: &Offer) -> escrowq32026::state::Escrow {
    let account = svm.get_account(&offer.escrow).unwrap();
    escrowq32026::state::Escrow::try_deserialize(&mut account.data.as_ref()).unwrap()
}

#[test]
fn test_update_then_take() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);
    let new_receive = ASKED_B - 5_000_000;
    let new_expiration = expiration + WINDOW;

    let update = update_ix(&offer, offer.maker.pubkey(), new_receive, new_expiration);
    send(&mut svm, &offer.maker, update).unwrap();
    let escrow = escrow_state(&svm, &offer);
    assert_eq!(escrow.receive, new_receive);
    assert_eq!(escrow.expiration, new_expiration);

    set_time(&mut svm, expiration);
    send(&mut svm, &offer.taker, take_ix(&offer)).unwrap();

    let maker_ata_b =
        associated_token::get_associated_token_address(&offer.maker.pubkey(), &offer.mint_b);
    assert_eq!(token_amount(&svm, &maker_ata_b), new_receive);
    assert_eq!(
        token_amount(&svm, &offer.taker_ata_b),
        TAKER_B - new_receive
    );
}

#[test]
fn test_update_cannot_worsen_terms() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);
    let maker = offer.maker.pubkey();

    let raise = update_ix(&offer, maker, ASKED_B + 1, expiration);
    let failed = send(&mut svm, &offer.maker, raise).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::ReceiveRaised));

    let shorten = update_ix(&offer, maker, ASKED_B, expiration - 1);
    let failed = send(&mut svm, &offer.maker, shorten).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::ExpirationShortened));

    let escrow = escrow_state(&svm, &offer);
    assert_eq!(escrow.receive, ASKED_B);
    assert_eq!(escrow.expiration, expiration);
}

#[test]
fn test_update_after_expiration_fails() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);

    set_time(&mut svm, expiration);
    let update = update_ix(&offer, offer.maker.pubkey(), ASKED_B, expiration + WINDOW);
    let failed = send(&mut svm, &offer.maker, update).unwrap_err();
    assert_eq!(failed.err, program_error(ErrorCode::EscrowExpired));

    assert_eq!(escrow_state(&svm, &offer).expiration, expiration);
}

#[test]
fn test_update_by_someone_else_fails() {
    let (mut svm, maker) = setup();
    let expiration = now(&svm) + WINDOW;
    let offer = open_offer(&mut svm, maker, expiration);

    let update = update_ix(&offer, offer.taker.pubkey(), 0, expiration + WINDOW);
    let failed = send(&mut svm, &offer.taker, update).unwrap_err();
    assert_eq!(
        failed.err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(anchor_lang::error::ErrorCode::ConstraintSeeds.into())
        )
    );

    let escrow = escrow_state(&svm, &offer);
    assert_eq!(escrow.receive, ASKED_B);
    assert_eq!(escrow.expiration, expiration);
}
