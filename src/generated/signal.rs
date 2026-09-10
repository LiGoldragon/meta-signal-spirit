#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CriomeSocketPath {
    pub criome_socket_path_text: CriomeSocketPathText,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionedLogHeadObject {
    pub database_marker: signal_spirit::DatabaseMarker,
    pub selected_head_object: SelectedHeadObject,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ArchiveDatabaseTarget {
    Path(ArchivePath),
    Default,
}
pub type HeadDigestHex = String;
pub type MirrorAddressText = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureRequest {
    pub archive_database_target: ArchiveDatabaseTarget,
    pub selected_mirror_target: SelectedMirrorTarget,
    pub selected_criome_gate_target: SelectedCriomeGateTarget,
    pub selected_guardian_prompt_target: SelectedGuardianPromptTarget,
}
pub type SelectedMirrorTarget = std::option::Option<MirrorTarget>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum CriomeGateTarget {
    Default,
    Socket(CriomeSocketPath),
}
pub type SelectedHeadObject = std::option::Option<HeadObjectHex>;
pub type SelectedHeadDigest = std::option::Option<HeadDigestHex>;
pub type ArchivePathText = String;
pub type HeadObjectHex = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum GuardianPromptTarget {
    Prompt(GuardianPrompt),
    Default,
}
pub type ImportedRecords = std::vec::Vec<ImportedRecord>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct GuardianPrompt {
    pub guardian_prompt_text: GuardianPromptText,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportedRecord {
    pub record_identifier: signal_spirit::RecordIdentifier,
    pub entry: signal_spirit::Entry,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureReceipt {
    pub archive_database_target: ArchiveDatabaseTarget,
    pub selected_mirror_target: SelectedMirrorTarget,
    pub selected_criome_gate_target: SelectedCriomeGateTarget,
    pub selected_guardian_prompt_target: SelectedGuardianPromptTarget,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConfigureRejectionReason {
    ArchiveTargetUnwritable,
    InternalError,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MirrorTarget {
    Address(MirrorAddress),
    Default,
}
pub type SelectedCriomeGateTarget = std::option::Option<CriomeGateTarget>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct VersionedLogHead {
    pub database_marker: signal_spirit::DatabaseMarker,
    pub selected_head_digest: SelectedHeadDigest,
}
pub type GuardianPromptText = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportRequest {
    pub imported_records: ImportedRecords,
}
pub type SelectedGuardianPromptTarget = std::option::Option<GuardianPromptTarget>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ConfigureRejection {
    pub configure_rejection_reason: ConfigureRejectionReason,
    pub database_marker: signal_spirit::DatabaseMarker,
}
pub type CriomeSocketPathText = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ImportReceipt {
    pub record_count: signal_spirit::RecordCount,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ArchivePath {
    pub archive_path_text: ArchivePathText,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MirrorAddress {
    pub mirror_address_text: MirrorAddressText,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    ObserveHead,
    Configure(ConfigureRequest),
    ObserveHeadObject,
    Import(ImportRequest),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Configured(ConfigureReceipt),
    HeadObserved(VersionedLogHead),
    HeadObjectObserved(VersionedLogHeadObject),
    Rejected(ConfigureRejection),
    Imported(ImportReceipt),
}
