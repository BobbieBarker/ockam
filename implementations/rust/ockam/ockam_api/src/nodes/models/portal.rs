//! Inlets and outlet request/response types

use std::time::Duration;

use minicbor::{Decode, Encode};
use ockam::route;
use ockam_core::compat::borrow::Cow;
#[cfg(feature = "tag")]
use ockam_core::TypeTag;
use ockam_core::{CowStr, Route};
use ockam_identity::IdentityIdentifier;
use ockam_multiaddr::MultiAddr;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::route_to_multiaddr;

/// Request body to create an inlet
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateInlet<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<1407961>,
    /// The address the portal should listen at.
    #[n(1)] listen_addr: String,
    /// The peer address.
    /// This can either be the address of an already
    /// created outlet, or a forwarding mechanism via ockam cloud.
    #[n(2)] outlet_addr: MultiAddr,
    /// A human-friendly alias for this portal endpoint
    #[b(3)] alias: Option<CowStr<'a>>,
    /// An authorised identity for secure channels.
    /// Only set for non-project addresses as for projects the project's
    /// authorised identity will be used.
    #[n(4)] authorized: Option<IdentityIdentifier>,
    /// A prefix route that will be applied before outlet_addr, and won't be used
    /// to monitor the state of the connection
    #[n(5)] prefix_route: Route,
    /// A suffix route that will be applied after outlet_addr, and won't be used
    /// to monitor the state of the connection
    #[n(6)] suffix_route: Route,
    /// The maximum duration to wait for an outlet to be available
    #[n(7)] wait_for_outlet_duration: Option<Duration>,
}

impl<'a> CreateInlet<'a> {
    pub fn via_project(
        listen: String,
        to: MultiAddr,
        prefix_route: Route,
        suffix_route: Route,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            listen_addr: listen,
            outlet_addr: to,
            alias: None,
            authorized: None,
            prefix_route,
            suffix_route,
            wait_for_outlet_duration: None,
        }
    }

    pub fn to_node(
        listen: String,
        to: MultiAddr,
        prefix_route: Route,
        suffix_route: Route,
        auth: Option<IdentityIdentifier>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            listen_addr: listen,
            outlet_addr: to,
            alias: None,
            authorized: auth,
            prefix_route,
            suffix_route,
            wait_for_outlet_duration: None,
        }
    }

    pub fn set_alias(&mut self, a: impl Into<Cow<'a, str>>) {
        self.alias = Some(CowStr(a.into()))
    }

    pub fn set_wait_ms(&mut self, ms: u64) {
        self.wait_for_outlet_duration = Some(Duration::from_millis(ms))
    }

    pub fn listen_addr(&self) -> String {
        self.listen_addr.clone()
    }

    pub fn outlet_addr(&self) -> &MultiAddr {
        &self.outlet_addr
    }

    pub fn authorized(&self) -> Option<IdentityIdentifier> {
        self.authorized.clone()
    }

    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    pub fn prefix_route(&self) -> &Route {
        &self.prefix_route
    }

    pub fn suffix_route(&self) -> &Route {
        &self.suffix_route
    }

    pub fn wait_for_outlet_duration(&self) -> Option<Duration> {
        self.wait_for_outlet_duration
    }
}

/// Request body to create an outlet
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateOutlet {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<5351558>,
    /// The address the portal should connect or bind to
    #[n(1)] pub tcp_addr: String,
    /// The address the portal should connect or bind to
    #[n(2)] pub worker_addr: String,
    /// A human-friendly alias for this portal endpoint
    #[n(3)] pub alias: Option<String>,
    /// Allow the outlet to be reachable from the default secure channel, useful when we want to
    /// tighten the flow control
    #[n(4)] pub reachable_from_default_secure_channel: bool,
}

impl CreateOutlet {
    pub fn new(
        tcp_addr: impl Into<String>,
        worker_addr: impl Into<String>,
        alias: impl Into<Option<String>>,
        reachable_from_default_secure_channel: bool,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: tcp_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
            reachable_from_default_secure_channel,
        }
    }
}

/// Response body when interacting with a portal endpoint
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct InletStatus {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<9302588>,
    #[n(1)] pub bind_addr: String,
    #[n(2)] pub worker_addr: String,
    #[n(3)] pub alias: String,
    /// An optional status payload
    #[n(4)] pub payload: Option<String>,
    #[n(5)] pub outlet_route: String,
}

impl Serialize for InletStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("InletStatus", 5)?;
        state.serialize_field("bind_addr", &self.bind_addr)?;
        state.serialize_field("worker_addr", &self.worker_addr)?;
        state.serialize_field("alias", &self.alias)?;
        state.serialize_field("payload", &self.payload)?;
        state.serialize_field("outlet_route", &self.outlet_route)?;
        state.end()
    }
}

impl InletStatus {
    pub fn bad_request(reason: &'static str) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            bind_addr: "".into(),
            worker_addr: "".into(),
            alias: "".into(),
            payload: Some(reason.into()),
            outlet_route: "".into(),
        }
    }

    pub fn new(
        bind_addr: impl Into<String>,
        worker_addr: impl Into<String>,
        alias: impl Into<String>,
        payload: impl Into<Option<String>>,
        outlet_route: impl Into<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            bind_addr: bind_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
            payload: payload.into(),
            outlet_route: outlet_route.into(),
        }
    }
}

/// Response body when interacting with a portal endpoint
#[derive(Clone, Debug, Decode, Encode, Serialize, Deserialize)]
#[rustfmt::skip]
#[cbor(map)]
pub struct OutletStatus {
    #[cfg(feature = "tag")]
    #[serde(skip)]
    #[n(0)] tag: TypeTag<4012569>,
    #[n(1)] pub tcp_addr: String,
    #[n(2)] pub worker_addr: String,
    #[n(3)] pub alias:String,
    /// An optional status payload
    #[n(4)] pub payload: Option<String>,
}

impl OutletStatus {
    pub fn bad_request(reason: &'static str) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: "".into(),
            worker_addr: "".into(),
            alias: "".into(),
            payload: Some(reason.into()),
        }
    }

    pub fn new(
        tcp_addr: impl Into<String>,
        worker_addr: impl Into<String>,
        alias: impl Into<String>,
        payload: impl Into<Option<String>>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: tcp_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
            payload: payload.into(),
        }
    }

    pub fn worker_address(&self) -> Result<MultiAddr, ockam_core::Error> {
        route_to_multiaddr(&route![self.worker_addr.to_string()])
            .ok_or_else(|| ApiError::generic("Invalid Worker Address"))
    }
}

/// Response body when returning a list of Inlets
#[derive(Debug, Clone, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct InletList {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<8401504>,
    #[n(1)] pub list: Vec<InletStatus>
}

impl InletList {
    pub fn new(list: Vec<InletStatus>) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            list,
        }
    }
}

/// Response body when returning a list of Outlets
#[derive(Debug, Clone, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct OutletList {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<8708916>,
    #[n(1)] pub list: Vec<OutletStatus>
}

impl OutletList {
    pub fn new(list: Vec<OutletStatus>) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            list,
        }
    }
}
