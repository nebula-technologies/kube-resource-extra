use std::time::Duration;

/// # TcpKeepalive
/// TCP keepalive.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TcpKeepalive {
    probes: u32,

    // Maximum number of keepalive probes to send without response before deciding the connection is dead. Default is to use the OS level configuration (unless overridden, Linux defaults to 9.)
    // No
    time: Duration,

    // The time duration a connection needs to be idle before keep-alive probes start being sent. Default is to use the OS level configuration (unless overridden, Linux defaults to 7200s (ie 2 hours.)
    // No
    interval: Duration,
    // The time duration between keep-alive probes. Default is to use the OS level configuration (unless overridden, Linux defaults to 75s.)
    // No
}
