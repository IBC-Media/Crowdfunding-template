#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
pub mod types;
pub use types::ProjectInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	pub type BalanceIn<Runtime> = <<Runtime as Config>::Currency as Currency<
		<Runtime as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	pub type Project<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash,
		ProjectInfo<T::AccountId, BalanceIn<T>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CrowdFundingInitiated { project: ProjectInfo<T::AccountId, BalanceIn<T>> },
		CrowdFundTransfer { source: T::AccountId, destination: T::AccountId },
		CrowdFundWithdrawn { source: T::AccountId, destination: T::AccountId },
		CrowdFundingStopped { project: T::Hash },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// If project not found
		ProjectNotFound,
		/// If project is not active
		ProjectNotActive,
		/// If the target amount reached
		TargetAmountReached,
		/// if the fund amount is less the minimum amount to fund
		IncreaseAmount,
		/// If random user tries to stop the crowd funding.
		OnlyOwnerCanStopCrowdFunding,
		/// if a user try to initiate the project with same project id
		DuplicateProjectIdNotAllowed,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.

		/// Any User can initiate the project
		/// User need to specify some field like: pot_account, target amount, minimum contribution.
		#[pallet::call_index(0)]
		#[pallet::weight({10_000})]
		pub fn initiate_project(
			origin: OriginFor<T>,
			project_id: T::Hash,
			project: ProjectInfo<T::AccountId, BalanceIn<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let is_available = Project::<T>::contains_key(project_id);

			ensure!(!is_available, Error::<T>::DuplicateProjectIdNotAllowed);

			// Update the Project storage
			// todo!()

			Self::deposit_event(Event::<T>::CrowdFundingInitiated { project });

			Ok(())
		}

		/// Any individual can support this project and fund to the pot account.
		/// If target amount hit then if the fund will automatically transferred to the owner's
		/// account. Also after the target reached, the no one can fund this project.
		#[pallet::call_index(1)]
		#[pallet::weight({10_000})]
		pub fn fund_project(
			origin: OriginFor<T>,
			project_id: T::Hash,
			amount: BalanceIn<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut project = Project::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

			// check the status of a proposal.
			ensure!(project.status == true, Error::<T>::ProjectNotActive);

			// Check the target amount of a project
			ensure!(project.target_fund > project.total_fund, Error::<T>::TargetAmountReached);

			// Check the minimum amount that should be fund
			ensure!(project.min_fund <= amount, Error::<T>::IncreaseAmount);

			// Add the balance to the total_fund;
			// let total_fund = project.total_fund + amount;
			project.total_fund += amount;

			// Check condition if already present..
			if !project.contributors.contains(&who) {
				// Add this user as a contributor
				project.contributors.push(who.clone());
			}

			// Transfer the balance
			T::Currency::transfer(
				&who,
				&project.pot_account,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;

			// dispatch the event
			Self::deposit_event(Event::<T>::CrowdFundTransfer {
				source: who,
				destination: project.pot_account.clone(),
			});

			Project::<T>::insert(project_id, project.clone());

			// Check if the target amount is reached then change the status of a project.
			if project.total_fund >= project.target_fund {

				// Update the status
				// todo!()
				// transfer the balance
				// todo!()

				// Update the total fund
				// todo!()

				Self::deposit_event(Event::<T>::CrowdFundWithdrawn {
					source: project.pot_account.clone(),
					destination: project.owner.clone(),
				});

				// Update the storage
				// todo!()
			}

			Ok(())
		}

		/// If owner wants to stop crowdFunding in the middle.
		/// After crowdFunding stopped, no one can fund this project
		/// Also the amount will transfer from the pot account to owner's account.
		#[pallet::call_index(2)]
		#[pallet::weight({10_000})]
		pub fn stop_crowdfunding(origin: OriginFor<T>, project_id: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut project = Project::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

			// Check this operation should perform from the owner only
			ensure!(project.owner == who.clone(), Error::<T>::OnlyOwnerCanStopCrowdFunding);

			// check the status of a proposal.
			ensure!(project.status == true, Error::<T>::ProjectNotActive);

			T::Currency::transfer(
				&project.pot_account,
				&project.owner,
				project.total_fund,
				ExistenceRequirement::KeepAlive,
			)?;

			project.status = false;

			// reduce the total fund to be zero.
			project.total_fund = project.total_fund - project.total_fund;

			Self::deposit_event(Event::<T>::CrowdFundWithdrawn {
				source: project.pot_account.clone(),
				destination: project.owner.clone(),
			});

			Project::<T>::insert(project_id, project.clone());

			Ok(())
		}
	}
}
