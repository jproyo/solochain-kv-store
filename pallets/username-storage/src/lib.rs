#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_username_storage_rpc as rpc;
pub use pallet_username_storage_runtime_api as runtime_api;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn usernames)]
    pub type Usernames<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, ConstU32<32>>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Username was set for an account
        UsernameSet {
            who: T::AccountId,
            username: BoundedVec<u8, ConstU32<32>>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Username is too long
        UsernameTooLong,
        /// Username is empty
        UsernameEmpty,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn set_username(origin: OriginFor<T>, username: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure username is not empty
            ensure!(!username.is_empty(), Error::<T>::UsernameEmpty);

            // Convert to bounded vec and ensure it's not too long
            let bounded_username: BoundedVec<_, _> = username
                .try_into()
                .map_err(|_| Error::<T>::UsernameTooLong)?;

            // Store the username
            Usernames::<T>::insert(&who, bounded_username.clone());

            // Emit the event
            Self::deposit_event(Event::UsernameSet {
                who,
                username: bounded_username,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_username(account_id: T::AccountId) -> Option<Vec<u8>> {
            Self::usernames(account_id).map(|v| v.to_vec())
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use super::*;
    use frame_benchmarking::v2::*;
    use frame_system::RawOrigin;

    #[benchmarks]
    mod set_username {
        use super::*;

        #[benchmark]
        fn set_username() {
            let username = b"test_username".to_vec();
            let caller: T::AccountId = whitelisted_caller();

            #[extrinsic_call]
            set_username(RawOrigin::Signed(caller), username);
        }
    }
}
