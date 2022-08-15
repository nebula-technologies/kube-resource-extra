pub mod client_tls_settings;
pub mod connection_pool_settings;
pub mod destination_rule;
pub mod envoy_filter;
pub mod gateway;
pub mod load_balancer_settings;
pub mod locality_load_balancer_settings;
pub mod traffic_policy;
pub mod virtual_service;

pub use destination_rule::DestinationRule;
pub use envoy_filter::EnvoyFilter;
pub use gateway::Gateway;
use std::collections::HashMap;
pub use virtual_service::VirtualService;

pub mod google {
    use k8s_openapi::serde_json::Value;
    use std::collections::HashMap;

    pub mod protobuf {
        /// # UInt32Value
        /// Wrapper message for uint32.
        /// The JSON representation for UInt32Value is JSON number.
        #[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct UInt32Value {
            // The uint32 value.
            // No
            pub value: Option<u32>,
        }
    }
    /// # Struct
    /// Struct represents a structured data value, consisting of fields which map to dynamically typed values. In some languages, Struct might be supported by a native representation. For example, in scripting languages like JS a struct is represented as an object. The details of that representation are described together with the proto support for the language.
    /// The JSON representation for Struct is JSON object.
    #[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Struct {
        pub fields: HashMap<String, Value>,
    }
}

/// # WorkloadSelector
/// `WorkloadSelector` specifies the criteria used to determine if the `Gateway`, `Sidecar`, `EnvoyFilter`, or `ServiceEntry` configuration can be applied to a proxy. The matching criteria includes the metadata associated with a proxy, workload instance info such as labels attached to the pod/VM, or any other info that the proxy provides to Istio during the initial handshake. If multiple conditions are specified, all conditions need to match in order for the workload instance to be selected. Currently, only label based selection mechanism is supported.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadSelector {
    // One or more labels that indicate a specific set of pods/VMs on which the configuration should be applied. The scope of label search is restricted to the configuration namespace in which the the resource is present.
    // Yes
    pub labels: HashMap<String, String>,
}
