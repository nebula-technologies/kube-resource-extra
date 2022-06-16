use std::time::Duration;

/// # HTTPCookie
/// Describes a HTTP cookie that will be used as the hash key for the Consistent Hash load balancer. If the cookie is not present, it will be generated.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HTTPCookie {
    // Name of the cookie.
    // Yes
    pub name: String,

    // Path to set for the cookie.
    // No
    pub path: Option<String>,

    // Lifetime of the cookie.
    // Yes
    pub ttl: Duration,
}
