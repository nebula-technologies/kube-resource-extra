use crate::istio::destination_rule::{
    ClientTLSSettings, ConnectionPoolSettings, LoadBalancerSettings, OutlierDetection,
};
use crate::istio::virtual_service::PortSelector;

/// # PortTrafficPolicy
/// Traffic policies that apply to specific ports of the service
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortTrafficPolicy {
    // Specifies the number of a port on the destination service on which this policy is being applied.
    // No
    port: Option<PortSelector>,

    // Settings controlling the load balancer algorithms.
    // No
    loadBalancer: Option<LoadBalancerSettings>,

    // Settings controlling the volume of connections to an upstream service
    // No
    connectionPool: Option<ConnectionPoolSettings>,

    // Settings controlling eviction of unhealthy hosts from the load balancing pool
    // No
    outlierDetection: Option<OutlierDetection>,

    // TLS related settings for connections to the upstream service.
    // No
    tls: Option<ClientTLSSettings>,
}
