use crate::{mock::*, Error, ProjectInfo};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_runtime::traits::Hash;

pub type HashType = <Test as frame_system::Config>::Hash;
pub type Hashing = <Test as frame_system::Config>::Hashing;
pub type AccountId = <Test as frame_system::Config>::AccountId;
type Balance = <Balances as Currency<AccountId>>::Balance;

#[test]
fn initiate_project_successfully() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));

		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(1), project_id, pr));
	})
}

#[test]
fn initiate_duplicate_project_fail() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));

		assert_ok!(TemplateModule::initiate_project(
			RuntimeOrigin::signed(1),
			project_id,
			pr.clone()
		));
		assert_noop!(
			TemplateModule::initiate_project(RuntimeOrigin::signed(1), project_id, pr),
			Error::<Test>::DuplicateProjectIdNotAllowed
		);
	})
}

#[test]
fn fund_project_successfully() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const BOB: <Test as frame_system::Config>::AccountId = 2;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));
		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(ALICE), project_id, pr));
		assert_ok!(TemplateModule::fund_project(RuntimeOrigin::signed(BOB), project_id, 70));
	})
}

#[test]
fn fund_inactive_project_fail() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const BOB: <Test as frame_system::Config>::AccountId = 2;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));
		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(ALICE), project_id, pr));
		assert_ok!(TemplateModule::fund_project(RuntimeOrigin::signed(BOB), project_id, 100));

		assert_noop!(
			TemplateModule::fund_project(RuntimeOrigin::signed(CHARLIE), project_id, 10),
			Error::<Test>::ProjectNotActive,
		);
	})
}

#[test]
fn fund_project_below_min_amount_fail() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));
		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(ALICE), project_id, pr));

		assert_noop!(
			TemplateModule::fund_project(RuntimeOrigin::signed(CHARLIE), project_id, 5),
			Error::<Test>::IncreaseAmount,
		);
	})
}

#[test]
fn stop_crowdfunding_successfully() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));
		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(ALICE), project_id, pr));

		assert_ok!(TemplateModule::stop_crowdfunding(RuntimeOrigin::signed(ALICE), project_id));
	})
}

#[test]
fn stop_crowdfunding_fail() {
	new_test_ext().execute_with(|| {
		const ALICE: <Test as frame_system::Config>::AccountId = 1;
		const CHARLIE: <Test as frame_system::Config>::AccountId = 3;

		let pr = ProjectInfo::<AccountId, Balance> {
			owner: ALICE,
			total_fund: 0,
			target_fund: 100,
			min_fund: 10,
			contributors: vec![],
			status: true,
			pot_account: CHARLIE,
		};

		let project_id = HashType::from(Hashing::hash_of(&40));
		assert_ok!(TemplateModule::initiate_project(RuntimeOrigin::signed(ALICE), project_id, pr));

		assert_noop!(
			TemplateModule::stop_crowdfunding(RuntimeOrigin::signed(CHARLIE), project_id),
			Error::<Test>::OnlyOwnerCanStopCrowdFunding,
		);
	})
}
