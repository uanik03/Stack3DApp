#![cfg(feature = "test-bpf")]
use borsh::{BorshDeserialize, BorshSerialize};
use std::{assert_eq, println, vec::Vec};

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{
    instruction::AccountMeta, signature::Keypair, signature::Signer, system_transaction,
    transaction::Transaction,
};
use solana_validator::test_validator::TestValidatorGenesis;
use stakingapp::{instruction::Instruction as StakingInstruction, state::PoolStorageAccount};

#[test]
fn initialize_pool() {
    solana_logger::setup_with_default("solana_program_runtime=debug");

    let program_id = Pubkey::new_unique();
    println!("Program ID: {}", program_id);

    // start testing env
    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("stakingapp", program_id)
        .start();

    let rpc_client = test_validator.get_rpc_client();

    let pool_authority = Keypair::new();
    println!("Pool Authority: {}", pool_authority.pubkey());

    let pool_storage_account = Keypair::new();
    println!("Pool Storage Account: {}", pool_storage_account.pubkey());

    {
        const ANI_INIT_BALANCE: u64 = 1_000_000_000;

        //Airdrop 10 sol to authority
        let airdrop_pool_owner_tx = system_transaction::transfer(
            &payer,
            &pool_authority.pubkey(),
            ANI_INIT_BALANCE,
            rpc_client.get_latest_blockhash().unwrap(),
        );
        rpc_client
            .send_and_confirm_transaction(&airdrop_pool_owner_tx)
            .unwrap();

        const POOL_STORAGE_TOTAL_BYTES: usize = 32 + 8 + 8 + 8 + 1; // https://www.anchor-lang.com/docs/space
        let rent_exempt_balance = rpc_client
            .get_minimum_balance_for_rent_exemption(POOL_STORAGE_TOTAL_BYTES)
            .unwrap();

        // Create a pool storage account owned by program_id
        // Allocate 57 bytes of the storage
        // Transfer enough SOL from pool authority to rent exempt 57 bytes
        let create_pool_storage_account_tx = system_transaction::create_account(
            &pool_authority,
            &pool_storage_account,
            rpc_client.get_latest_blockhash().unwrap(),
            rent_exempt_balance,
            POOL_STORAGE_TOTAL_BYTES as u64,
            &program_id,
        );
        rpc_client
            .send_and_confirm_transaction(&create_pool_storage_account_tx)
            .unwrap();

        // Fetch the pool storage account and verify that everything is created correctly
        let account = rpc_client
            .get_account(&pool_storage_account.pubkey())
            .unwrap();
        println!("{:#?}", &account);

        assert_eq!(account.owner, program_id);
        assert_eq!(account.lamports, rent_exempt_balance);
        assert_eq!(account.data.len(), POOL_STORAGE_TOTAL_BYTES);
    }

    let initialize_ix = StakingInstruction::Initialize {
        rewards_per_token: 42,
        };
    let mut instruction_data: Vec<u8> = vec![];
       
     // Serialize instruction into bytes that would be given as instruction_data to the entrypoint!
    initialize_ix.serialize(&mut instruction_data).unwrap();


    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(pool_authority.pubkey(), true),
                AccountMeta::new(pool_storage_account.pubkey(), false),
            ],
            // Borsh-packed Instruction::Initialize
            data: instruction_data,
        }],
        // Signer of the transaction
        Some(&pool_authority.pubkey()),
    );
    transaction.sign(
        &[&pool_authority],
        rpc_client.get_latest_blockhash().unwrap(),
        );
        rpc_client
        .send_and_confirm_transaction(&transaction)
        .unwrap();

        let account_data = rpc_client
        .get_account_data(&pool_storage_account.pubkey())
        .unwrap();
    let pool_storage = PoolStorageAccount::try_from_slice(&account_data).unwrap();
    println!("pool_storage {:#?}", pool_storage);

    assert_eq!(pool_storage.pool_authority, pool_authority.pubkey());
    assert_eq!(pool_storage.total_staked, 0);
    assert_eq!(pool_storage.user_count, 0);
    assert_eq!(pool_storage.rewards_per_token, 42);
    assert!(pool_storage.is_initialized);
}       
