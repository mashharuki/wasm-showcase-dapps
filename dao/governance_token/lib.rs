#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/**
 * governance_token contract
 */
#[openbrush::contract]
pub mod governance_token {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp22::extensions::metadata::*;

    /**
     * MyPSP22 Token Struct
     */
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Metadata for MyPSP22 {}

    impl MyPSP22 {
        /**
         * new メソッド
         */
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.name = Some(String::from("GOV TOKEN"));
                instance.metadata.symbol = Some(String::from("GOV"));
                instance.metadata.decimals = 18;
                instance
                    ._mint_to(instance.env().caller(), total_supply)
                    .expect("Should mint total_supply");
            })
        }
    }
}
