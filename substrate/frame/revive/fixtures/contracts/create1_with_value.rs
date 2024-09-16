// This file is part of Substrate.

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

#![no_std]
#![no_main]

use common::{input, u256_bytes};
use uapi::{HostFn, HostFnImpl as api, ReturnErrorCode};

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
	input!(code_hash: &[u8; 32],);

	let mut value = [0; 32];
	api::value_transferred(&mut value);

	// Deploy the contract with no salt (equivalent to create1).
	let ret = api::instantiate(code_hash, 0u64, 0u64, None, &value, &[], None, None, None);
	assert!(ret.is_ok());
}
