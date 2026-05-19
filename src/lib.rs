//! OwnerSignal contract for privileged `persona-spirit` lifecycle.
//!
//! Ordinary psyche and intent vocabulary lives in `signal-persona-spirit`.
//! This crate carries supervisor-issued lifecycle and policy orders only.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub struct SpiritGeneration {
    pub value: u64,
}

impl SpiritGeneration {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct PsycheIdentityName(String);

impl PsycheIdentityName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StartSpiritOrder {
    pub generation: SpiritGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DrainAndStopOrder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ReloadBootstrapPolicyOrder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RegisterPsycheIdentity {
    pub name: PsycheIdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetirePsycheIdentity {
    pub name: PsycheIdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpiritStarted {
    pub generation: SpiritGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpiritDrainedAndStopped {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPolicyReloaded {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PsycheIdentityRegistered {
    pub name: PsycheIdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PsycheIdentityRetired {
    pub name: PsycheIdentityName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerSpiritOperationKind {
    StartSpiritOrder,
    DrainAndStopOrder,
    ReloadBootstrapPolicyOrder,
    RegisterPsycheIdentity,
    RetirePsycheIdentity,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerSpiritUnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerSpiritRequestUnimplemented {
    pub operation: OwnerSpiritOperationKind,
    pub reason: OwnerSpiritUnimplementedReason,
}

signal_channel! {
    channel OwnerSpirit {
        request OwnerSpiritRequest {
            Mutate StartSpiritOrder(StartSpiritOrder),
            Mutate DrainAndStopOrder(DrainAndStopOrder),
            Mutate ReloadBootstrapPolicyOrder(ReloadBootstrapPolicyOrder),
            Mutate RegisterPsycheIdentity(RegisterPsycheIdentity),
            Retract RetirePsycheIdentity(RetirePsycheIdentity),
        }
        reply OwnerSpiritReply {
            SpiritStarted(SpiritStarted),
            SpiritDrainedAndStopped(SpiritDrainedAndStopped),
            BootstrapPolicyReloaded(BootstrapPolicyReloaded),
            PsycheIdentityRegistered(PsycheIdentityRegistered),
            PsycheIdentityRetired(PsycheIdentityRetired),
            OwnerSpiritRequestUnimplemented(OwnerSpiritRequestUnimplemented),
        }
    }
}

pub type Frame = OwnerSpiritFrame;
pub type FrameBody = OwnerSpiritFrameBody;
pub type ChannelRequest = OwnerSpiritChannelRequest;
pub type ChannelReply = OwnerSpiritChannelReply;
pub type RequestBuilder = OwnerSpiritRequestBuilder;

impl OwnerSpiritRequest {
    pub fn operation_kind(&self) -> OwnerSpiritOperationKind {
        match self {
            Self::StartSpiritOrder(_) => OwnerSpiritOperationKind::StartSpiritOrder,
            Self::DrainAndStopOrder(_) => OwnerSpiritOperationKind::DrainAndStopOrder,
            Self::ReloadBootstrapPolicyOrder(_) => {
                OwnerSpiritOperationKind::ReloadBootstrapPolicyOrder
            }
            Self::RegisterPsycheIdentity(_) => OwnerSpiritOperationKind::RegisterPsycheIdentity,
            Self::RetirePsycheIdentity(_) => OwnerSpiritOperationKind::RetirePsycheIdentity,
        }
    }
}

impl From<SpiritStarted> for OwnerSpiritReply {
    fn from(payload: SpiritStarted) -> Self {
        Self::SpiritStarted(payload)
    }
}

impl From<SpiritDrainedAndStopped> for OwnerSpiritReply {
    fn from(payload: SpiritDrainedAndStopped) -> Self {
        Self::SpiritDrainedAndStopped(payload)
    }
}

impl From<BootstrapPolicyReloaded> for OwnerSpiritReply {
    fn from(payload: BootstrapPolicyReloaded) -> Self {
        Self::BootstrapPolicyReloaded(payload)
    }
}

impl From<PsycheIdentityRegistered> for OwnerSpiritReply {
    fn from(payload: PsycheIdentityRegistered) -> Self {
        Self::PsycheIdentityRegistered(payload)
    }
}

impl From<PsycheIdentityRetired> for OwnerSpiritReply {
    fn from(payload: PsycheIdentityRetired) -> Self {
        Self::PsycheIdentityRetired(payload)
    }
}

impl From<OwnerSpiritRequestUnimplemented> for OwnerSpiritReply {
    fn from(payload: OwnerSpiritRequestUnimplemented) -> Self {
        Self::OwnerSpiritRequestUnimplemented(payload)
    }
}
