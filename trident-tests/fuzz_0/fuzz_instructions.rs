

pub mod fuzzme_fuzz_instructions {
    use std::ops::Sub;

    use crate::accounts_snapshots::*;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use anchor_lang::{prelude::System, solana_program::native_token::LAMPORTS_PER_SOL};
    use arbitrary::Arbitrary;
    use solana_sdk::{instruction::InstructionError, transaction::TransactionError};
    use trident_client::{fuzzing::*, prelude::solana_sdk::{self, account::AccountSharedData}};
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Fuzzme(Fuzzme),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Fuzzme {
        pub accounts: FuzzmeAccounts,
        pub data: FuzzmeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FuzzmeAccounts {
        // payer: AccountId,
        // receiver: AccountId
    }
    #[derive(Arbitrary, Debug)]
    pub struct FuzzmeData {
        pub value: u8,
    }
    impl<'info> IxOps<'info> for Fuzzme {
        type IxData = fuzzme::instruction::Fuzzme;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = FuzzmeSnapshot<'info>;
        fn check(
                &self,
                pre_ix: Self::IxSnapshot,
                post_ix: Self::IxSnapshot,
                ix_data: Self::IxData,
            ) -> Result<(), FuzzingError> {
                let pre_balance = pre_ix.payer.lamports.try_borrow().unwrap().to_le();
                let post_balance = post_ix.payer.lamports.try_borrow().unwrap().to_le();
                if pre_balance - post_balance > 10000 {
                    println!("unexpectedly lost lamports {} to {}",pre_balance, post_balance);
                    return Err(FuzzingError::Custom(41));
                    //panic!("unexpectedly lost lamports {} to {}",pre_balance, post_balance)
                }
            Ok(())
        }
        fn tx_error_handler(
                &self,
                e: FuzzClientErrorWithOrigin,
                ix_data: Self::IxData,
                pre_ix_acc_infos: &'info mut [Option<AccountInfo<'info>>],
            ) -> Result<(), FuzzClientErrorWithOrigin> {
                

            let client_error = &e.client_error;
            let error_code = fuzzme::FuzzMeError::NumberTooHigh as u32 + 6000;
            match client_error {
                FuzzClientError::BanksError(banks_error) => match banks_error.unwrap() {
                    TransactionError::InstructionError(_, InstructionError::Custom(error_code)) =>
                        {
                            if ix_data.value > 0xF0 {
                                return Ok(())
                            }
                            // println!("instruction resulted in an unexpected error! {}", banks_error);
                            // return Err(e);
                            panic!("instruction resulted in an unexpected error! {}", banks_error);
                        },
                    _ => Ok(())
                },
                _ => Ok(())
            }
            
        }
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzzme::instruction::Fuzzme { value: self.data.value };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            // let receiver = fuzz_accounts.receiver.get_or_create_account(
            //     self.accounts.receiver,
            //     client,
            //     1*LAMPORTS_PER_SOL,
            // );

            // let receiver = fuzz_accounts
            //     .receiver
            //     // gets the storage of all `state` account variants
            //     .storage()
            //     // returns the Keypair of the `state` account with
            //     // the given `AccountId` if it has been added previously
            //     .entry(self.accounts.receiver)
            //     .or_insert_with(|| {
            //         let space = 0;
            //         let rent_exempt_lamports = client.get_rent().unwrap()
            //                             .minimum_balance(space);
            //         let keypair = Keypair::new();
            //         let account = AccountSharedData::new_data_with_space::<[u8; 0]>(
            //             rent_exempt_lamports,
            //             &[],
            //             space,
            //             &SYSTEM_PROGRAM_ID,
            //         ).unwrap();
            //         // insert the custom account also into the client
            //         client.set_account_custom(&keypair.pubkey(), &account);
            //         keypair
            //     });

            let receiver = Keypair::new();

            let signers = vec![client.payer()];
            let mut acc_meta = fuzzme::accounts::Initialize {
                payer: client.payer().pubkey(), 
                receiver: receiver.pubkey(),
                system_progrm: SYSTEM_PROGRAM_ID
            }.to_account_metas(None);
            // acc_meta.push(AccountMeta{pubkey: client.payer().pubkey(), is_signer: true, is_writable: true});
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // payer: AccountsStorage<Keypair>,
        // receiver: AccountsStorage<Keypair>,
    }
}
