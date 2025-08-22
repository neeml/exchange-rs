// src/client.rs

// SPDX-FileCopyrightText: 2025 The Neeml Developers
//
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use std::marker::PhantomData;

// --- Client State Marker Traits ---
mod private {
    pub trait Sealed {}
}
pub trait ClientState: private::Sealed {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disconnected;
impl private::Sealed for Disconnected {}
impl ClientState for Disconnected {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connected;
impl private::Sealed for Connected {}
impl ClientState for Connected {}

// --- Client Struct ---
pub struct Client<ServerType: ExchangeServerType, State: ClientState> {
    connection: ExchangeConnection<ServerType>,
    state: PhantomData<State>,
}

// --- Methods for Disconnected state ---
impl<ServerType: ExchangeServerType> Client<ServerType, Disconnected> {
    pub fn new(server_type: ServerType) -> Self {
        Self {
            connection: ExchangeConnection::new(server_type),
            state: PhantomData,
        }
    }

    pub async fn connect(self) -> anyhow::Result<Client<ServerType, Connected>> {
        // Perform connection logic here...
        println!("Connecting...");
        Ok(Client {
            connection: self.connection,
            state: PhantomData,
        })
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_connect(self) -> anyhow::Result<Client<ServerType, Connected>> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.connect())
    }
}

// --- Methods for Connected state ---
impl<ServerType: ExchangeServerType> Client<ServerType, Connected> {
    pub async fn disconnect(self) -> Client<ServerType, Disconnected> {
        // Perform disconnection logic here...
        println!("Disconnecting...");
        Client {
            connection: self.connection,
            state: PhantomData,
        }
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_disconnect(self) -> Client<ServerType, Disconnected> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.disconnect())
    }
}

// --- EWS-specific methods for Connected state ---
impl<ServerType: ExchangeServerType + EwsCompatible> Client<ServerType, Connected> {
    pub async fn ews_operation(&self) -> anyhow::Result<()> {
        self.connection.ews_operation()
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_ews_operation(&self) -> anyhow::Result<()> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.ews_operation())
    }
}

// --- Graph API-specific methods for Connected state ---
impl<ServerType: ExchangeServerType + GraphApiCompatible> Client<ServerType, Connected> {
    pub async fn graph_operation(&self) -> anyhow::Result<()> {
        self.connection.graph_operation()
    }

    #[cfg(feature = "blocking")]
    pub fn blocking_graph_operation(&self) -> anyhow::Result<()> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.graph_operation())
    }
}
