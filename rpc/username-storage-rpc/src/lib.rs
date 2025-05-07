#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::error::{ErrorCode, ErrorObject},
};
use pallet_username_storage::UsernameStorageApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_std::sync::Arc;

#[rpc(server, client)]
pub trait UsernameStorageRpcApi<AccountId> {
    #[method(name = "usernameStorage_getUsername")]
    fn get_username(&self, account_id: AccountId) -> RpcResult<Option<String>>;

    #[method(name = "usernameStorage_setUsername")]
    fn set_username(&self, account_id: AccountId, username: String) -> RpcResult<()>;
}

pub struct UsernameStorageRpc<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> UsernameStorageRpc<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> UsernameStorageRpcApiServer<AccountId> for UsernameStorageRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: UsernameStorageApi<Block, AccountId>,
    AccountId: Codec + Send + Sync + 'static,
{
    fn get_username(&self, account_id: AccountId) -> RpcResult<Option<String>> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        api.get_username(at, account_id)
            .map(|opt| opt.map(|bytes| String::from_utf8_lossy(&bytes).into_owned()))
            .map_err(|e| {
                ErrorObject::owned(
                    ErrorCode::ServerError(1).code(),
                    "Unable to get username",
                    Some(format!("{:?}", e)),
                )
            })
    }

    fn set_username(&self, account_id: AccountId, username: String) -> RpcResult<()> {
        let api = self.client.runtime_api();
        let at = self.client.info().best_hash;

        api.set_username(at, account_id, username.into_bytes())
            .map_err(|e| {
                ErrorObject::owned(
                    ErrorCode::ServerError(2).code(),
                    "Unable to set username",
                    Some(format!("{:?}", e)),
                )
            })?
            .map_err(|e| {
                ErrorObject::owned(
                    ErrorCode::ServerError(3).code(),
                    "Failed to set username",
                    Some(format!("{:?}", e)),
                )
            })
    }
}
