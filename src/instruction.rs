use crate::{error::MailError::InvalidInstruction, state::Mail};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize)]
pub enum MailInstruction {
    InitAccount,
    SendMail { mail: Mail },
}

impl MailInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitAccount,
            1 => Self::SendMail {
                mail: Mail::try_from_slice(&rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
