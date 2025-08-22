// SPDX-FileCopyrightText: 2025 The Neeml Developers
//
// SPDX-License-Identifier: Apache-2.0

mod types;
mod client;

pub mod prelude {
    pub use super::types::*;
    pub use super::client::*;
}

pub trait Mailbox {
    fn mailbox_name() -> Option<String> {
        None
    }
}
