// //! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive)]
pub enum Instruction {
    
    /// accounts
    /// - signer, write dweller
    ///
    /// Input: [InitializeDwellerInput]
    InitializeDweller,

    /// accounts
    /// - signer, write     owner     must be dweller
    /// - signer, write     server
    ///
    /// Input: [InitializeServerInput]
    InitializeServer,

    /// Change dweller's display name
    /// Accounts:
    /// - write   dweller
    /// Input: [SetNameInput]
    SetDwellerName,

    /// Change dweller's display photo. Consider using PNG or JPEG photos for usability.
    /// Accounts:
    /// - signer, write   dweller
    /// Input: [SetHashInput]
    SetDwellerPhoto,

    /// Update the users status
    /// Accounts:
    /// - signer  owner Dweller who ows account 
    SetStatus,

    /// - signer  admin
    /// - write   channel
    /// input:
    /// - type_id u8
    /// - name  [u8; 32] 
    AddChannel,

    /// - signer  admin
    /// - write   server
    /// - write   channel
    /// - write   what about channel group mapping account? reuse place holder with SOL? require more sol?
    /// - write   channel_last
    DeleteChannel,

    /// Accounts:
    /// - write  server
    /// - signer admin
    /// - write  group
    /// Input: 
    /// - [CreateGroupInput]
    CreateGroup,

    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - write     group
    /// - write     group_last
    /// - write     group_channel
    /// - write     group_channel_last
    DeleteGroup,

    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - write     group_channel
    /// - read      channel
    AddChannelToGroup,


    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - read      channel
    /// - write     group_channel
    /// - write     group_channel_last
    RemoveChannelFromGroup,

    /// Accounts:
    /// - signer    owner
    /// - write     server
    /// - write     admin
    AddAdmin,

    /// Accounts:
    /// - signer    owner
    /// - write     server
    /// - write     admin
    /// - write     admin_last
    RemoveAdmin,
    
    ///   - writeable         server     
    ///   - writeable         dweller
    ///   - writeable         dweller_server
    ///   - writeable         member
    JoinServer,

    /// accounts:
    /// 
    /// - write    server
    /// - write    member
    /// - write    member_last
    /// - write    dweller_server
    /// - write    dweller_server_last
    LeaveServer,

    /// accounts:
    /// - write     server
    /// - write     member_status
    InviteToServer, 

    /// accounts:
    /// - write     server
    /// - write     member_status
    /// - write     member_status_last
    RevokeInviteServer, 

    /// accounts:
    /// - signer admin
    /// - write  server
    /// 
    /// Input: [SetNameInput]
    SetServerName,

    /// accounts:
    /// - signer admin
    /// - write  server
    /// 
    /// Input: [SetHashInput]        
    SetServerDb,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct CreateGroupInput {
    pub name: [u8;32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetNameInput {
    pub name: [u8;32],
}


/// IPFS hash
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetHashInput {
    pub hash: [u8;64],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeDwellerInput {
    pub name: [u8;32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeServerInput {
    pub name: [u8;32],
}


// /// Create `InitializeAsset` instruction
// #[allow(clippy::too_many_arguments)]
// pub fn initialize_asset(
//     rent: &Pubkey,
//     pool: &Pubkey,
//     asset: &Pubkey,
//     token: &Pubkey,
//     input: InitializeAssetInput,
// ) -> Result<solana_program::instruction::Instruction, ProgramError> {
//     let mut data = Instruction::InitializeAsset.try_to_vec()?;
//     let mut input = input.try_to_vec()?;
//     data.append(&mut input);
//     let accounts = vec![
//         AccountMeta::new_readonly(*rent, false),
//         AccountMeta::new_readonly(*pool, false), // makes sure prepare in same transaction
//         AccountMeta::new(*asset, false),
//         AccountMeta::new(*token, false),
//     ];
//     Ok(solana_program::instruction::Instruction {
//         program_id: crate::id(),
//         accounts,
//         data,
//     })
// }

// /// Create `InitializePool` instruction
// #[allow(clippy::too_many_arguments)]
// pub fn initialize_pool(
//     rent: &Pubkey,
//     program_token: &Pubkey,
//     pool: &Pubkey,
//     pool_mint: &Pubkey,
//     assets: &[Pubkey],
// ) -> Result<solana_program::instruction::Instruction, ProgramError> {
//     let data = Instruction::InitializePool.try_to_vec()?;
//     let mut accounts = vec![
//         AccountMeta::new_readonly(*rent, false),
//         AccountMeta::new_readonly(*program_token, false), // makes sure prepare in same transaction
//         AccountMeta::new_readonly(*pool, false),          // makes sure prepare in same transaction
//         AccountMeta::new(*pool_mint, false),
//     ];

//     for asset in assets {
//         accounts.push(AccountMeta::new(*asset, false));
//     }

//     Ok(solana_program::instruction::Instruction {
//         program_id: crate::id(),
//         accounts,
//         data,
//     })
// }
