// SPDX-FileCopyrightText: 2025 The Neeml Developers
//
// SPDX-License-Identifier: Apache-2.0

use std::ops::Not;

type Result<T> = anyhow::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeApiKind {
    Ews,
    Graph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServerKind {
    Exchange2013(ExchangeApiKind),
    Exchange2016(Vec<ExchangeApiKind>),
    Exchange2019(Vec<ExchangeApiKind>),
    ExchangeOnline(ExchangeApiKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServer {
    OnPremise(ExchangeServerKind),
    Online(ExchangeServerKind),
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ExchangeServerState {
    Connected,
    Authenticated,
    Unauthenticated,
    #[default]
    Disconnected,
}

pub struct ExchangeConnection<ServerType = ExchangeServer, ServerState = ExchangeServerState> {
    server_type: ServerType,
    server_state: ServerState,
}

impl ExchangeConnection {
    pub fn new(server_type: ExchangeServer, server_state: ExchangeServerState) -> Result<Self> {
        Ok(Self {
            server_type,
            server_state,
        })
    }

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

    pub fn on_premise(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(_) => true,
            _ => false,
        }
    }

    pub fn exchange_online(&self) -> bool {
        match self.server_type {
            ExchangeServer::Online(_) => true,
            _ => false,
        }
    }

    pub fn is_ews(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2013(ExchangeApiKind::Ews)) => {
                true
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2016(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2019(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            _ => false,
        }
    }

    pub fn is_graph(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2016(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2019(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            ExchangeServer::Online(ExchangeServerKind::ExchangeOnline(_)) => true,
            _ => false,
        }
    }
}
