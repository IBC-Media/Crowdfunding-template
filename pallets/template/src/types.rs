use codec::{Decode, Encode};
use frame_support::sp_runtime::RuntimeDebug;

use frame_support::dispatch::Vec;
use scale_info::TypeInfo;

#[derive(Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ProjectInfo<AccountId, Balance> {
	pub owner: AccountId,
	pub total_fund: Balance,
	pub target_fund: Balance,
	pub min_fund: Balance,
	pub contributors: Vec<AccountId>,
	pub status: bool,
	pub pot_account: AccountId,
}
