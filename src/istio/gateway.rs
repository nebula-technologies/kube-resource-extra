use k8s_openapi::{Metadata, Resource};
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Gateway {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<GatewaySpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<()>,
}

impl Resource for Gateway {
    const API_VERSION: &'static str = "networking.istio.io/v1beta1";
    const GROUP: &'static str = "networking.istio.io";
    const KIND: &'static str = "Gateway";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "gateways";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl Metadata for Gateway {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as Metadata>::Ty {
        &mut self.metadata
    }
}

/// # Gateway
///
/// Gateway describes a load balancer operating at the edge of the mesh receiving incoming or outgoing HTTP/TCP connections.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewaySpec {
    /// A list of server specifications.
    /// Required: Yes
    pub servers: Vec<Server>,
    /// One or more labels that indicate a specific set of pods/VMs on which this gateway
    /// configuration should be applied. By default workloads are searched across all namespaces
    /// based on label selectors. This implies that a gateway resource in the namespace “foo” can
    /// select pods in the namespace “bar” based on labels. This behavior can be controlled via
    /// the PILOT_SCOPE_GATEWAY_TO_NAMESPACE environment variable in istiod. If this variable is
    /// set to true, the scope of label search is restricted to the configuration namespace in
    /// which the the resource is present. In other words, the Gateway resource must reside in the
    /// same namespace as the gateway workload instance. If selector is nil, the Gateway will be
    /// applied to all workloads.
    /// Required: Yes
    pub selector: HashMap<String, String>,
}

/// # Server
///
/// Server describes the properties of the proxy on a given load balancer port. For example,
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    /// The Port on which the proxy should listen for incoming connections.
    /// Required: Yes
    pub port: Port,

    /// The ip or the Unix domain socket to which the listener should be bound to.
    /// Format: x.x.x.x or unix:///path/to/uds or unix://@foobar (Linux abstract namespace). When
    /// using Unix domain sockets, the port number should be 0. This can be used to restrict the
    /// reachability of this server to be gateway internal only. This is typically used when a
    /// gateway needs to communicate to another mesh service e.g. publishing metrics. In such case,
    /// the server created with the specified bind will not be available to external gateway clients.
    /// Required: No
    pub bind: Option<String>,

    /// One or more hosts exposed by this gateway. While typically applicable to HTTP services,
    /// it can also be used for TCP services using TLS with SNI. A host is specified as a
    /// dnsName with an optional namespace/ prefix. The dnsName should be specified using FQDN
    /// format, optionally including a wildcard character in the left-most component
    /// (e.g., prod/*.example.com). Set the dnsName to * to select all VirtualService hosts from
    /// the specified namespace (e.g.,prod/*).
    ///
    /// The namespace can be set to * or ., representing any or the current namespace, respectively.
    /// For example, */foo.example.com selects the service from any available namespace while
    /// ./foo.example.com only selects the service from the namespace of the sidecar. The default,
    /// if no namespace/ is specified, is */, that is, select services from any namespace.
    /// Any associated DestinationRule in the selected namespace will also be used.
    ///
    /// A VirtualService must be bound to the gateway and must have one or more hosts that match
    /// the hosts specified in a server. The match could be an exact match or a suffix match with
    /// the server’s hosts. For example, if the server’s hosts specifies *.example.com, a
    /// VirtualService with hosts dev.example.com or prod.example.com will match. However, a
    /// VirtualService with host example.com or newexample.com will not match.
    ///
    /// NOTE: Only virtual services exported to the gateway’s namespace (e.g., exportTo value of *)
    /// can be referenced. Private configurations (e.g., exportTo set to .) will not be available.
    /// Refer to the exportTo setting in VirtualService, DestinationRule, and ServiceEntry
    /// configurations for details.
    /// Required: Yes
    pub hosts: Vec<String>,

    /// Set of TLS related options that govern the server’s behavior. Use these options to control
    /// if all http requests should be redirected to https, and the TLS modes to use.
    /// Required: No
    pub tls: Option<ServerTLSSettings>,

    /// An optional name of the server, when set must be unique across all servers. This will be
    /// used for variety of purposes like prefixing stats generated with this name etc.
    /// Required: No
    pub name: Option<String>,
}

/// # Port
/// Port describes the properties of a specific port of a service.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Port {
    /// A valid non-negative integer port number.
    /// Required: Yes
    pub number: i32,

    /// The protocol exposed on the port. MUST BE one of HTTP|HTTPS|GRPC|HTTP2|MONGO|TCP|TLS. TLS implies the connection will be routed based on the SNI header to the destination without terminating the TLS connection.
    /// Required: Yes
    pub protocol: String,

    /// Label assigned to the port.
    /// Required: Yes
    pub name: String,

    /// The port number on the endpoint where the traffic will be received. Applicable only when used with ServiceEntries.
    /// Required: No
    #[serde(rename = "targetPort")]
    pub target_port: Option<u32>,
}

/// # ServerTLSSettings
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerTLSSettings {
    /// If set to true, the load balancer will send a 301 redirect for all http connections, asking the clients to use HTTPS.
    /// Required: No
    #[serde(rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,

    /// Optional: Indicates whether connections to this port should be secured using TLS.The value of this field determines how TLS is enforced.
    /// No
    pub mode: Option<TLSmode>,

    /// REQUIRED if mode is SIMPLE or MUTUAL.The path to the file holding the server - side TLS certificate to use.
    /// No
    #[serde(rename = "serverCertificate")]
    pub server_certificate: Option<String>,

    /// REQUIRED if mode is SIMPLE or MUTUAL.The path to the file holding the server’s private key.
    // Required: No
    #[serde(rename = "privateKey")]
    pub private_key: Option<String>,

    /// REQUIRED if mode is MUTUAL.The path to a file containing certificate authority certificates to use in verifying a presented client side certificate.
    /// Required: No
    #[serde(rename = "caCertificates")]
    pub ca_certificates: Option<String>,

    /// For gateways running on Kubernetes, the name of the secret that holds the TLS certs including the CA certificates.Applicable only on Kubernetes.The secret (of type generic) should contain the following keys and values: key: < privateKey > and cert: < serverCert >.For mutual TLS,
    /// cacert: < CACertificate > can be provided in the same secret or a separate secret named < secret > - cacert.Secret of type tls for server certificates along with ca.crt key for CA certificates is also supported.Only one of server certificates and CA certificate or credentialName can be specified.
    /// Required: No
    #[serde(rename = "credentialName")]
    pub credential_name: Option<String>,

    /// A list of alternate names to verify the subject identity in the certificate presented by the client.
    /// Required: No
    #[serde(rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,

    /// An optional list of base64 - encoded SHA - 256 hashes of the SKPIs of authorized client certificates.Note: When both verifycertificatehash and verifycertificatespki are specified,
    /// a hash matching either value will result in the certificate being accepted.
    /// Required: No
    #[serde(rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,

    /// An optional list of hex - encoded SHA - 256 hashes of the authorized client certificates.Both simple and colon separated formats are acceptable.Note: When both verifycertificatehash and verifycertificatespki are specified,
    /// a hash matching either value will result in the certificate being accepted.
    /// Required: No
    #[serde(rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,

    /// Optional: Minimum TLS protocol version.
    /// Required: No
    #[serde(rename = "minProtocolVersion")]
    pub min_protocol_version: Option<TLSProtocol>,

    /// Optional: Maximum TLS protocol version.
    /// Required: No
    #[serde(rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<TLSProtocol>,

    /// Optional: If specified,
    /// only support the specified cipher list.Otherwise default to the default cipher list supported by Envoy.
    /// Required: No
    #[serde(rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
}

/// # ServerTLSSettings.TLSmode
/// TLS modes enforced by the proxy
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TLSmode {
    /// The SNI string presented by the client will be used as the match criterion in a VirtualService TLS route to determine the destination service from the service registry.
    PASSTHROUGH,

    /// Secure connections with standard TLS semantics.
    SIMPLE,

    /// Secure connections to the downstream using mutual TLS by presenting server certificates for authentication.
    MUTUAL,

    /// Similar to the passthrough mode, except servers with this TLS mode do not require an associated VirtualService to map from the SNI value to service in the registry. The destination details such as the service/subset/port are encoded in the SNI value. The proxy will forward to the upstream (Envoy) cluster (a group of endpoints) specified by the SNI value. This server is typically used to provide connectivity between services in disparate L3 networks that otherwise do not have direct connectivity between their respective endpoints. Use of this mode assumes that both the source and the destination are using Istio mTLS to secure traffic.
    AUTO_PASSTHROUGH,

    /// Secure connections from the downstream using mutual TLS by presenting server certificates for authentication. Compared to Mutual mode, this mode uses certificates, representing gateway workload identity, generated automatically by Istio for mTLS authentication. When this mode is used, all other fields in TLSOptions should be empty.
    ISTIO_MUTUAL,
}

/// # ServerTLSSettings.TLSProtocol
/// TLS protocol versions.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TLSProtocol {
    /// Automatically choose the optimal TLS version.
    TLS_AUTO,

    /// TLS version 1.0
    TLSV1_0,

    /// TLS version 1.1
    TLSV1_1,

    /// TLS version 1.2
    TLSV1_2,

    /// TLS version 1.3
    TLSV1_3,
}
