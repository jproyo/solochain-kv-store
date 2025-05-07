#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_runtime::DispatchError;

sp_api::decl_runtime_apis! {
    pub trait UsernameStorageApi<AccountId: Codec> {
        fn get_username(account_id: AccountId) -> Result<Option<Vec<u8>>, DispatchError>;
    }
}
