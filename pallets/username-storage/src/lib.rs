#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, traits::Get};
    use frame_system::pallet_prelude::*;
    use sp_api::decl_runtime_apis;
    use sp_std::{convert::TryInto, vec::Vec};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Maximum length of username
        #[pallet::constant]
        type MaxUsernameLength: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn usernames)]
    pub type Usernames<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u8, T::MaxUsernameLength>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Username was set for an account
        UsernameSet {
            /// The account that set the username
            who: T::AccountId,
            /// The username that was set
            username: BoundedVec<u8, T::MaxUsernameLength>,
        },
        /// Username was updated for an account
        UsernameUpdated {
            /// The account that updated the username
            who: T::AccountId,
            /// The new username
            username: BoundedVec<u8, T::MaxUsernameLength>,
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

            // Validate username
            ensure!(!username.is_empty(), Error::<T>::UsernameEmpty);
            ensure!(
                username.len() <= T::MaxUsernameLength::get() as usize,
                Error::<T>::UsernameTooLong
            );

            let bounded_username: BoundedVec<_, _> = username
                .try_into()
                .map_err(|_| Error::<T>::UsernameTooLong)?;

            // Check if username already exists
            let exists = Usernames::<T>::contains_key(&who);

            // Update storage
            Usernames::<T>::insert(&who, bounded_username.clone());

            // Emit appropriate event
            if exists {
                Self::deposit_event(Event::UsernameUpdated {
                    who,
                    username: bounded_username,
                });
            } else {
                Self::deposit_event(Event::UsernameSet {
                    who,
                    username: bounded_username,
                });
            }

            Ok(())
        }
    }
    decl_runtime_apis! {
        pub trait UsernameStorageApi<AccountId: codec::Codec> {
            fn get_username(account_id: AccountId) -> Option<Vec<u8>>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_noop, assert_ok, parameter_types,
        traits::{ConstU32, Everything},
    };
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use sp_std::convert::TryInto;

    type Block = frame_system::mocking::MockBlock<Test>;

    #[frame_support::runtime]
    mod runtime {
        #[runtime::runtime]
        #[runtime::derive(
            RuntimeCall,
            RuntimeEvent,
            RuntimeError,
            RuntimeOrigin,
            RuntimeFreezeReason,
            RuntimeHoldReason,
            RuntimeSlashReason,
            RuntimeLockId,
            RuntimeTask
        )]
        pub struct Test;

        #[runtime::pallet_index(0)]
        pub type System = frame_system;

        #[runtime::pallet_index(1)]
        pub type UsernameStorage = crate::pallet;
    }

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const SS58Prefix: u8 = 42;
        pub const MaxUsernameLength: u32 = 32;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Nonce = u64;
        type Block = Block;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type DbWeight = ();
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = SS58Prefix;
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
        type RuntimeTask = RuntimeTask;
        type ExtensionsWeightInfo = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type MaxUsernameLength = MaxUsernameLength;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| frame_system::Pallet::<Test>::set_block_number(1));
        ext
    }

    #[test]
    fn test_set_username() {
        new_test_ext().execute_with(|| {
            let account_id = 1;
            let username = b"test_user".to_vec();

            assert_ok!(UsernameStorage::set_username(
                RuntimeOrigin::signed(account_id),
                username.clone()
            ));
            assert_eq!(
                UsernameStorage::usernames(account_id),
                Some(username.try_into().unwrap())
            );
        });
    }

    #[test]
    fn test_set_username_empty() {
        new_test_ext().execute_with(|| {
            let account_id = 1;
            let username = b"".to_vec();

            assert_noop!(
                UsernameStorage::set_username(RuntimeOrigin::signed(account_id), username),
                Error::<Test>::UsernameEmpty
            );
        });
    }

    #[test]
    fn test_set_username_too_long() {
        new_test_ext().execute_with(|| {
            let account_id = 1;
            let username = vec![0; 33]; // 33 bytes is too long

            assert_noop!(
                UsernameStorage::set_username(RuntimeOrigin::signed(account_id), username),
                Error::<Test>::UsernameTooLong
            );
        });
    }
}
