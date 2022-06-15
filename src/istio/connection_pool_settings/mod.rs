use std::time::Duration;

pub mod http_settings;
pub mod tcp_settings;

/// # TCPSettings
/// Settings common to both HTTP and TCP upstream connections.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TCPSettings {
    // Maximum number of HTTP1 /TCP connections to a destination host. Default 2^32-1.
    // No
    maxConnections: i32,

    // TCP connection timeout. format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is 10s.
    // No
    connectTimeout: Duration,

    // If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
    // No
    tcpKeepalive: tcp_settings::TcpKeepalive,
}

/// #
/// Settings applicable to HTTP1.1/HTTP2/GRPC connections.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HTTPSettings {
    // Maximum number of pending HTTP requests to a destination. Default 2^32-1.
    // No
    http1MaxPendingRequests: i32,

    // Maximum number of requests to a backend. Default 2^32-1.
    // No
    http2MaxRequests: i32,

    // Maximum number of requests per connection to a backend. Setting this parameter to 1 disables keep alive. Default 0, meaning “unlimited”, up to 2^29.
    // No
    maxRequestsPerConnection: i32,

    // Maximum number of retries that can be outstanding to all hosts in a cluster at a given time. Defaults to 2^32-1.
    // No
    maxRetries: i32,

    // The idle timeout for upstream connection pool connections. The idle timeout is defined as the period in which there are no active requests. If not set, the default is 1 hour. When the idle timeout is reached, the connection will be closed. If the connection is an HTTP/2 connection a drain sequence will occur prior to closing the connection. Note that request based timeouts mean that HTTP/2 PINGs will not keep the connection alive. Applies to both HTTP1.1 and HTTP2 connections.
    // No
    idleTimeout: Duration,

    // Specify if http1.1 connection should be upgraded to http2 for the associated destination.
    // No
    h2UpgradePolicy: http_settings::H2UpgradePolicy,

    // If set to true, client protocol will be preserved while initiating connection to backend. Note that when this is set to true, h2upgradepolicy will be ineffective i.e. the client connections will not be upgraded to http2.
    // No
    useClientProtocol: bool,
}
