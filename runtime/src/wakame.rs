use support::{traits::Currency, traits::WithdrawReason, traits::ExistenceRequirement, decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{As, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Hash};

pub trait Trait: balances::Trait + timestamp::Trait + system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as WakameModule {
		Amounts get(get_amount): map T::AccountId => Option<T::Balance>;
		Timestamps get(get_timestamp): map T::AccountId => Option<T::Moment>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn deposit(origin, amount: T::Balance) -> Result {
			let who = ensure_signed(origin)?;
			let _ = <balances::Module<T>>::withdraw(&who, amount, WithdrawReason::TransactionPayment, ExistenceRequirement::KeepAlive)?;
			if !<Amounts<T>>::exists(&who) {
				// Store argument
				<Amounts<T>>::insert(&who, amount);
				// Store current time
				<Timestamps<T>>::insert(&who, <timestamp::Module<T>>::now());
			} else {
				// get prev time and calculate difference
				let currentTime = <timestamp::Module<T>>::now();
				let prevTime = Self::get_timestamp(&who).ok_or("Timestamp not registered")?;
				let timeDifference = <T::Moment>::as_(currentTime.checked_sub(&prevTime).ok_or("Invalid timestamp")?);

				// Store argument
				let coeff = timeDifference
					.checked_mul(timeDifference).ok_or("Invalid timestamp")?
					.checked_mul(timeDifference).ok_or("Invalid timestamp")?
					.checked_div(1000).ok_or("Invalid timestamp")?;

				let nextAmount = Self::get_amount(&who).ok_or("Amount not registered")?
					.checked_mul(&<T::Balance>::sa(coeff)).ok_or("Invalid timestamp")?
					.checked_add(&amount).ok_or("Invalid amount")?;

				<Amounts<T>>::insert(&who, nextAmount);
			}
			Self::deposit_event(RawEvent::Deposit(who, amount));
			return Ok(())
		}

		pub fn withdraw(origin, amount: T::Balance) -> Result {
			//let who = ensure_signed(origin)?;
			//ensure!(<Amounts<T>>::exists(&who));
			//Self::deposit_event(RawEvent::Withdraw(who, amount));
			return Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where
	    AccountId = <T as system::Trait>::AccountId,
		Balance = <T as balances::Trait>::Balance,
	{
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		Deposit(AccountId, Balance),
		Withdraw(AccountId, Balance),
		Timetick(AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type WakameModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(WakameModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(WakameModule::something(), Some(42));
		});
	}
}
