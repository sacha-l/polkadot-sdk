// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Genesis configs presets for the BridgeHubRococo runtime

use crate::*;
use sp_core::sr25519;
use sp_std::vec::Vec;
use testnet_parachains_constants::genesis_presets::*;

const BRIDGE_HUB_ROCOCO_ED: Balance = ExistentialDeposit::get();

/// Default genesis pallet configurations for BridgeHubRococo
pub fn bridge_hub_rococo_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
	bridges_pallet_owner: Option<AccountId>,
	asset_hub_para_id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1u128 << 60)).collect::<Vec<_>>(),
		},
		"parachainInfo": ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		"collatorSelection": CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: BRIDGE_HUB_ROCOCO_ED * 16,
			..Default::default()
		},
		"session": SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                         // account id
						acc,                                 // validator id
						SessionKeys { aura }, 			// session keys
					)
				})
				.collect(),
			..Default::default()
		},
		"polkadotXcm": PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		"bridgeWestendGrandpa": BridgeWestendGrandpaConfig {
			owner: bridges_pallet_owner.clone(),
			..Default::default()
		},
		"bridgeWestendMessages": BridgeWestendMessagesConfig {
			owner: bridges_pallet_owner.clone(),
			..Default::default()
		},
		"ethereumSystem": EthereumSystemConfig {
			para_id: id,
			asset_hub_para_id,
			..Default::default()
		}
	})
}

/// Default genesis setup for `local_testnet` preset id.
pub fn bridge_hub_rococo_local_testnet_genesis(
	para_id: ParaId,
	bridges_pallet_owner: Option<AccountId>,
	asset_hub_para_id: ParaId,
) -> serde_json::Value {
	bridge_hub_rococo_genesis(
		test_invulnerables(),
		testnet_accounts(),
		para_id,
		bridges_pallet_owner,
		asset_hub_para_id,
	)
}

/// Default genesis setup for `development` preset id.
pub fn bridge_hub_rococo_development_genesis(
	para_id: ParaId,
	bridges_pallet_owner: Option<AccountId>,
	asset_hub_para_id: ParaId,
) -> serde_json::Value {
	bridge_hub_rococo_genesis(
		test_invulnerables(),
		testnet_accounts(),
		para_id,
		bridges_pallet_owner,
		asset_hub_para_id,
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<sp_std::vec::Vec<u8>> {
	let patch = match id.try_into() {
		Ok("development") => bridge_hub_rococo_development_genesis(
			1013.into(),
			Some(get_account_id_from_seed::<sr25519::Public>("Bob")),
			rococo_runtime_constants::system_parachain::ASSET_HUB_ID.into(),
		),
		Ok("local_testnet") => bridge_hub_rococo_local_testnet_genesis(
			1013.into(),
			Some(get_account_id_from_seed::<sr25519::Public>("Bob")),
			rococo_runtime_constants::system_parachain::ASSET_HUB_ID.into(),
		),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> sp_std::vec::Vec<sp_genesis_builder::PresetId> {
	sp_std::vec![
		sp_genesis_builder::PresetId::from("local_testnet"),
		sp_genesis_builder::PresetId::from("development"),
	]
}
