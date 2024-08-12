// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec::{Extensions, GenericChainSpec};
use asset_hub_rococo_runtime::genesis_config_presets::{
	asset_hub_rococo_development_genesis, asset_hub_rococo_genesis,
	asset_hub_rococo_local_testnet_genesis,
};
use asset_hub_westend_runtime::genesis_config_presets::{
	asset_hub_westend_development_genesis, asset_hub_westend_genesis,
	asset_hub_westend_local_testnet_genesis,
};
use hex_literal::hex;
use parachains_common::Balance as AssetHubBalance;
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;

const ASSET_HUB_WESTEND_ED: AssetHubBalance = asset_hub_westend_runtime::ExistentialDeposit::get();
const ASSET_HUB_ROCOCO_ED: AssetHubBalance = asset_hub_rococo_runtime::ExistentialDeposit::get();

pub fn asset_hub_westend_development_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());
	let para_id = 1000;

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend".into(), para_id },
	)
	.with_name("Westend Asset Hub Development")
	.with_id("asset-hub-westend-dev")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_westend_development_genesis(para_id.into()))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_westend_local_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());
	let para_id = 1000;

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend-local".into(), para_id },
	)
	.with_name("Westend Asset Hub Local")
	.with_id("asset-hub-westend-local")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_westend_local_testnet_genesis(para_id.into()))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_westend_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend".into(), para_id: 1000 },
	)
	.with_name("Westend Asset Hub")
	.with_id("asset-hub-westend")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(asset_hub_westend_genesis(
		// initial collators.
		vec![
			(
				hex!("9cfd429fa002114f33c1d3e211501d62830c9868228eb3b4b8ae15a83de04325").into(),
				hex!("9cfd429fa002114f33c1d3e211501d62830c9868228eb3b4b8ae15a83de04325")
					.unchecked_into(),
			),
			(
				hex!("12a03fb4e7bda6c9a07ec0a11d03c24746943e054ff0bb04938970104c783876").into(),
				hex!("12a03fb4e7bda6c9a07ec0a11d03c24746943e054ff0bb04938970104c783876")
					.unchecked_into(),
			),
			(
				hex!("1256436307dfde969324e95b8c62cb9101f520a39435e6af0f7ac07b34e1931f").into(),
				hex!("1256436307dfde969324e95b8c62cb9101f520a39435e6af0f7ac07b34e1931f")
					.unchecked_into(),
			),
			(
				hex!("98102b7bca3f070f9aa19f58feed2c0a4e107d203396028ec17a47e1ed80e322").into(),
				hex!("98102b7bca3f070f9aa19f58feed2c0a4e107d203396028ec17a47e1ed80e322")
					.unchecked_into(),
			),
		],
		Vec::new(),
		ASSET_HUB_WESTEND_ED * 4096,
		1000.into(),
	))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_rococo_development_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_rococo_like_development_config(
		properties,
		"Rococo Asset Hub Development",
		"asset-hub-rococo-dev",
		1000,
	)
}

fn asset_hub_rococo_like_development_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-dev".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_rococo_development_genesis(para_id.into()))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_rococo_local_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_rococo_like_local_config(
		properties,
		"Rococo Asset Hub Local",
		"asset-hub-rococo-local",
		1000,
	)
}

fn asset_hub_rococo_like_local_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_rococo_local_testnet_genesis(para_id.into()))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_rococo_genesis_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	let para_id = 1000;
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo".into(), para_id },
	)
	.with_name("Rococo Asset Hub")
	.with_id("asset-hub-rococo")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(asset_hub_rococo_genesis(
		// initial collators.
		vec![
			// E8XC6rTJRsioKCp6KMy6zd24ykj4gWsusZ3AkSeyavpVBAG
			(
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into(),
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608")
					.unchecked_into(),
			),
			// G28iWEybndgGRbhfx83t7Q42YhMPByHpyqWDUgeyoGF94ri
			(
				hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944").into(),
				hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944")
					.unchecked_into(),
			),
			// G839e2eMiq7UXbConsY6DS1XDAYG2XnQxAmLuRLGGQ3Px9c
			(
				hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948").into(),
				hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948")
					.unchecked_into(),
			),
			// GLao4ukFUW6qhexuZowdFrKa2NLCfnEjZMftSXXfvGv1vvt
			(
				hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f").into(),
				hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f")
					.unchecked_into(),
			),
		],
		Vec::new(),
		ASSET_HUB_ROCOCO_ED * 524_288,
		para_id.into(),
	))
	.with_properties(properties)
	.build()
}
