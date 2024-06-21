use std::ops::Not;

type Result<T> = anyhow::Result<T, anyhow::Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeApiKind {
    Ews,
    Graph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServerKind {
    Exchange2016(Vec<ExchangeApiKind>),
    Exchange2019(Vec<ExchangeApiKind>),
    ExchangeOnline(Vec<ExchangeApiKind>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServer {
    OnPremise(ExchangeServerKind),
    Online(ExchangeServerKind),
}

pub struct ExchangeConnection<T = ExchangeServer> {
    server_type: T,
    connected: bool,
}

impl ExchangeConnection {
    pub fn new(server_type: ExchangeServer) -> Result<Self> {
        Ok(Self {
            server_type,
            connected: false,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn is_disconnected(&self) -> bool {
        self.connected.not()
    }

    pub fn is_onpremise(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(_) => true,
            _ => false,
        }
    }

    pub fn is_online(&self) -> bool {
        match self.server_type {
            ExchangeServer::Online(_) => true,
            _ => false,
        }
    }

    pub fn is_ews(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2016(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2019(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            ExchangeServer::Online(ExchangeServerKind::ExchangeOnline(ref api)) => {
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
            ExchangeServer::Online(ExchangeServerKind::ExchangeOnline(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExchangeApiKind, ExchangeConnection, ExchangeServer, ExchangeServerKind};

    #[test]
    pub fn run_test_ews() {
        let exchange = ExchangeConnection::new(ExchangeServer::OnPremise(
            ExchangeServerKind::Exchange2016(vec![ExchangeApiKind::Ews]),
        ))
        .unwrap();

        assert_eq!(exchange.is_connected(), false);
        assert_eq!(exchange.is_disconnected(), true);
        assert_eq!(exchange.is_onpremise(), true);
        assert_eq!(exchange.is_online(), false);
        assert_eq!(exchange.is_ews(), true);
        assert_eq!(exchange.is_graph(), false);
    }

    #[test]
    pub fn run_test_online_graph() {
        let exchange = ExchangeConnection::new(ExchangeServer::Online(
            ExchangeServerKind::ExchangeOnline(vec![ExchangeApiKind::Graph]),
        ))
        .unwrap();

        assert_eq!(exchange.is_connected(), false);
        assert_eq!(exchange.is_disconnected(), true);
        assert_eq!(exchange.is_onpremise(), false);
        assert_eq!(exchange.is_online(), true);
        assert_eq!(exchange.is_ews(), false);
        assert_eq!(exchange.is_graph(), true);
    }
}
