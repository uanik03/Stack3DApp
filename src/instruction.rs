use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Initialize{
        rewards_per_token: u64
    },
    CreateUser{

    },
    Stake{
        amount: u64
    },
    Unstake{
        amount: u64
    },
    Claim{
        
    }
}
