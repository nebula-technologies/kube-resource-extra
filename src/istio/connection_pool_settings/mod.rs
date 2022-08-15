use std::time::Duration;

pub mod http_settings;
pub mod tcp_settings;

/// # TCPSettings
/// Settings common to both HTTP and TCP upstream connections.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TCPSettings {
    // Maximum number of HTTP1 /TCP connections to a destination host. Default 2^32-1.
    // Required: No
    #[serde(rename = "maxConnections")]
    pub max_connections: Option<i32>,

    // TCP connection timeout. format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is 10s.
    // Required: No
    #[serde(rename = "connectTimeout")]
    pub connect_timeout: Option<Duration>,

    // If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
    // Required: No
    #[serde(rename = "tcpKeepalive")]
    pub tcp_keepalive: Option<tcp_settings::TcpKeepalive>,
}

/// #HTTPSettings
/// Settings applicable to HTTP1.1/HTTP2/GRPC connections.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HTTPSettings {
    // Maximum number of pending HTTP requests to a destination. Default 2^32-1.
    // No
    #[serde(rename = "http1MaxPendingRequests")]
    pub http1_max_pending_requests: Option<i32>,

    // Maximum number of requests to a backend. Default 2^32-1.
    // No
    #[serde(rename = "http2MaxRequests")]
    pub http2_max_requests: Option<i32>,

    // Maximum number of requests per connection to a backend. Setting this parameter to 1 disables keep alive. Default 0, meaning “unlimited”, up to 2^29.
    // No
    #[serde(rename = "maxRequestsPerConnection")]
    pub max_requests_per_connection: Option<i32>,

    // Maximum number of retries that can be outstanding to all hosts in a cluster at a given time. Defaults to 2^32-1.
    // No
    #[serde(rename = "maxRetries")]
    pub max_retries: Option<i32>,

    // The idle timeout for upstream connection pool connections. The idle timeout is defined as the period in which there are no active requests. If not set, the default is 1 hour. When the idle timeout is reached, the connection will be closed. If the connection is an HTTP/2 connection a drain sequence will occur prior to closing the connection. Note that request based timeouts mean that HTTP/2 PINGs will not keep the connection alive. Applies to both HTTP1.1 and HTTP2 connections.
    // No
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: Option<Duration>,

    // Specify if http1.1 connection should be upgraded to http2 for the associated destination.
    // No
    #[serde(rename = "h2UpgradePolicy")]
    pub h2_upgrade_policy: Option<http_settings::H2UpgradePolicy>,

    // If set to true, client protocol will be preserved while initiating connection to backend. Note that when this is set to true, h2upgradepolicy will be ineffective i.e. the client connections will not be upgraded to http2.
    // No
    #[serde(rename = "useClientProtocol")]
    pub use_client_protocol: Option<bool>,
}
