use crate::istio::load_balancer_settings::{ConsistentHashLB, SimpleLB};
use crate::istio::traffic_policy::PortTrafficPolicy;
use k8s_openapi::{Metadata, Resource};
/// # Destination Rule
/// DestinationRule defines policies that apply to traffic intended for a service after routing has occurred. These rules specify configuration for load balancing, connection pool size from the sidecar, and outlier detection settings to detect and evict unhealthy hosts from the load balancing pool. For example, a simple load balancing policy for the ratings service would look as follows:
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     loadBalancer:
///       simple: LEAST_CONN
/// ```
/// Version specific policies can be specified by defining a named subset and overriding the settings specified at the service level. The following rule uses a round robin load balancing policy for all traffic going to a subset named testversion that is composed of endpoints (e.g., pods) with labels (version:v3).
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     loadBalancer:
///       simple: LEAST_CONN
///   subsets:
///   - name: testversion
///     labels:
///       version: v3
///     trafficPolicy:
///       loadBalancer:
///         simple: ROUND_ROBIN
/// ```
/// Note: Policies specified for subsets will not take effect until a route rule explicitly sends traffic to this subset.
///
/// Traffic policies can be customized to specific ports as well. The following rule uses the least connection load balancing policy for all traffic to port 80, while uses a round robin load balancing setting for traffic to the port 9080.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings-port
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy: # Apply to all ports
///     portLevelSettings:
///     - port:
///         number: 80
///       loadBalancer:
///         simple: LEAST_CONN
///     - port:
///         number: 9080
///       loadBalancer:
///         simple: ROUND_ROBIN
/// ```
use std::collections::HashMap;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DestinationRule {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<DestinationRuleSpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<()>,
}

impl Resource for DestinationRule {
    const API_VERSION: &'static str = "networking.istio.io/v1beta1";
    const GROUP: &'static str = "networking.istio.io";
    const KIND: &'static str = "DestinationRule";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "destinationrules";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl Metadata for DestinationRule {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    fn metadata(&self) -> &<Self as Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as Metadata>::Ty {
        &mut self.metadata
    }
}

/// # DestinationRuleSpec
/// DestinationRule defines policies that apply to traffic intended for a service after routing has occurred.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DestinationRuleSpec {
    // The name of a service from the service registry.Service names are looked up from the platform’s service registry (e.g.,
    // Kubernetes services,
    // Consul services,
    // etc.) and from the hosts declared by ServiceEntries.Rules defined for services that do not exist in the service registry will be ignored.
    //
    // Note for Kubernetes users: When short names are used (e.g.“reviews” instead of “reviews.default.svc.cluster.local”),
    // Istio will interpret the short name based on the namespace of the rule,
    // not the service.A rule in the “default” namespace containing a host “reviews” will be interpreted as “reviews.default.svc.cluster.local”,
    // irrespective of the actual namespace associated with the reviews service.To avoid potential misconfigurations,
    // it is recommended to always use fully qualified domain names over short names.
    //
    // Note that the host field applies to both HTTP and TCP services.
    // Required: Yes
    pub host: String,

    // Traffic policies to apply (load balancing policy,
    // connection pool sizes,
    // outlier detection).
    // Required: No
    #[serde(rename = "trafficPolicy")]
    pub traffic_policy: TrafficPolicy,

    // One or more named sets that represent individual versions of a service.Traffic policies can be overridden at subset level.
    // Required: No
    pub subsets: Option<Vec<Subset>>,

    // A list of namespaces to which this destination rule is exported.The resolution of a destination rule to apply to a service occurs in the context of a hierarchy of namespaces.Exporting a destination rule allows it to be included in the resolution hierarchy for services in other namespaces.This feature provides a mechanism for service owners and mesh administrators to control the visibility of destination rules across namespace boundaries.
    //
    // If no namespaces are specified then the destination rule is exported to all namespaces by default.
    //
    // The value “.” is reserved and defines an export to the same namespace that the destination rule is declared in.Similarly,
    // the value “ * ” is reserved and defines an export to all namespaces.
    // Required: No
    #[serde(rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
}

/// # TrafficPolicy
///
/// Traffic policies to apply for a specific destination, across all destination ports. See DestinationRule for examples.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficPolicy {
    // Settings controlling the load balancer algorithms.
    // Required: No
    #[serde(rename = "loadBalancer")]
    pub load_balancer: Option<LoadBalancerSettings>,

    // Settings controlling the volume of connections to an upstream service
    // Required: No
    #[serde(rename = "connectionPool")]
    pub connection_pool: Option<ConnectionPoolSettings>,

    // Settings controlling eviction of unhealthy hosts from the load balancing pool
    // Required: No
    #[serde(rename = "outlierDetection")]
    pub outlier_detection: Option<OutlierDetection>,

    // TLS related settings for connections to the upstream service.
    // Required: No
    pub tls: Option<ClientTLSSettings>,

    // Traffic policies specific to individual ports.Note that port level settings will override the destination - level settings.Traffic settings specified at the destination - level will not be inherited when overridden by port - level settings,
    // i.e.default values will be applied to fields omitted in port - level traffic policies.
    // Required: No
    #[serde(rename = "portLevelSettings")]
    pub port_level_settings: Option<Vec<PortTrafficPolicy>>,
}

/// # Subset
/// A subset of endpoints of a service. Subsets can be used for scenarios like A/B testing, or routing to a specific version of a service. Refer to VirtualService documentation for examples of using subsets in these scenarios. In addition, traffic policies defined at the service-level can be overridden at a subset-level. The following rule uses a round robin load balancing policy for all traffic going to a subset named testversion that is composed of endpoints (e.g., pods) with labels (version:v3).
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     loadBalancer:
///       simple: LEAST_CONN
///   subsets:
///   - name: testversion
///     labels:
///       version: v3
///     trafficPolicy:
///       loadBalancer:
///         simple: ROUND_ROBIN
/// ```
/// Note: Policies specified for subsets will not take effect until a route rule explicitly sends traffic to this subset.
///
/// One or more labels are typically required to identify the subset destination, however, when the corresponding DestinationRule represents a host that supports multiple SNI hosts (e.g., an egress gateway), a subset without labels may be meaningful. In this case a traffic policy with ClientTLSSettings can be used to identify a specific SNI host corresponding to the named subset.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subset {
    // Name of the subset.The service name and the subset name can be used for traffic splitting in a route rule.
    // Yes
    pub name: String,

    // Labels apply a filter over the endpoints of a service in the service registry.See route rules for examples of usage.
    // No
    pub labels: HashMap<String, String>,

    // Traffic policies that apply to this subset.Subsets inherit the traffic policies specified at the DestinationRule level.Settings specified at the subset level will override the corresponding settings specified at the DestinationRule level.
    // No
    #[serde(rename = "trafficPolicy")]
    pub traffic_policy: TrafficPolicy,
}

/// # LoadBalancerSettings
/// Load balancing policies to apply for a specific destination. See Envoy’s load balancing documentation for more details.
///
/// For example, the following rule uses a round robin load balancing policy for all traffic going to the ratings service.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     loadBalancer:
///       simple: ROUND_ROBIN
/// ```
/// The following example sets up sticky sessions for the ratings service hashing-based load balancer for the same ratings service using the the User cookie as the hash key.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-ratings
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     loadBalancer:
///       consistentHash:
///         httpCookie:
///           name: user
///           ttl: 0s
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LoadBalancerSettings {
    Simple {
        simple: SimpleLB,
        // Locality load balancer settings,
        // this will override mesh wide settings in entirety,
        // meaning no merging would be performed between this object and the object one in MeshConfig
        // Required: No
        localityLbSetting: LocalityLoadBalancerSetting,
    },
    ConsistentHash {
        consistentHash: ConsistentHashLB,
        // Locality load balancer settings,
        // this will override mesh wide settings in entirety,
        // meaning no merging would be performed between this object and the object one in MeshConfig
        // Required: No
        localityLbSetting: LocalityLoadBalancerSetting,
    },
}

/// # ConnectionPoolSettings
/// Connection pool settings for an upstream host. The settings apply to each individual host in the upstream service. See Envoy’s circuit breaker for more details. Connection pool settings can be applied at the TCP level as well as at HTTP level.
///
/// For example, the following rule sets a limit of 100 connections to redis service called myredissrv with a connect timeout of 30ms
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: bookinfo-redis
/// spec:
///   host: myredissrv.prod.svc.cluster.local
///   trafficPolicy:
///     connectionPool:
///       tcp:
///         maxConnections: 100
///         connectTimeout: 30ms
///         tcpKeepalive:
///           time: 7200s
///           interval: 75s
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectionPoolSettings {
    // Settings common to both HTTP and TCP upstream connections.
    // Required: No
    pub tcp: Option<super::connection_pool_settings::TCPSettings>,

    // HTTP connection pool settings.
    // Required: No
    pub http: Option<super::connection_pool_settings::HTTPSettings>,
}

/// # OutlierDetection
/// A Circuit breaker implementation that tracks the status of each individual host in the upstream service. Applicable to both HTTP and TCP services. For HTTP services, hosts that continually return 5xx errors for API calls are ejected from the pool for a pre-defined period of time. For TCP services, connection timeouts or connection failures to a given host counts as an error when measuring the consecutive errors metric. See Envoy’s outlier detection for more details.
///
/// The following rule sets a connection pool size of 100 HTTP1 connections with no more than 10 req/connection to the “reviews” service. In addition, it sets a limit of 1000 concurrent HTTP2 requests and configures upstream hosts to be scanned every 5 mins so that any host that fails 7 consecutive times with a 502, 503, or 504 error code will be ejected for 15 minutes.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: reviews-cb-policy
/// spec:
///   host: reviews.prod.svc.cluster.local
///   trafficPolicy:
///     connectionPool:
///       tcp:
///         maxConnections: 100
///       http:
///         http2MaxRequests: 1000
///         maxRequestsPerConnection: 10
///     outlierDetection:
///       consecutive5xxErrors: 7
///       interval: 5m
///       baseEjectionTime: 15m
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutlierDetection {
    // Determines whether to distinguish local origin failures from external errors.If set to true consecutivelocalorigin_failure is taken into account for outlier detection calculations.This should be used when you want to derive the outlier detection status based on the errors seen locally such as failure to connect,
    // timeout while connecting etc.rather than the status code retuned by upstream service.This is especially useful when the upstream service explicitly returns a 5xx for some requests and you want to ignore those responses from upstream service while determining the outlier detection status of a host.Defaults to false.
    // No
    #[serde(rename = "splitExternalLocalOriginErrors")]
    pub split_external_local_origin_errors: Option<bool>,

    // The number of consecutive locally originated failures before ejection occurs.Defaults to 5.Parameter takes effect only when splitexternallocaloriginerrors is set to true.
    // No
    #[serde(rename = "consecutiveLocalOriginFailures")]
    pub consecutive_local_origin_failures: Option<super::google::protobuf::UInt32Value>,

    // Number of gateway errors before a host is ejected from the connection pool.When the upstream host is accessed over HTTP,
    // a 502, 503, or 504 return code qualifies as a gateway error.When the upstream host is accessed over an opaque TCP connection,
    // connect timeouts and connection error / failure events qualify as a gateway error.This feature is disabled by default or when set to the value 0.
    //
    // Note that consecutivegatewayerrors and consecutive5xxerrors can be used separately or together.Because the errors counted by consecutivegatewayerrors are also included in consecutive5xxerrors,
    // if the value of consecutivegatewayerrors is greater than or equal to the value of consecutive5xxerrors,
    // consecutivegatewayerrors will have no effect.
    // No
    #[serde(rename = "consecutiveGatewayErrors")]
    pub consecutive_gateway_errors: Option<super::google::protobuf::UInt32Value>,

    // Number of 5xx errors before a host is ejected from the connection pool.When the upstream host is accessed over an opaque TCP connection,
    // connect timeouts,
    // connection error / failure and request failure events qualify as a 5xx error.This feature defaults to 5 but can be disabled by setting the value to 0.
    //
    // Note that consecutivegatewayerrors and consecutive5xxerrors can be used separately or together.Because the errors counted by consecutivegatewayerrors are also included in consecutive5xxerrors,
    // if the value of consecutivegatewayerrors is greater than or equal to the value of consecutive5xxerrors,
    // consecutivegatewayerrors will have no effect.
    // No
    #[serde(rename = "consecutive5xxErrors")]
    pub consecutive5xx_errors: Option<super::google::protobuf::UInt32Value>,

    // Time interval between ejection sweep analysis.format: 1h / 1m / 1s / 1ms.MUST BE > = 1ms.Default is 10s.
    // No
    pub interval: Option<Duration>,

    // Minimum ejection duration.A host will remain ejected for a period equal to the product of minimum ejection duration and the number of times the host has been ejected.This technique allows the system to automatically increase the ejection period for unhealthy upstream servers.format: 1h / 1m / 1s / 1ms.MUST BE > = 1ms.Default is 30s.
    // No
    #[serde(rename = "baseEjectionTime")]
    pub base_ejection_time: Option<Duration>,

    // Maximum % of hosts in the load balancing pool for the upstream service that can be ejected.Defaults to 10 %.
    // No
    #[serde(rename = "maxEjectionPercent")]
    pub max_ejection_percent: Option<i32>,

    // Outlier detection will be enabled as long as the associated load balancing pool has at least minhealthpercent hosts in healthy mode.When the percentage of healthy hosts in the load balancing pool drops below this threshold,
    // outlier detection will be disabled and the proxy will load balance across all hosts in the pool (healthy and unhealthy).The threshold can be disabled by setting it to 0 %.The default is 0 % as it’s not typically applicable in k8s environments with few pods per service.
    // No
    #[serde(rename = "minHealthPercent")]
    pub min_health_percent: Option<i32>,
}

/// # ClientTLSSettings
/// SSL/TLS related settings for upstream connections. See Envoy’s TLS context for more details. These settings are common to both HTTP and TCP upstreams.
///
/// For example, the following rule configures a client to use mutual TLS for connections to upstream database cluster.
///
/// apiVersion: networking.istio.io/v1beta1
/// ```yaml
/// kind: DestinationRule
/// metadata:
///   name: db-mtls
/// spec:
///   host: mydbserver.prod.svc.cluster.local
///   trafficPolicy:
///     tls:
///       mode: MUTUAL
///       clientCertificate: /etc/certs/myclientcert.pem
///       privateKey: /etc/certs/client_private_key.pem
///       caCertificates: /etc/certs/rootcacerts.pem
/// ```
/// The following rule configures a client to use TLS when talking to a foreign service whose domain matches *.foo.com.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: tls-foo
/// spec:
///   host: "*.foo.com"
///   trafficPolicy:
///     tls:
///       mode: SIMPLE
/// ```
///
/// The following rule configures a client to use Istio mutual TLS when talking to rating services.
/// ```yaml
/// apiVersion: networking.istio.io/v1beta1
/// kind: DestinationRule
/// metadata:
///   name: ratings-istio-mtls
/// spec:
///   host: ratings.prod.svc.cluster.local
///   trafficPolicy:
///     tls:
///       mode: ISTIO_MUTUAL
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientTLSSettings {
    // Indicates whether connections to this port should be secured using TLS.The value of this field determines how TLS is enforced.
    // Yes
    pub mode: super::client_tls_settings::TLSmode,

    // REQUIRED if mode is MUTUAL.The path to the file holding the client - side TLS certificate to use.Should be empty if mode is ISTIO_MUTUAL.
    // No
    #[serde(rename = "clientCertificate")]
    pub client_certificate: Option<String>,

    // REQUIRED if mode is MUTUAL.The path to the file holding the client’s private key.Should be empty if mode is ISTIO_MUTUAL.
    // No
    #[serde(rename = "privateKey")]
    pub private_key: Option<String>,

    // OPTIONAL: The path to the file containing certificate authority certificates to use in verifying a presented server certificate.If omitted,
    // the proxy will not verify the server’s certificate.Should be empty if mode is ISTIO_MUTUAL.
    // No
    #[serde(rename = "caCertificates")]
    pub ca_certificates: Option<String>,

    // The name of the secret that holds the TLS certs for the client including the CA certificates.Secret must exist in the same namespace with the proxy using the certificates.The secret (of type generic)should contain the following keys and values: key: < privateKey >,
    // cert: < clientCert >,
    // cacert: < CACertificate >.Here CACertificate is used to verify the server certificate.Secret of type tls for client certificates along with ca.crt key for CA certificates is also supported.Only one of client certificates and CA certificate or credentialName can be specified.
    //
    // NOTE: This field is currently applicable only at gateways.Sidecars will continue to use the certificate paths.
    // No
    #[serde(rename = "credentialName")]
    pub credential_name: Option<String>,

    // A list of alternate names to verify the subject identity in the certificate.If specified,
    // the proxy will verify that the server certificate’s subject alt name matches one of the specified values.If specified,
    // this list overrides the value of subjectaltnames from the ServiceEntry.
    // No
    #[serde(rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,

    // SNI string to present to the server during TLS handshake.
    // No
    pub sni: Option<String>,

    // InsecureSkipVerify specifies whether the proxy should skip verifying the CA signature and SAN for the server certificate corresponding to the host.This flag should only be set if global CA signature verifcation is enabled,
    // VerifyCertAtClient environmental variable is set to true,
    // but no verification is desired for a specific host.If enabled with or without VerifyCertAtClient enabled,
    // verification of the CA signature and SAN will be skipped.
    //
    // InsecureSkipVerify is false by default.VerifyCertAtClient is false by default in Istio version 1.9 but will be true by default in a later version where,
    // going forward,
    // it will be enabled by default.
    // Required: No
    #[serde(rename = "insecureSkipVerify")]
    pub insecure_skip_verify: Option<bool>,
}

/// # LocalityLoadBalancerSetting
/// Locality-weighted load balancing allows administrators to control the distribution of traffic to endpoints based on the localities of where the traffic originates and where it will terminate. These localities are specified using arbitrary labels that designate a hierarchy of localities in {region}/{zone}/{sub-zone} form. For additional detail refer to Locality Weight The following example shows how to setup locality weights mesh-wide.
///
/// Given a mesh with workloads and their service deployed to “us-west/zone1/” and “us-west/zone2/”. This example specifies that when traffic accessing a service originates from workloads in “us-west/zone1/”, 80% of the traffic will be sent to endpoints in “us-west/zone1/”, i.e the same zone, and the remaining 20% will go to endpoints in “us-west/zone2/”. This setup is intended to favor routing traffic to endpoints in the same locality. A similar setting is specified for traffic originating in “us-west/zone2/”.
/// ```yaml
///   distribute:
///     - from: us-west/zone1/*
///       to:
///         "us-west/zone1/*": 80
///         "us-west/zone2/*": 20
///     - from: us-west/zone2/*
///       to:
///         "us-west/zone1/*": 20
///         "us-west/zone2/*": 80
/// ```
/// If the goal of the operator is not to distribute load across zones and regions but rather to restrict the regionality of failover to meet other operational requirements an operator can set a ‘failover’ policy instead of a ‘distribute’ policy.
///
/// The following example sets up a locality failover policy for regions. Assume a service resides in zones within us-east, us-west & eu-west this example specifies that when endpoints within us-east become unhealthy traffic should failover to endpoints in any zone or sub-zone within eu-west and similarly us-west should failover to us-east.
/// ```yaml
///  failover:
///    - from: us-east
///      to: eu-west
///    - from: us-west
///      to: us-east
/// ```
/// Locality load balancing settings.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocalityLoadBalancerSetting {
    // Optional: only one of distribute, failover or failoverPriority can be set. Explicitly specify loadbalancing weight across different zones and geographical locations. Refer to Locality weighted load balancing If empty, the locality weight is set according to the endpoints number within it.
    // Required: No
    pub distribute: Option<Vec<super::locality_load_balancer_settings::Distribute>>,

    // Optional: only one of distribute, failover or failoverPriority can be set. Explicitly specify the region traffic will land on when endpoints in local region becomes unhealthy. Should be used together with OutlierDetection to detect unhealthy endpoints. Note: if no OutlierDetection specified, this will not take effect.
    // Required: No
    pub failover: Option<Vec<super::locality_load_balancer_settings::Failover>>,

    // failoverPriority is an ordered list of labels used to sort endpoints to do priority based load balancing. This is to support traffic failover across different groups of endpoints. Suppose there are total N labels specified:
    //
    //     Endpoints matching all N labels with the client proxy have priority P(0) i.e. the highest priority.
    //     Endpoints matching the first N-1 labels with the client proxy have priority P(1) i.e. second highest priority.
    //     By extension of this logic, endpoints matching only the first label with the client proxy has priority P(N-1) i.e. second lowest priority.
    //     All the other endpoints have priority P(N) i.e. lowest priority.
    //
    // Note: For a label to be considered for match, the previous labels must match, i.e. nth label would be considered matched only if first n-1 labels match.
    //
    // It can be any label specified on both client and server workloads. The following labels which have special semantic meaning are also supported:
    //
    //     topology.istio.io/network is used to match the network metadata of an endpoint, which can be specified by pod/namespace label topology.istio.io/network, sidecar env ISTIO_META_NETWORK or MeshNetworks.
    //     topology.istio.io/cluster is used to match the clusterID of an endpoint, which can be specified by pod label topology.istio.io/cluster or pod env ISTIO_META_CLUSTER_ID.
    //     topology.kubernetes.io/region is used to match the region metadata of an endpoint, which maps to Kubernetes node label topology.kubernetes.io/region or the deprecated label failure-domain.beta.kubernetes.io/region.
    //     topology.kubernetes.io/zone is used to match the zone metadata of an endpoint, which maps to Kubernetes node label topology.kubernetes.io/zone or the deprecated label failure-domain.beta.kubernetes.io/zone.
    //     topology.istio.io/subzone is used to match the subzone metadata of an endpoint, which maps to Istio node label topology.istio.io/subzone.
    //
    // The below topology config indicates the following priority levels:
    // ```yaml
    // failoverPriority:
    // - "topology.istio.io/network"
    // - "topology.kubernetes.io/region"
    // - "topology.kubernetes.io/zone"
    // - "topology.istio.io/subzone"
    // ```
    //     endpoints match same [network, region, zone, subzone] label with the client proxy have the highest priority.
    //     endpoints have same [network, region, zone] label but different [subzone] label with the client proxy have the second highest priority.
    //     endpoints have same [network, region] label but different [zone] label with the client proxy have the third highest priority.
    //     endpoints have same [network] but different [region] labels with the client proxy have the fourth highest priority.
    //     all the other endpoints have the same lowest priority.
    //
    // Optional: only one of distribute, failover or failoverPriority can be set. And it should be used together with OutlierDetection to detect unhealthy endpoints, otherwise has no effect.
    // Required: No
    #[serde(rename = "failoverPriority")]
    pub failover_priority: Option<Vec<String>>,

    // enable locality load balancing, this is DestinationRule-level and will override mesh wide settings in entirety. e.g. true means that turn on locality load balancing for this DestinationRule no matter what mesh wide settings is.
    // Required: No
    pub enabled: Option<bool>,
}
