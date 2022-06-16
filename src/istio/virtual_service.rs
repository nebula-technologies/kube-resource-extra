use k8s_openapi::{Metadata, Resource};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualService {
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,

    /// Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub spec: Option<VirtualServiceSpec>,

    /// Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    pub status: Option<()>,
}

impl Resource for VirtualService {
    const API_VERSION: &'static str = "networking.istio.io/v1beta1";
    const GROUP: &'static str = "networking.istio.io";
    const KIND: &'static str = "VirtualService";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "virtualservices";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl Metadata for VirtualService {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as Metadata>::Ty {
        &mut self.metadata
    }
}

/// # Virtual Service
/// Configuration affecting traffic routing. Here are a few terms useful to define in the context
/// of traffic routing.
///
/// `Service` a unit of application behavior bound to a unique name in a service registry. Services
/// consist of multiple network endpoints implemented by workload instances running on pods,
/// containers, VMs etc.
///
/// `Service versions (a.k.a. subsets)` - In a continuous deployment scenario, for a given service,
/// there can be distinct subsets of instances running different variants of the application binary.
/// These variants are not necessarily different API versions. They could be iterative changes to
/// the same service, deployed in different environments (prod, staging, dev, etc.). Common
/// scenarios where this occurs include A/B testing, canary rollouts, etc. The choice of a
/// particular version can be decided based on various criterion (headers, url, etc.) and/or by
/// weights assigned to each version. Each service has a default version consisting of all its
/// instances.
///
/// - `Source` - A downstream client calling a service.
/// - `Host` - The address used by a client when attempting to connect to a service.
/// - `Access model` - Applications address only the destination service (Host) without knowledge of
/// individual service versions (subsets). The actual choice of the version is determined by the
/// proxy/sidecar, enabling the application code to decouple itself from the evolution of dependent
/// services.
///
/// A `VirtualService` defines a set of traffic routing rules to apply when a host is addressed.
/// Each routing rule defines matching criteria for traffic of a specific protocol. If the traffic
/// is matched, then it is sent to a named destination service (or subset/version of it) defined in
/// the registry.
///
/// The source of traffic can also be matched in a routing rule. This allows routing to be
/// customized for specific client contexts.
///
/// The following example on Kubernetes, routes all HTTP traffic by default to pods of the reviews
/// service with label “version: v1”. In addition, HTTP requests with path starting with
/// /wpcatalog/ or /consumercatalog/ will be rewritten to /newcatalog and sent to pods with label
/// “version: v2”.
///
/// ```yaml
/// apiVersion: networking.istio.io/v1alpha3
/// kind: VirtualService
/// metadata:
///   name: reviews-route
/// spec:
///   hosts:
///   - reviews.prod.svc.cluster.local
///   http:
///   - name: "reviews-v2-routes"
///     match:
///     - uri:
///         prefix: "/wpcatalog"
///     - uri:
///         prefix: "/consumercatalog"
///     rewrite:
///       uri: "/newcatalog"
///     route:
///     - destination:
///         host: reviews.prod.svc.cluster.local
///         subset: v2
///   - name: "reviews-v1-route"
///     route:
///     - destination:
///         host: reviews.prod.svc.cluster.local
///         subset: v1
/// ```
///
/// A subset/version of a route destination is identified with a reference to a named service subset
/// which must be declared in a corresponding DestinationRule.
///
/// ```yaml
/// apiVersion: networking.istio.io/v1alpha3
/// kind: DestinationRule
/// metadata:
///   name: reviews-destination
/// spec:
///   host: reviews.prod.svc.cluster.local
///   subsets:
///   - name: v1
///     labels:
///       version: v1
///   - name: v2
///     labels:
///       version: v2
/// ```
/// # VirtualService
/// Configuration affecting traffic routing.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceSpec {
    /// The destination hosts to which traffic is being sent. Could be a DNS name with wildcard prefix or an IP address. Depending on the platform, short-names can also be used instead of a FQDN (i.e. has no dots in the name). In such a scenario, the FQDN of the host would be derived based on the underlying platform.
    //
    /// A single VirtualService can be used to describe all the traffic properties of the corresponding hosts, including those for multiple HTTP and TCP ports. Alternatively, the traffic properties of a host can be defined using more than one VirtualService, with certain caveats. Refer to the Operations Guide for details.
    //
    /// Required: Note for Kubernetes users: When short names are used (e.g. “reviews” instead of “reviews.default.svc.cluster.local”), Istio will interpret the short name based on the namespace of the rule, not the service. A rule in the “default” namespace containing a host “reviews” will be interpreted as “reviews.default.svc.cluster.local”, irrespective of the actual namespace associated with the reviews service. To avoid potential misconfigurations, it is recommended to always use fully qualified domain names over short names.
    //
    /// The hosts field applies to both HTTP and TCP services. Service inside the mesh, i.e., those found in the service registry, must always be referred to using their alphanumeric names. IP addresses are allowed only for services defined via the Gateway.
    //
    /// Required: Note: It must be empty for a delegate VirtualService.
    /// Required: No
    pub hosts: Option<Vec<String>>,

    /// The names of gateways and sidecars that should apply these routes. Gateways in other namespaces may be referred to by <gateway namespace>/<gateway name>; specifying a gateway with no namespace qualifier is the same as specifying the VirtualService’s namespace. A single VirtualService is used for sidecars inside the mesh as well as for one or more gateways. The selection condition imposed by this field can be overridden using the source field in the match conditions of protocol-specific routes. The reserved word mesh is used to imply all the sidecars in the mesh. When this field is omitted, the default gateway (mesh) will be used, which would apply the rule to all sidecars in the mesh. If a list of gateway names is provided, the rules will apply only to the gateways. To apply the rules to both gateways and sidecars, specify mesh as one of the gateway names.
    /// Required: No
    pub gateways: Option<Vec<String>>,

    /// An ordered list of route rules for HTTP traffic. HTTP routes will be applied to platform service ports named ‘http-’/‘http2-’/‘grpc-*’, gateway ports with protocol HTTP/HTTP2/GRPC/ TLS-terminated-HTTPS and service entry ports using HTTP/HTTP2/GRPC protocols. The first rule matching an incoming request is used.
    /// Required: No
    pub http: Option<Vec<HttpRoute>>,

    /// An ordered list of route rule for non-terminated TLS & HTTPS traffic. Routing is typically performed using the SNI value presented by the ClientHello message. TLS routes will be applied to platform service ports named ‘https-’, ‘tls-’, unterminated gateway ports using HTTPS/TLS protocols (i.e. with “passthrough” TLS mode) and service entry ports using HTTPS/TLS protocols. The first rule matching an incoming request is used. NOTE: Traffic ‘https-’ or ‘tls-’ ports without associated virtual service will be treated as opaque TCP traffic.
    /// Required: No
    pub tls: Option<Vec<TlsRoute>>,

    /// An ordered list of route rules for opaque TCP traffic. TCP routes will be applied to any port that is not a HTTP or TLS port. The first rule matching an incoming request is used.
    /// Required: No
    pub tcp: Option<Vec<TcpRoute>>,

    /// A list of namespaces to which this virtual service is exported. Exporting a virtual service allows it to be used by sidecars and gateways defined in other namespaces. This feature provides a mechanism for service owners and mesh administrators to control the visibility of virtual services across namespace boundaries.
    //
    /// If no namespaces are specified then the virtual service is exported to all namespaces by default.
    //
    /// The value “.” is reserved and defines an export to the same namespace that the virtual service is declared in. Similarly the value “*” is reserved and defines an export to all namespaces.
    /// Required: No
    pub exportTo: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Destination {
    /// The name of a service from the service registry. Service names are looked up from the platform’s service registry (e.g., Kubernetes services, Consul services, etc.) and from the hosts declared by ServiceEntry. Traffic forwarded to destinations that are not found in either of the two, will be dropped.
    //
    /// Required: Note for Kubernetes users: When short names are used (e.g. “reviews” instead of “reviews.default.svc.cluster.local”), Istio will interpret the short name based on the namespace of the rule, not the service. A rule in the “default” namespace containing a host “reviews will be interpreted as “reviews.default.svc.cluster.local”, irrespective of the actual namespace associated with the reviews service. To avoid potential misconfiguration, it is recommended to always use fully qualified domain names over short names.
    /// Required: Yes
    pub host: String,

    /// The name of a subset within the service. Applicable only to services within the mesh. The subset must be defined in a corresponding DestinationRule.
    /// Required: No
    pub subset: Option<String>,

    /// Specifies the port on the host that is being addressed. If a service exposes only a single port it is not required to explicitly select the port.
    /// Required: No
    pub port: Option<PortSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Delegate {
    /// Name specifies the name of the delegate VirtualService.
    /// Required: No
    pub name: Option<String>,

    /// Namespace specifies the namespace where the delegate VirtualService resides. By default, it is same to the root’s.
    /// Required: No
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Headers {
    /// Header manipulation rules to apply before forwarding a request to the destination service
    /// Required: No
    pub request: Option<HeaderOperations>,
    /// Header manipulation rules to apply before forwarding a request to the destination service
    /// Required: No
    pub response: Option<HeaderOperations>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TlsRoute {
    /// Match conditions to be satisfied for the rule to be activated. All conditions inside a single
    /// match block have AND semantics, while the list of match blocks have OR semantics. The rule is
    /// matched if any one of the match blocks succeed.
    /// Required: Yes
    pub r#match: Vec<TlsMatchAttribures>,
    /// The destination to which the connection should be forwarded to.
    /// Required: No
    pub route: Option<Vec<RouteDestination>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TcpRoute {
    /// Match conditions to be satisfied for the rule to be activated. All conditions inside a single
    /// match block have AND semantics, while the list of match blocks have OR semantics. The rule is
    /// matched if any one of the match blocks succeed.
    /// Required: No
    pub r#match: Option<Vec<L4MatchAttributes>>,
    /// The destination to which the connection should be forwarded to.
    /// Required: No
    pub route: Option<Vec<RouteDestination>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpRoute {
    /// The name assigned to the route for debugging purposes. The route’s name will be concatenated
    /// with the match’s name and will be logged in the access logs for requests matching this route/match.
    /// Required: No
    pub name: Option<String>,

    /// Match conditions to be satisfied for the rule to be activated. All conditions inside a single
    /// match block have AND semantics, while the list of match blocks have OR semantics. The rule is
    /// matched if any one of the match blocks succeed.
    /// Required: No
    pub r#match: Option<Vec<HttpMatchRequest>>,

    /// A HTTP rule can either redirect or forward (default) traffic. The forwarding target can be
    /// one of several versions of a service (see glossary in beginning of document). Weights
    /// associated with the service version determine the proportion of traffic it receives.
    /// Required: No
    pub route: Option<Vec<HttpRouteDestination>>,

    /// A HTTP rule can either redirect or forward (default) traffic. If traffic passthrough option
    /// is specified in the rule, route/redirect will be ignored. The redirect primitive can be used
    /// to send a HTTP 301 redirect to a different URI or Authority.
    /// Required: No
    pub redirect: Option<HttpRedirect>,

    /// Delegate is used to specify the particular VirtualService which can be used to define delegate
    /// HTTPRoute. It can be set only when Route and Redirect are empty, and the route rules of the
    /// delegate VirtualService will be merged with that in the current one. > Note: 1. Only one level
    /// delegation is supported. 2. The delegate’s HTTPMatchRequest must be a strict subset of the
    /// root’s, otherwise there is a conflict and the HTTPRoute will not take effect.
    /// Required: No
    pub delegate: Option<Delegate>,

    /// Rewrite HTTP URIs and Authority headers. Rewrite cannot be used with Redirect primitive.
    /// Rewrite will be performed before forwarding.
    /// Required: No
    pub rewrite: Option<HttpRewrite>,

    /// Timeout for HTTP requests.
    /// Required: No
    pub timeout: Option<Duration>,

    /// Retry policy for HTTP requests.
    /// Required: No
    pub retries: Option<HttpRetry>,

    /// Fault injection policy to apply on HTTP traffic at the client side. Note that timeouts or
    /// retries will not be enabled when faults are enabled on the client side.
    /// Required: No
    pub fault: Option<HttpFaultInjection>,

    /// Mirror HTTP traffic to a another destination in addition to forwarding the requests to the
    /// intended destination. Mirrored traffic is on a best effort basis where the sidecar/gateway
    /// will not wait for the mirrored cluster to respond before returning the response from the
    /// original destination. Statistics will be generated for the mirrored destination.
    /// Required: No
    pub mirror: Option<Destination>,

    /// Percentage of the traffic to be mirrored by the mirror field. If this field is absent, all
    /// the traffic (100%) will be mirrored. Max value is 100.
    /// Required: No
    pub mirrorPercentage: Option<Percent>,

    /// Cross-Origin Resource Sharing policy (CORS). Refer to CORS for further details about cross
    /// origin resource sharing.
    /// Required: No
    pub corsPolicy: Option<CorsPolicy>,

    /// Header manipulation rules
    /// Required: No
    pub headers: Option<Headers>,

    /// Percentage of the traffic to be mirrored by the mirror field. Use of integer mirror_percent
    /// value is deprecated. Use the double mirror_percentage field instead
    /// Required: No
    pub mirrorPercent: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpMatchRequest {
    /// The name assigned to a match. The match’s name will be concatenated with the parent route’s name and will be logged in the access logs for requests matching this route.
    /// Required: No
    pub name: Option<String>,

    /// URI to match values are case-sensitive and formatted as follows:
    //
    /// - exact: "value" for exact string match
    /// - prefix: "value" for prefix-based match
    /// - regex: "value" for RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    //
    /// > Note: Case-insensitive matching could be enabled via the ignore_uri_case flag.
    /// Required: No
    pub uri: Option<StringMatch>,

    /// URI Scheme values are case-sensitive and formatted as follows:
    //
    /// - exact: "value" for exact string match
    /// - prefix: "value" for prefix-based match
    /// - regex: "value" for RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    //
    /// Required: No
    pub scheme: Option<StringMatch>,

    /// HTTP Method values are case-sensitive and formatted as follows:
    //
    /// - exact: "value" for exact string match
    /// - prefix: "value" for prefix-based match
    /// - regex: "value" for RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    //
    /// Required: No
    pub method: Option<StringMatch>,

    /// HTTP Authority values are case-sensitive and formatted as follows:
    //
    /// - exact: "value" for exact string match
    /// - prefix: "value" for prefix-based match
    /// - regex: "value" for RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    //
    /// Required: No
    pub authority: Option<StringMatch>,

    /// The header keys must be lowercase and use hyphen as the separator, e.g. x-request-id.
    //
    /// Header values are case-sensitive and formatted as follows:
    //
    /// - exact: "value" for exact string match
    /// - prefix: "value" for prefix-based match
    /// - regex: "value" for RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    //
    /// If the value is empty and only the name of header is specfied, presence of the header is checked. > Note: The keys uri, scheme, method, and authority will be ignored.
    /// Required: No
    pub headers: Option<HashMap<String, StringMatch>>,

    /// Specifies the ports on the host that is being addressed. Many services only expose a single port or label ports with the protocols they support, in these cases it is not required to explicitly select the port.
    /// Required: No
    pub port: Option<i32>,

    /// One or more labels that constrain the applicability of a rule to source (client) workloads with the given labels. If the VirtualService has a list of gateways specified in the top-level gateways field, it must include the reserved gateway mesh for this field to be applicable.
    /// Required: No
    pub sourceLabels: Option<HashMap<String, String>>,

    /// Names of gateways where the rule should be applied. Gateway names in the top-level gateways field of the VirtualService (if any) are overridden. The gateway match is independent of sourceLabels.
    /// Required: No
    pub gateways: Option<Vec<String>>,

    /// Query parameters for matching.
    //
    /// Ex:
    /// - For a query parameter like “?key=true”, the map key would be “key” and the string match could be defined as exact: "true".
    /// - For a query parameter like “?key”, the map key would be “key” and the string match could be defined as exact: "".
    /// - For a query parameter like “?key=123”, the map key would be “key” and the string match could be defined as regex: "\d+$". Note that this configuration will only match values like “123” but not “a123” or “123a”.
    //
    /// > Note: prefix matching is currently not supported.
    /// Required: No
    pub queryParams: Option<HashMap<String, StringMatch>>,

    /// Flag to specify whether the URI matching should be case-insensitive.
    //
    /// > Note: The case will be ignored only in the case of exact and prefix URI matches.
    /// Required: No
    pub ignoreUriCase: Option<bool>,

    /// withoutHeader has the same syntax with the header, but has opposite meaning. If a header is matched with a matching rule among withoutHeader, the traffic becomes not matched one.
    /// Required: No
    pub withoutHeaders: Option<HashMap<String, StringMatch>>,

    /// Source namespace constraining the applicability of a rule to workloads in that namespace. If the VirtualService has a list of gateways specified in the top-level gateways field, it must include the reserved gateway mesh for this field to be applicable.
    /// Required: No
    pub sourceNamespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RouteDestination {
    /// Destination uniquely identifies the instances of a service to which the request/connection should be forwarded to.
    /// Required: Yes
    pub destination: Destination,

    /// Weight specifies the relative proportion of traffic to be forwarded to the destination. A destination will receive weight/(sum of all weights) requests. If there is only one destination in a rule, it will receive all traffic. Otherwise, if weight is 0, the destination will not receive any traffic.
    /// Required: No
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct L4MatchAttributes {
    /// IPv4 or IPv6 ip addresses of destination with optional subnet. E.g., a.b.c.d/xx form or just a.b.c.d.
    /// Required: No
    pub destinationSubnets: Option<Vec<String>>,

    /// Specifies the port on the host that is being addressed. Many services only expose a single port or label ports with the protocols they support, in these cases it is not required to explicitly select the port.
    /// Required: No
    pub port: Option<i32>,

    /// One or more labels that constrain the applicability of a rule to workloads with the given labels. If the VirtualService has a list of gateways specified in the top-level gateways field, it should include the reserved gateway mesh in order for this field to be applicable.
    /// Required: No
    pub sourceLabels: Option<HashMap<String, String>>,

    /// Names of gateways where the rule should be applied. Gateway names in the top-level gateways field of the VirtualService (if any) are overridden. The gateway match is independent of sourceLabels.
    /// Required: No
    pub gateways: Option<Vec<String>>,

    /// Source namespace constraining the applicability of a rule to workloads in that namespace. If the VirtualService has a list of gateways specified in the top-level gateways field, it must include the reserved gateway mesh for this field to be applicable.
    /// Required: No
    pub sourceNamespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TlsMatchAttribures {
    /// SNI (server name indicator) to match on. Wildcard prefixes can be used in the SNI value, e.g., *.com will match foo.example.com as well as example.com. An SNI value must be a subset (i.e., fall within the domain) of the corresponding virtual serivce’s hosts.
    /// Required: Yes
    pub sniHosts: Vec<String>,

    /// IPv4 or IPv6 ip addresses of destination with optional subnet. E.g., a.b.c.d/xx form or just a.b.c.d.
    /// Required: No
    pub destinationSubnets: Option<Vec<String>>,

    /// Specifies the port on the host that is being addressed. Many services only expose a single port or label ports with the protocols they support, in these cases it is not required to explicitly select the port.
    /// Required: No
    pub port: Option<u32>,

    /// One or more labels that constrain the applicability of a rule to workloads with the given labels. If the VirtualService has a list of gateways specified in the top-level gateways field, it should include the reserved gateway mesh in order for this field to be applicable.
    /// Required: No
    pub sourceLabels: Option<HashMap<String, String>>,

    /// Names of gateways where the rule should be applied. Gateway names in the top-level gateways field of the VirtualService (if any) are overridden. The gateway match is independent of sourceLabels.
    /// Required: No
    pub gateways: Option<String>,

    /// Source namespace constraining the applicability of a rule to workloads in that namespace. If the VirtualService has a list of gateways specified in the top-level gateways field, it must include the reserved gateway mesh for this field to be applicable.
    /// Required: No
    pub sourceNamespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpRedirect {
    /// On a redirect, overwrite the Path portion of the URL with this value. Note that the entire path will be replaced, irrespective of the request URI being matched as an exact path or prefix.
    /// Required: No
    pub uri: Option<String>,

    /// On a redirect, overwrite the Authority/Host portion of the URL with this value.
    /// Required: No
    pub authority: Option<String>,

    /// On a redirect, overwrite the port portion of the URL with this value.
    /// Required: No
    pub port: Option<u32>,

    /// On a redirect, dynamically set the port: * FROMPROTOCOLDEFAULT: automatically set to 80 for HTTP and 443 for HTTPS. * FROMREQUESTPORT: automatically use the port of the request.
    /// Required: No,
    pub derivePort: Option<RedirectPortSelection>,

    /// On a redirect, overwrite the scheme portion of the URL with this value. For example, http or https. If unset, the original scheme will be used. If derivePort is set to FROM_PROTOCOL_DEFAULT, this will impact the port used as well
    /// Required: No
    pub scheme: Option<String>,

    /// On a redirect, Specifies the HTTP status code to use in the redirect response. The default response code is MOVED_PERMANENTLY (301).
    /// Required: No
    pub redirectCode: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpRouteDestination {
    /// Destination uniquely identifies the instances of a service to which the request/connection should be forwarded to.
    /// Required: Yes
    pub destination: Destination,

    /// Weight specifies the relative proportion of traffic to be forwarded to the destination. A destination will receive weight/(sum of all weights) requests. If there is only one destination in a rule, it will receive all traffic. Otherwise, if weight is 0, the destination will not receive any traffic.
    /// Required: No
    pub weight: Option<i32>,

    /// Header manipulation rules
    /// Required: No
    pub headers: Option<Headers>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpRewrite {
    /// rewrite the path (or the prefix) portion of the URI with this value. If the original URI was matched based on prefix, the value provided in this field will replace the corresponding matched prefix.
    /// Required: No
    pub uri: Option<String>,

    /// rewrite the Authority/Host header with this value.
    /// Required: No
    pub authority: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StringMatch {
    /// exact string match
    exact(String),

    /// prefix-based match
    prefix(String),

    /// RE2 style regex-based match (https://github.com/google/re2/wiki/Syntax).
    regex(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpRetry {
    /// Number of retries to be allowed for a given request. The interval between retries will be determined automatically (25ms+). When request timeout of the HTTP route or per_try_timeout is configured, the actual number of retries attempted also depends on the specified request timeout and per_try_timeout values.
    /// Required: Yes
    pub attempts: i32,

    /// Timeout per attempt for a given request, including the initial call and any retries. Format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is same value as request timeout of the HTTP route, which means no timeout.
    /// Required: No
    pub perTryTimeout: Option<Duration>,

    /// Specifies the conditions under which retry takes place. One or more policies can be specified using a ‘,’ delimited list. If retry_on specifies a valid HTTP status, it will be added to retriablestatuscodes retry policy. See the retry policies and gRPC retry policies for more details.
    /// Required: No
    pub retryOn: Option<String>,

    /// Flag to specify whether the retries should retry to other localities. See the retry plugin configuration for more details.
    /// Required: No
    pub retryRemoteLocalities: Option<bool>,
}

/// Cross-Origin Resource Sharing policy (CORS).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CorsPolicy {
    /// String patterns that match allowed origins. An origin is allowed if any of the string matchers match. If a match is found, then the outgoing Access-Control-Allow-Origin would be set to the origin as provided by the client.
    /// Required: No
    pub allowOrigins: Option<Vec<StringMatch>>,

    /// List of HTTP methods allowed to access the resource. The content will be serialized into the Access-Control-Allow-Methods header.
    /// Required: No
    pub allowMethods: Option<Vec<String>>,

    /// List of HTTP headers that can be used when requesting the resource. Serialized to Access-Control-Allow-Headers header.
    /// Required: No
    pub allowHeaders: Option<Vec<String>>,

    /// A list of HTTP headers that the browsers are allowed to access. Serialized into Access-Control-Expose-Headers header.
    /// Required: No
    pub exposeHeaders: Option<Vec<String>>,

    /// Specifies how long the results of a preflight request can be cached. Translates to the Access-Control-Max-Age header.
    /// Required: No
    pub maxAge: Option<Duration>,

    /// Indicates whether the caller is allowed to send the actual request (not the preflight) using credentials. Translates to Access-Control-Allow-Credentials header.
    /// Required: No
    pub allowCredentials: Option<bool>,
}

//// # HTTPFaultInjection
//// HTTPFaultInjection can be used to specify one or more faults to inject while forwarding HTTP requests to the destination specified in a route. Fault specification is part of a VirtualService rule. Faults include aborting the Http request from downstream service, and/or delaying proxying of requests. A fault rule MUST HAVE delay or abort or both.
///
//// > Note: Delay and abort faults are independent of one another, even if both are specified simultaneously.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HttpFaultInjection {
    /// Delay requests before forwarding, emulating various failures such as network issues, overloaded upstream service, etc.
    /// Required: No
    pub delay: Option<FaultInjectionDelay>,

    /// Abort Http request attempts and return error codes back to downstream service, giving the impression that the upstream service is faulty.
    /// Required: No
    pub abort: Option<FaultInjectionAbort>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PortSelector {
    /// Valid port number
    /// Required: No
    pub number: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Percent(f32);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderOperations {
    /// Overwrite the headers specified by key with the given values
    /// Required: No
    pub set: Option<HashMap<String, String>>,

    /// Append the given values to the headers specified by keys (will create a comma-separated list of values)
    /// Required: No
    pub add: Option<HashMap<String, String>>,

    /// Remove the specified headers
    /// Required: No
    pub remove: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FaultInjectionDelay {
    /// Add a fixed delay before forwarding the request. Format: 1h/1m/1s/1ms. MUST be >=1ms.
    /// Required: Yes
    #[serde(rename = "fixedDelay")]
    pub fixed_delay: Duration,

    /// Percentage of requests on which the delay will be injected.
    /// Required: No
    pub percentage: Option<Percent>,

    /// Percentage of requests on which the delay will be injected (0-100). Use of integer percent value is deprecated. Use the double percentage field instead.
    /// Required: No
    pub percent: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FaultInjectionAbort {
    /// HTTP status code to use to abort the Http request.
    /// Required: Yes
    #[serde(rename = "httpStatus")]
    pub http_status: i32,

    /// Percentage of requests to be aborted with the error code provided.
    /// Required: No
    pub percentage: Option<Percent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RedirectPortSelection {
    #[serde(rename = "FROM_PROTOCOL_DEFAULT")]
    FromProtocolDefault,
    #[serde(rename = "FROM_REQUEST_PORT")]
    FromRequestPort,
}
