

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError{
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid user storage PDA")]
    InvalidUserStoragePda,
    /// Invalid SystemProgram account
    #[error("Invalid SystemProgram account")]
    SystemProgramMismatch,
    /// Account is not initialized
    #[error("Account is not initialized")]
    NotInitialized,
}


impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}