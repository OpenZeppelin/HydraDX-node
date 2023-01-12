// This file is part of HydraDX-node

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use sp_runtime::FixedU128;

use frame_benchmarking::account;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use hydradx_traits::Registry;
use orml_traits::MultiCurrencyExtended;
use sp_runtime::Permill;

const TVL_CAP: Balance = 222_222_000_000_000_000_000_000;

benchmarks! {
	 where_clause {  where
		CurrencyOf<T>: MultiCurrencyExtended<T::AccountId, Amount=i128>,
		T: crate::pallet::Config,
		<T as pallet_omnipool::Config>::AssetId: From<u32>,
		<T as pallet_stableswap::Config>::AssetId: From<u32>,
		<T as pallet_omnipool::Config>::AssetId: Into<<T as pallet_stableswap::Config>::AssetId>,
		<T as pallet_omnipool::Config>::AssetId: From<<T as pallet_stableswap::Config>::AssetId>
	}
	create_subpool{

		let stable_amount: Balance = 1_000_000_000_000_000u128;
		let native_amount: Balance = 1_000_000_000_000_000u128;
		let stable_price: FixedU128= FixedU128::from((1,2));
		let native_price: FixedU128= FixedU128::from(1);
		let acc = OmnipoolPallet::<T>::protocol_account();

		OmnipoolPallet::<T>::set_tvl_cap(RawOrigin::Root.into(), TVL_CAP)?;

		CurrencyOf::<T>::update_balance(T::StableCoinAssetId::get(), &acc, stable_amount as i128)?;
		CurrencyOf::<T>::update_balance(T::HdxAssetId::get(), &acc, native_amount as i128)?;

		OmnipoolPallet::<T>::initialize_pool(RawOrigin::Root.into(), stable_price, native_price,Permill::from_percent(100), Permill::from_percent(100))?;

		// Register new asset in asset registry
		let asset_a = <T as pallet_omnipool::Config>::AssetRegistry::create_asset(&b"FCK".to_vec(), Balance::from(1_000u32))?;
		let asset_b = <T as pallet_omnipool::Config>::AssetRegistry::create_asset(&b"FCK2".to_vec(), Balance::from(1_000u32))?;
		let share_asset = <T as pallet_omnipool::Config>::AssetRegistry::create_asset(&b"SHR".to_vec(), Balance::from(1_000u32))?;

		// Create account for token provider and set balance
		let owner: T::AccountId = account("owner", 0, 1);

		let token_price: FixedU128= FixedU128::from((1,5));
		let token_amount = 200_000_000_000_000u128;

		CurrencyOf::<T>::update_balance(asset_a, &acc, token_amount as i128)?;
		CurrencyOf::<T>::update_balance(asset_b, &acc, token_amount as i128)?;

		// Add the token to the pool
		OmnipoolPallet::<T>::add_token(RawOrigin::Root.into(), asset_a, token_price,Permill::from_percent(100), owner.clone())?;
		OmnipoolPallet::<T>::add_token(RawOrigin::Root.into(), asset_b, token_price,Permill::from_percent(100), owner)?;

		let cap = Permill::from_percent(100);
		let amplification = 100u16;
		let trade_fee = Permill::from_percent(1);
		let withdraw_fee = Permill::from_percent(1);

	}: _(RawOrigin::Root, share_asset, asset_a, asset_b, cap, amplification, trade_fee, withdraw_fee)
	verify {
	}
}

#[cfg(test)]
mod tests {
	use super::Pallet;
	use crate::tests::mock::*;
	use frame_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(
		Pallet,
		super::ExtBuilder::default()
			.with_registered_asset(0)
			.with_registered_asset(1)
			.with_registered_asset(2)
			.build(),
		super::Test
	);
}
