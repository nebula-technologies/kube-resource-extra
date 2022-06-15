//! # Kube Extra
//! kube-extra is a small package to handle other resources that are normally not part of kubernetes.
//! In this case kube-extra is adding a well-known istio resource - `VirtualService` to allow to
//! better manipulation of virtual services.

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate k8s_openapi;

#[cfg(feature = "istio")]
pub mod istio;
