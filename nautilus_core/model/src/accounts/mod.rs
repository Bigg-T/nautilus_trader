// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Account types such as `CashAccount` and `MarginAccount`.

pub mod any;
pub mod base;
pub mod cash;
pub mod margin;

#[cfg(feature = "stubs")]
pub mod stubs;

// Re-exports
pub use crate::accounts::{
    any::AccountAny,
    base::{Account, BaseAccount},
    cash::CashAccount,
    margin::MarginAccount,
};
