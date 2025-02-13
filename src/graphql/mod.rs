use num_bigint::ParseBigIntError;

pub mod client_graph_node;
pub mod client_network;
pub mod client_registry;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error(transparent)]
    Transport(#[from] reqwest::Error),
    #[error("Failed to parse")]
    ParseBigIntError(#[from] ParseBigIntError),
    #[error("The subgraph is in a failed state")]
    IndexingError,
    #[error("Query response is unexpected: {0}")]
    ParseResponseError(String),
    #[error("Query response is empty: {0}")]
    PrometheusError(#[from] prometheus_http_query::Error),
    #[error("Unknown error: {0}")]
    Other(anyhow::Error),
}
