/// # H2UpgradePolicy
/// Policy for upgrading http1.1 connections to http2.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum H2UpgradePolicy {
    // Use the global default.
    DEFAULT,

    // Do not upgrade the connection to http2. This opt-out option overrides the default.
    DO_NOT_UPGRADE,

    // Upgrade the connection to http2. This opt- in option overrides the default.
    UPGRADE,
}
