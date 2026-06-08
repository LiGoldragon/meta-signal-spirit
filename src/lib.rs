//! MetaSignal contract for privileged `spirit` lifecycle.
//!
//! Ordinary psyche and intent vocabulary lives in `signal-spirit`.
//! This crate carries supervisor-issued lifecycle and policy orders only.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Generation(u64);

impl Generation {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentityName(String);

impl IdentityName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Start {
    pub generation: Generation,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Drain {}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPolicy {}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub name: IdentityName,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Retirement {
    pub name: IdentityName,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Started {
    pub generation: Generation,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DrainedAndStopped {}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPolicyReloaded {}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct IdentityRegistered {
    pub name: IdentityName,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct IdentityRetired {
    pub name: IdentityName,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[cfg_attr(
    feature = "nota-text",
    derive(::nota_next::NotaEncode, ::nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Meta {
        operation Start(Start),
        operation Drain(Drain),
        operation Reload(BootstrapPolicy),
        operation Register(Registration),
        operation Retire(Retirement),
    }
    reply Reply {
        Started(Started),
        DrainedAndStopped(DrainedAndStopped),
        BootstrapPolicyReloaded(BootstrapPolicyReloaded),
        IdentityRegistered(IdentityRegistered),
        IdentityRetired(IdentityRetired),
        RequestUnimplemented(RequestUnimplemented),
    }
}
