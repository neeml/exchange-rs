// SPDX-FileCopyrightText: 2025 The Neeml Developers
//
// SPDX-License-Identifier: Apache-2.0

mod types;

pub mod prelude {
    pub use super::types::*;
}

pub trait Mailbox {
    fn mailbox_name() -> Option<String> {
        None
    }
}
