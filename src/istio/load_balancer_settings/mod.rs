pub mod consistent_hash_lb;

/// # ConsistentHashLB
/// Consistent Hash-based load balancing can be used to provide soft session affinity based on HTTP headers, cookies or other properties. The affinity to a particular destination host will be lost when one or more hosts are added/removed from the destination service.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConsistentHashLB {
    HttpHeaderName {
        // Hash based on a specific HTTP header.
        // Required: No
        #[serde(rename = "httpHeaderName")]
        http_header_name: Option<String>,

        // The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
        // No
        #[serde(rename = "minimumRingSize")]
        minimum_ring_size: Option<u64>,
    },
    HttpCookie {
        // Hash based on HTTP cookie.
        // No
        #[serde(rename = "httpCookie")]
        http_cookie: Option<consistent_hash_lb::HTTPCookie>,

        // The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
        // No
        #[serde(rename = "minimumRingSize")]
        minimum_ring_size: Option<u64>,
    },
    UseSourceIp {
        // Hash based on the source IP address. This is applicable for both TCP and HTTP connections.
        // No
        #[serde(rename = "useSourceIp")]
        use_source_ip: Option<bool>,

        // The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
        // No
        #[serde(rename = "minimumRingSize")]
        minimum_ring_size: Option<u64>,
    },
    HttpQueryParameterName {
        // Hash based on a specific HTTP query parameter.
        // No
        #[serde(rename = "httpQueryParameterName")]
        http_query_parameter_name: Option<String>,

        // The minimum number of virtual nodes to use for the hash ring. Defaults to 1024. Larger ring sizes result in more granular load distributions. If the number of hosts in the load balancing pool is larger than the ring size, each host will be assigned a single virtual node.
        // No
        #[serde(rename = "minimumRingSize")]
        minimum_ring_size: Option<u64>,
    },
}

/// # SimpleLB
/// Standard load balancing algorithms that require no tuning.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SimpleLB {
    // Round Robin policy. Default
    ROUND_ROBIN,

    // The least request load balancer uses an O(1) algorithm which selects two random healthy hosts and picks the host which has fewer active requests.
    LEAST_CONN,

    // The random load balancer selects a random healthy host. The random load balancer generally performs better than round robin if no health checking policy is configured.
    RANDOM,

    // This option will forward the connection to the original IP address requested by the caller without doing any form of load balancing. This option must be used with care. It is meant for advanced use cases.Refer to Original Destination load balancer in Envoy for further details.
    PASSTHROUGH,
}
