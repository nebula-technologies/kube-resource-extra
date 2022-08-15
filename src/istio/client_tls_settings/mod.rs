/// # TLSMode
/// TLS connection mode
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TLSmode {
    // Do not setup a TLS connection to the upstream endpoint.
    DISABLE,

    // Originate a TLS connection to the upstream endpoint.
    SIMPLE,

    // Secure connections to the upstream using mutual TLS by presenting client certificates for authentication.
    MUTUAL,

    // Secure connections to the upstream using mutual TLS by presenting client certificates for authentication. Compared to Mutual mode, this mode uses certificates generated automatically by Istio for mTLS authentication. When this mode is used, all other fields in ClientTLSSettings should be empty.
    ISTIO_MUTUAL,
}
