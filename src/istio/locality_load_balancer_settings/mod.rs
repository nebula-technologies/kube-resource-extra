use std::collections::HashMap;

/// # Distribute
/// Describes how traffic originating in the ‘from’ zone or sub-zone is distributed over a set of ‘to’ zones. Syntax for specifying a zone is {region}/{zone}/{sub-zone} and terminal wildcards are allowed on any segment of the specification. Examples:
///
/// `*` - matches all localities
///
/// `us-west/*` - all zones and sub-zones within the us-west region
///
/// `us-west/zone-1/*` - all sub-zones within us-west/zone-1
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Distribute {
    // Originating locality, `/` separated, e.g.`region / zone / sub_zone`.
    // Required: No
    from: String,

    // Map of upstream localities to traffic distribution weights.The sum of all weights should be 100.Any locality not present will receive no traffic.
    // Required: No
    to: HashMap<String, u32>,
}

/// # Failover
/// Specify the traffic failover policy across regions. Since zone and sub-zone failover is supported by default this only needs to be specified for regions when the operator needs to constrain traffic failover so that the default behavior of failing over to any endpoint globally does not apply. This is useful when failing over traffic across regions would not improve service health or may need to be restricted for other reasons like regulatory controls.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Failover {
    // Originating region.
    // No
    from: String,

    // Destination region the traffic will fail over to when endpoints in the ‘from’ region becomes unhealthy.
    // No
    to: String,
}
