use solana_program::account_info::AccountInfo;
use solana_program::{entrypoint, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RelationshipEnum {
    ParentOf,
    RelatedTo,
    Reference,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddressRelationship {
    pub first_address: String,
    pub second_address: String,
    pub relationship: RelationshipEnum,
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddressesStore {
    pub relationships: Vec<AddressRelationship>
}

entrypoint!(process_address);
pub fn process_address(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8],
) -> ProgramResult {
    let f = AddressRelationship::try_from(_instruction_data).unwrap();

    msg!("Hello, we're addressing your content.");
    msg!(format!("{:?}", f).as_str());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data = AddressRelationship {
            first_address: String::from("first_address"),
            second_address: String::from("second_address"),
            relationship: RelationshipEnum::ParentOf,
        };
        // instruction_data

        let accounts = vec![account];

        // assert_eq!(
        //     AddressesStore::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .relationships,
        //     0
        // );
        process_address(&program_id, &accounts, &instruction_data).unwrap();
        // assert_eq!(
        //     AddressesStore::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .relationships,
        //     1
        // );
        // process_address(&program_id, &accounts, &instruction_data).unwrap();
        // assert_eq!(
        //     AddressesStore::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .relationships,
        //     2
        // );
    }
}
