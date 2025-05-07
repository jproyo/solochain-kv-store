#[cfg(test)]
mod tests {
    use super::*;
    use crate::pallet::*;
    use frame_support::{
        assert_ok, parameter_types,
        traits::{ConstU32, ConstU64},
    };
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system,
            UsernameStorage: Pallet,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const SS58Prefix: u8 = 42;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type Origin = Origin;
        type Call = Call;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = Event;
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
    }

    impl Config for Test {
        type RuntimeEvent = Event;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    #[test]
    fn test_set_username() {
        new_test_ext().execute_with(|| {
            let account_id = 1;
            let username = b"test_user".to_vec();

            assert_ok!(UsernameStorage::set_username(
                Origin::signed(account_id),
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
                UsernameStorage::set_username(Origin::signed(account_id), username),
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
                UsernameStorage::set_username(Origin::signed(account_id), username),
                Error::<Test>::UsernameTooLong
            );
        });
    }
}
