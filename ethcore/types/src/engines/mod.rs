// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Engine-specific types.

use ethereum_types::{Address, H256, H64};
use bytes::Bytes;
use ethjson;
use rlp::Rlp;
use unexpected::Mismatch;

use crate::{BlockNumber, errors::{BlockError, EthcoreError}};

pub mod epoch;
pub mod params;
pub mod machine;

/// Optimize cache for CPU or memory usage
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OptimizeFor {
	/// Optimize cache for CPU
	Cpu,
	/// Optimize cache for memory
	Memory,
}

impl Default for OptimizeFor {
	fn default() -> Self {
		OptimizeFor::Cpu
	}
}

/// Ethash/Clique specific seal
#[derive(Debug, PartialEq)]
pub struct EthashSeal {
	/// Ethash seal mix_hash
	pub mix_hash: H256,
	/// Ethash seal nonce
	pub nonce: H64,
}

impl EthashSeal {
	/// Tries to parse rlp encoded bytes as an Ethash/Clique seal.
	pub fn parse_seal<T: AsRef<[u8]>>(seal: &[T]) -> Result<Self, EthcoreError> {
		if seal.len() != 2 {
			return Err(BlockError::InvalidSealArity(
				Mismatch {
					expected: 2,
					found: seal.len()
				}
			).into());
		}

		let mix_hash = Rlp::new(seal[0].as_ref()).as_val::<H256>()?;
		let nonce = Rlp::new(seal[1].as_ref()).as_val::<H64>()?;
		Ok(EthashSeal { mix_hash, nonce })
	}
}


/// Seal type.
#[derive(Debug, PartialEq, Eq)]
pub enum Seal {
	/// Regular block seal; should be part of the blockchain.
	Regular(Vec<Bytes>),
	/// Engine does not generate seal for this block right now.
	None,
}

/// The type of sealing the engine is currently able to perform.
#[derive(Debug, PartialEq, Eq)]
pub enum SealingState {
	/// The engine is ready to seal a block.
	Ready,
	/// The engine can't seal at the moment, and no block should be prepared and queued.
	NotReady,
	/// The engine does not seal internally.
	External,
}

/// The number of generations back that uncles can be.
pub const MAX_UNCLE_AGE: usize = 6;

/// Default EIP-210 contract code.
/// As defined in https://github.com/ethereum/EIPs/pull/210
pub const DEFAULT_BLOCKHASH_CONTRACT: &'static [u8] = &[
	0x73, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xfe, 0x33, 0x14, 0x15, 0x61, 0x00, 0x6a, 0x57, 0x60, 0x01, 0x43, 0x03,
	0x60, 0x00, 0x35, 0x61, 0x01, 0x00, 0x82, 0x07, 0x55, 0x61, 0x01, 0x00, 0x81, 0x07, 0x15, 0x15,
	0x61, 0x00, 0x45, 0x57, 0x60, 0x00, 0x35, 0x61, 0x01, 0x00, 0x61, 0x01, 0x00, 0x83, 0x05, 0x07,
	0x61, 0x01, 0x00, 0x01, 0x55, 0x5b, 0x62, 0x01, 0x00, 0x00, 0x81, 0x07, 0x15, 0x15, 0x61, 0x00,
	0x64, 0x57, 0x60, 0x00, 0x35, 0x61, 0x01, 0x00, 0x62, 0x01, 0x00, 0x00, 0x83, 0x05, 0x07, 0x61,
	0x02, 0x00, 0x01, 0x55, 0x5b, 0x50, 0x61, 0x01, 0x3e, 0x56, 0x5b, 0x43, 0x60, 0x00, 0x35, 0x12,
	0x15, 0x15, 0x61, 0x00, 0x84, 0x57, 0x60, 0x00, 0x60, 0x40, 0x52, 0x60, 0x20, 0x60, 0x40, 0xf3,
	0x61, 0x01, 0x3d, 0x56, 0x5b, 0x61, 0x01, 0x00, 0x60, 0x00, 0x35, 0x43, 0x03, 0x13, 0x15, 0x15,
	0x61, 0x00, 0xa8, 0x57, 0x61, 0x01, 0x00, 0x60, 0x00, 0x35, 0x07, 0x54, 0x60, 0x60, 0x52, 0x60,
	0x20, 0x60, 0x60, 0xf3, 0x61, 0x01, 0x3c, 0x56, 0x5b, 0x61, 0x01, 0x00, 0x60, 0x00, 0x35, 0x07,
	0x15, 0x15, 0x61, 0x00, 0xc5, 0x57, 0x62, 0x01, 0x00, 0x00, 0x60, 0x00, 0x35, 0x43, 0x03, 0x13,
	0x15, 0x61, 0x00, 0xc8, 0x56, 0x5b, 0x60, 0x00, 0x5b, 0x15, 0x61, 0x00, 0xea, 0x57, 0x61, 0x01,
	0x00, 0x61, 0x01, 0x00, 0x60, 0x00, 0x35, 0x05, 0x07, 0x61, 0x01, 0x00, 0x01, 0x54, 0x60, 0x80,
	0x52, 0x60, 0x20, 0x60, 0x80, 0xf3, 0x61, 0x01, 0x3b, 0x56, 0x5b, 0x62, 0x01, 0x00, 0x00, 0x60,
	0x00, 0x35, 0x07, 0x15, 0x15, 0x61, 0x01, 0x09, 0x57, 0x63, 0x01, 0x00, 0x00, 0x00, 0x60, 0x00,
	0x35, 0x43, 0x03, 0x13, 0x15, 0x61, 0x01, 0x0c, 0x56, 0x5b, 0x60, 0x00, 0x5b, 0x15, 0x61, 0x01,
	0x2f, 0x57, 0x61, 0x01, 0x00, 0x62, 0x01, 0x00, 0x00, 0x60, 0x00, 0x35, 0x05, 0x07, 0x61, 0x02,
	0x00, 0x01, 0x54, 0x60, 0xa0, 0x52, 0x60, 0x20, 0x60, 0xa0, 0xf3, 0x61, 0x01, 0x3a, 0x56, 0x5b,
	0x60, 0x00, 0x60, 0xc0, 0x52, 0x60, 0x20, 0x60, 0xc0, 0xf3, 0x5b, 0x5b, 0x5b, 0x5b, 0x5b];

/// Fork choice.
#[derive(Debug, PartialEq, Eq)]
pub enum ForkChoice {
	/// Choose the new block.
	New,
	/// Choose the current best block.
	Old,
}

/// Ethash-specific extensions.
#[derive(Debug, Clone)]
pub struct EthashExtensions {
	/// Homestead transition block number.
	pub homestead_transition: BlockNumber,
	/// DAO hard-fork transition block (X).
	pub dao_hardfork_transition: u64,
	/// DAO hard-fork refund contract address (C).
	pub dao_hardfork_beneficiary: Address,
	/// DAO hard-fork DAO accounts list (L)
	pub dao_hardfork_accounts: Vec<Address>,
    /// ETG hard-fork transition block.
    pub etg_hardfork_transition: u64,
    /// ETG hard-fork dev address.
    pub etg_hardfork_dev_accounts: Vec<Address>,
}

impl From<ethjson::spec::EthashParams> for EthashExtensions {
	fn from(p: ::ethjson::spec::EthashParams) -> Self {
		EthashExtensions {
			homestead_transition: p.homestead_transition.map_or(0, Into::into),
			dao_hardfork_transition: p.dao_hardfork_transition.map_or(u64::max_value(), Into::into),
			dao_hardfork_beneficiary: p.dao_hardfork_beneficiary.map_or_else(Address::zero, Into::into),
			dao_hardfork_accounts: p.dao_hardfork_accounts.unwrap_or_else(Vec::new).into_iter().map(Into::into).collect(),
            etg_hardfork_transition: p.etg_hardfork_transition.map_or(u64::max_value(), Into::into),
            etg_hardfork_dev_accounts: p.etg_hardfork_dev_accounts.unwrap_or_else(Vec::new).into_iter().map(Into::into).collect(),
		}
	}
}

/// Type alias for a function we can get headers by hash through.
pub type Headers<'a, H> = dyn Fn(H256) -> Option<H> + 'a;

/// Type alias for a function we can query pending transitions by block hash through.
pub type PendingTransitionStore<'a> = dyn Fn(H256) -> Option<epoch::PendingTransition> + 'a;
