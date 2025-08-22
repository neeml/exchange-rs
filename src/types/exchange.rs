// SPDX-FileCopyrightText: 2025 The Neeml Developers
//
// SPDX-License-Identifier: Apache-2.0

use std::ops::Not;

type Result<T> = anyhow::Result<T, Box<dyn std::error::Error>>;

// --- Sealed trait for Exchange Server types ---
mod private {
    pub trait Sealed {}
}

pub trait ExchangeServerType: private::Sealed {}

// --- Marker structs for each server type ---


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exchange2016;
impl private::Sealed for Exchange2016 {}
impl ExchangeServerType for Exchange2016 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exchange2019;
impl private::Sealed for Exchange2019 {}
impl ExchangeServerType for Exchange2019 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeOnline;
impl private::Sealed for ExchangeOnline {}
impl ExchangeServerType for ExchangeOnline {}

// --- Connection State ---
#[derive(Default, Clone, PartialEq, Eq)]
pub enum ExchangeServerState {
    Connected,
    Authenticated,
    Unauthenticated,
    #[default]
    Disconnected,
}

// --- Generic ExchangeConnection ---
pub struct ExchangeConnection<ServerType: ExchangeServerType> {
    server_type: ServerType,
    server_state: ExchangeServerState,
}

impl<ServerType: ExchangeServerType> ExchangeConnection<ServerType> {
    pub fn new(server_type: ServerType) -> Self {
        Self {
            server_type,
            server_state: ExchangeServerState::Disconnected,
        }
    }

    // ... other common methods
    pub fn connected(&self) -> bool {
        self.server_state == ExchangeServerState::Connected
    }

    pub fn disconnected(&self) -> bool {
        self.connected().not()
    }

    pub fn authenticated(&self) -> bool {
        self.server_state == ExchangeServerState::Authenticated
    }

    pub fn unauthenticated(&self) -> bool {
        self.authenticated().not()
    }
}

// --- API-specific implementations ---

// For types that support EWS
pub trait EwsCompatible {}

impl EwsCompatible for Exchange2016 {}
impl EwsCompatible for Exchange2019 {}

impl<ServerType: ExchangeServerType + EwsCompatible> ExchangeConnection<ServerType> {
    pub async fn ews_operation(&self) -> Result<()> {
        // EWS-specific async logic here
        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_ews_operation(&self) -> Result<()> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.ews_operation())
    }
}

// For types that support Graph API
pub trait GraphApiCompatible {}
impl GraphApiCompatible for Exchange2016 {}
impl GraphApiCompatible for Exchange2019 {}
impl GraphApiCompatible for ExchangeOnline {}

impl<ServerType: ExchangeServerType + GraphApiCompatible> ExchangeConnection<ServerType> {
    pub async fn graph_operation(&self) -> Result<()> {
        // Graph API-specific async logic here
        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_graph_operation(&self) -> Result<()> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.graph_operation())
    }
}