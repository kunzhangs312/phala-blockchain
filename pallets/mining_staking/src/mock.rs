// Creating mock runtime here

use crate as phala;
use crate::{Module, Trait};
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use pallet_balances as balances;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

pub(crate) type Balance = u128;
pub(crate) type BlockNumber = u64;

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		system<T>,
		phala<T>,
		balances<T>,
	}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const MinimumPeriod: u64 = 1;
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl pallet_balances::Trait for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type DustRemoval = ();
	type Event = TestEvent;
	type ExistentialDeposit = ();
	type AccountStore = System;
	type WeightInfo = ();
}

impl Trait for Test {
	type Event = TestEvent;
	type Currency = Balances;
}

mod test_events {
	pub use crate::Event;
}

pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type MiningStaking = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	sp_io::TestExternalities::new(t)
}
