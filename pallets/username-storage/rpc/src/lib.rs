#![cfg_attr(not(feature = "std"), no_std)]

use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::{ErrorCode, ErrorObjectOwned},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

use pallet_username_storage_runtime_api::UsernameStorageApi;

#[rpc(client, server)]
pub trait UsernameStorageRpcApi<BlockHash, AccountId> {
    #[method(name = "usernameStorage_getUsername")]
    fn get_username(
        &self,
        account_id: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u8>>>;
}

pub struct UsernameStorageRpc<C, P> {
    client: Arc<C>,
    _phantom: std::marker::PhantomData<P>,
}

impl<C, P> UsernameStorageRpc<C, P> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: Default::default(),
        }
    }
}

impl<C, Block, AccountId> UsernameStorageRpcApiServer<<Block as BlockT>::Hash, AccountId>
    for UsernameStorageRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: UsernameStorageApi<Block, AccountId>,
    AccountId: codec::Codec + Send + Sync + 'static,
{
    fn get_username(
        &self,
        account_id: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<Vec<u8>>> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_username(at, account_id)
            .map_err(|e| {
                ErrorObjectOwned::owned(
                    ErrorCode::InternalError.code(),
                    format!("Failed to fetch username: {:?}", e),
                    None::<Option<Vec<u8>>>,
                )
            })?
            .map_err(|e| {
                ErrorObjectOwned::owned(
                    ErrorCode::InternalError.code(),
                    format!("Runtime error: {:?}", e),
                    None::<Option<Vec<u8>>>,
                )
            })
    }
}
