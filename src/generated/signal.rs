#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CriomeSocketPath {
    pub criome_socket_path_text: CriomeSocketPathText,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionedLogHeadObject {
    pub database_marker: signal_spirit::DatabaseMarker,
    pub selected_head_object: SelectedHeadObject,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ArchiveDatabaseTarget {
    Path(ArchivePath),
    Default,
}
#[rustfmt::skip]
pub type HeadDigestHex = String;
#[rustfmt::skip]
pub type MirrorAddressText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureRequest {
    pub spirit_nexus_configuration: signal_spirit::SpiritNexusConfiguration,
    pub archive_database_target: ArchiveDatabaseTarget,
    pub selected_mirror_target: SelectedMirrorTarget,
    pub selected_criome_gate_target: SelectedCriomeGateTarget,
    pub selected_guardian_prompt_target: SelectedGuardianPromptTarget,
}
#[rustfmt::skip]
pub type SelectedMirrorTarget = std::option::Option<MirrorTarget>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CriomeGateTarget {
    Default,
    Socket(CriomeSocketPath),
}
#[rustfmt::skip]
pub type SelectedHeadObject = std::option::Option<HeadObjectHex>;
#[rustfmt::skip]
pub type SelectedHeadDigest = std::option::Option<HeadDigestHex>;
#[rustfmt::skip]
pub type ArchivePathText = String;
#[rustfmt::skip]
pub type HeadObjectHex = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum GuardianPromptTarget {
    Prompt(GuardianPrompt),
    Default,
}
#[rustfmt::skip]
pub type ImportedRecords = std::vec::Vec<ImportedRecord>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GuardianPrompt {
    pub guardian_prompt_text: GuardianPromptText,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportedRecord {
    pub record_identifier: signal_spirit::RecordIdentifier,
    pub entry: signal_spirit::Entry,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureReceipt {
    pub spirit_nexus_configuration: signal_spirit::SpiritNexusConfiguration,
    pub archive_database_target: ArchiveDatabaseTarget,
    pub selected_mirror_target: SelectedMirrorTarget,
    pub selected_criome_gate_target: SelectedCriomeGateTarget,
    pub selected_guardian_prompt_target: SelectedGuardianPromptTarget,
    pub database_marker: signal_spirit::DatabaseMarker,
    pub meta_configure_done: MetaConfigureDone,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigurationReceipt {
    pub spirit_nexus_configuration: signal_spirit::SpiritNexusConfiguration,
    pub meta_configure_done: MetaConfigureDone,
}
#[rustfmt::skip]
pub type MetaConfigureDone = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConfigureRejectionReason {
    ArchiveTargetUnwritable,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MirrorTarget {
    Address(MirrorAddress),
    Default,
}
#[rustfmt::skip]
pub type SelectedCriomeGateTarget = std::option::Option<CriomeGateTarget>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionedLogHead {
    pub database_marker: signal_spirit::DatabaseMarker,
    pub selected_head_digest: SelectedHeadDigest,
}
#[rustfmt::skip]
pub type GuardianPromptText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportRequest {
    pub imported_records: ImportedRecords,
}
#[rustfmt::skip]
pub type SelectedGuardianPromptTarget = std::option::Option<GuardianPromptTarget>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureRejection {
    pub configure_rejection_reason: ConfigureRejectionReason,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[rustfmt::skip]
pub type CriomeSocketPathText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportReceipt {
    pub record_count: signal_spirit::RecordCount,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ArchivePath {
    pub archive_path_text: ArchivePathText,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MirrorAddress {
    pub mirror_address_text: MirrorAddressText,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    ObserveHead,
    Configure(ConfigureRequest),
    ReverseMetaConfiguration,
    ObserveHeadObject,
    Import(ImportRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Configured(ConfigureReceipt),
    OrdinaryConfigurationReopened(ConfigurationReceipt),
    HeadObserved(VersionedLogHead),
    HeadObjectObserved(VersionedLogHeadObject),
    Rejected(ConfigureRejection),
    Imported(ImportReceipt),
}
