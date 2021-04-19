//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, msg, program_error::PrintProgramError, program_error::ProgramError,
};

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, thiserror::Error, FromPrimitive, PartialEq)]
pub enum Error {
    #[error("Cannot execute instruction")]
    Failed,
    #[error("Only owner can add remove admins")]
    OnlyOwnerCanAddRemoveAdmins,

    #[error("Overflow")]
    Overflow,

    #[error("Invalid derived address")]
    InvalidDerivedAddress,

    #[error("Invalid derived dweller server address")]
    InvalidDerivedDwellerServerAddress,

    #[error("Invalid derived server member status address")]
    InvalidDerivedServerMemberStatusAddress,

    #[error("Invalid derived server member address")]
    InvalidDerivedServerMemberAddress,

    #[error("Invalid derived server group address")]
    InvalidDerivedServerGroupAddress,

    #[error("Invalid derived server channel address")]
    InvalidDerivedServerChannelAddress,

    #[error("Invalid derived group channel address")]
    InvalidDerivedGroupChannelAddress,

    #[error("Invalid derived server administrator address")]
    InvalidDerivedServerAdministratorAddress,

    #[error("Provided dweller is not the owner of the server")]
    ProvidedDwellerIsNotTheOwnerOfTheServer,

    #[error("Invalid derived address wrong server")]
    InvalidDerivedAddressWrongServer,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Satellite Server"
    }
}

impl PrintProgramError for Error {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string())
    }
}
