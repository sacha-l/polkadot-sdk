//! Facade of currency implementation. Useful while migrating from old to new currency system.

use frame_support::traits::{
	fungible::{
		hold::{Balanced as FunHoldBalanced, Inspect as FunHoldInspect, Mutate as FunHoldMutate},
		Balanced, Inspect as FunInspect, Mutate as FunMutate,
	},
	tokens::Precision,
	Currency, InspectLockableCurrency, LockableCurrency,
};
use sp_runtime::{traits::Zero, DispatchResult};

use crate::{BalanceOf, Config, Error, HoldReason, NegativeImbalanceOf, PositiveImbalanceOf};

/// Existential deposit for the chain.
pub fn existential_deposit<T: Config>() -> BalanceOf<T> {
	T::Fungible::minimum_balance()
}

pub fn total_issuance<T: Config>() -> BalanceOf<T> {
	T::Fungible::total_issuance()
}

/// Make total balance equal to value.
pub fn set_balance<T: Config>(who: &T::AccountId, value: BalanceOf<T>) {
	T::Fungible::set_balance(who, value - T::Fungible::total_balance_on_hold(who));
}

pub fn burn<T: Config>(amount: BalanceOf<T>) -> PositiveImbalanceOf<T> {
	T::Currency::burn(amount)
}

/// Stakeable balance. Includes already staked + free to stake.
pub fn free_balance<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	// T::Currency::free_balance(who)
	T::Fungible::balance(who) + T::Fungible::balance_on_hold(&HoldReason::Staking.into(), who)
}

pub fn total_balance<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	T::Currency::total_balance(who)
}

/// Balance that is staked and at stake.
pub fn staked<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	T::Currency::balance_locked(crate::STAKING_ID, who) +
		T::Fungible::balance_on_hold(&HoldReason::Staking.into(), who)
}

pub fn update_stake<T: Config>(who: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
	// if first stake, inc provider.
	if staked::<T>(who) == Zero::zero() && amount > Zero::zero() {
		frame_system::Pallet::<T>::inc_providers(who);
	}

	T::Fungible::set_on_hold(&HoldReason::Staking.into(), who, amount)
	// T::Currency::set_lock(
	// 	crate::STAKING_ID,
	// 	who,
	// 	amount,
	// 	frame_support::traits::WithdrawReasons::all(),
	// );
}

pub fn kill_stake<T: Config>(who: &T::AccountId) -> DispatchResult {
	frame_system::Pallet::<T>::dec_providers(who)?;
	T::Fungible::release_all(&HoldReason::Staking.into(), who, Precision::BestEffort).map(|_| ())
	// T::Currency::remove_lock(crate::STAKING_ID, who);
}

pub fn slash<T: Config>(
	who: &T::AccountId,
	value: BalanceOf<T>,
) -> (NegativeImbalanceOf<T>, BalanceOf<T>) {
	T::Currency::slash(who, value)
}

/// Mint reward into an existing account. Does not increase the total issuance.
pub fn mint_existing<T: Config>(
	who: &T::AccountId,
	value: BalanceOf<T>,
) -> Option<PositiveImbalanceOf<T>> {
	T::Currency::deposit_into_existing(who, value).ok()
}

/// Mint reward and create if account does not exist. Does not increase the total issuance.
pub fn mint_creating<T: Config>(who: &T::AccountId, value: BalanceOf<T>) -> PositiveImbalanceOf<T> {
	T::Currency::deposit_creating(who, value)
}

/// Deposit to who from slashed value.
pub fn deposit_slashed<T: Config>(who: &T::AccountId, value: NegativeImbalanceOf<T>) {
	T::Currency::resolve_creating(who, value)
}

/// Increases total issuance.
pub fn issue<T: Config>(value: BalanceOf<T>) -> NegativeImbalanceOf<T> {
	T::Currency::issue(value)
}
