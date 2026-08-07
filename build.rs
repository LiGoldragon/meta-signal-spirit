//! Bootstrap generation for the meta Interface through the unified
//! authority pipeline.
//!
//! This build script authorizes `schema/meta.schema` through
//! `SemaBootstrapAuthority`, generates canonical Ethos source and Rust
//! projections through `BootstrapGeneration`, and installs them atomically
//! through `CommitBootstrap`.
//!
//! The dependency chain is signal-domain -> signal-spirit -> meta, so both
//! dependency sources are authorized first so the meta source can resolve
//! its `signal/spirit` imports (which in turn resolved `signal/domain`).
//!
// psyche-grasp: unseen

use std::{collections::BTreeMap, env, fs, path::PathBuf};

use core_nomos::InterfaceRoleTraitIdentities;
use name_table::{EncodedName, NameView, TextualMetadata};
use rust_logos::{RustLogos, RustTypePath, RustTypePathResolver};
use schema_rust::{
    bootstrap::{BootstrapGeneration, CommitBootstrap},
    build::CargoEthosSourceMetadata,
};
use sema_translator::bootstrap::{AuthorityNameView, SemaBootstrapAuthority, SourcePlacement};

fn main() {
    MetaBuild::from_environment().run();
}

struct MetaBuild {
    crate_root: PathBuf,
}

impl MetaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/meta.schema");
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=src/schema/meta/generated.rs");

        let domain_metadata = CargoEthosSourceMetadata::new("signal-domain");
        domain_metadata.emit_dependency_rerun_instruction();

        let spirit_metadata = CargoEthosSourceMetadata::new("signal-spirit");
        spirit_metadata.emit_dependency_rerun_instruction();

        let source_path = self.crate_root.join("schema/meta.schema");
        let rust_path = self.crate_root.join("src/schema/meta/generated.rs");
        let source = fs::read_to_string(&source_path).expect("read meta Interface source");

        let mut authority =
            SemaBootstrapAuthority::new().expect("empty authority owns its seed");

        // Authorize the domain dependency first so signal-spirit can resolve
        // its signal/domain imports during planning.
        self.authorize_domain_dependency(&mut authority, &domain_metadata);

        // Authorize the spirit dependency so the meta source can resolve its
        // signal/spirit imports during planning.
        self.authorize_spirit_dependency(&mut authority, &spirit_metadata);

        let priors = authority.prior_identities();
        let role_traits = InterfaceRoleTraitIdentities::new(
            priors.input_role.clone(),
            priors.output_role.clone(),
            priors.refusal_role.clone(),
            priors.stream_role.clone(),
        );

        let placement = SourcePlacement::new(
            vec!["meta_signal_spirit".to_owned(), "meta".to_owned()],
            vec![
                "meta_signal_spirit".to_owned(),
                "meta".to_owned(),
                "meta.schema".to_owned(),
            ],
        );

        let assembly = authority
            .authorize(&source, placement)
            .expect("assemble authority-approved meta Interface transaction");
        let rust = RustLogos::new();
        let type_paths = MetaRustTypePaths::from_name_view(assembly.name_view());

        let generated = BootstrapGeneration::new(
            &assembly,
            &rust,
            &type_paths,
            &[],
            &source_path,
            &rust_path,
        )
        .with_role_traits(&role_traits)
        .generate()
        .expect("project meta Interface from the verified transaction");

        CommitBootstrap::single(generated)
            .write_or_check("META_SIGNAL_SPIRIT_UPDATE_INTERFACE_ARTIFACTS")
            .expect("checked-in meta Interface source and Rust projection are fresh");

        CargoEthosSourceMetadata::new("meta-signal-spirit")
            .publish_owned_source_directory(self.crate_root.join("schema"));
    }

    fn authorize_domain_dependency(
        &self,
        authority: &mut SemaBootstrapAuthority,
        metadata: &CargoEthosSourceMetadata,
    ) {
        let source_dir = metadata
            .dependency_source_directory()
            .expect("signal-domain must publish its Ethos source directory via `links`");
        let domain_source_path = source_dir.join("domain.schema");
        let domain_source =
            fs::read_to_string(&domain_source_path).expect("read domain Interface source");

        let domain_placement = SourcePlacement::new(
            vec!["signal".to_owned(), "domain".to_owned()],
            vec![
                "signal".to_owned(),
                "domain".to_owned(),
                "domain.schema".to_owned(),
            ],
        );

        authority
            .admit_domain_shape("ScopeOf", 1)
            .expect("authority admits the domain ScopeOf shape constructor");
        authority
            .authorize(&domain_source, domain_placement)
            .expect("authorize the domain dependency Interface for import resolution");
    }

    fn authorize_spirit_dependency(
        &self,
        authority: &mut SemaBootstrapAuthority,
        metadata: &CargoEthosSourceMetadata,
    ) {
        let source_dir = metadata
            .dependency_source_directory()
            .expect("signal-spirit must publish its Ethos source directory via `links`");
        let spirit_source_path = source_dir.join("spirit.schema");
        let spirit_source =
            fs::read_to_string(&spirit_source_path).expect("read spirit Interface source");

        let spirit_placement = SourcePlacement::new(
            vec!["signal".to_owned(), "spirit".to_owned()],
            vec![
                "signal".to_owned(),
                "spirit".to_owned(),
                "spirit.schema".to_owned(),
            ],
        );

        authority
            .authorize(&spirit_source, spirit_placement)
            .expect("authorize the spirit dependency Interface for import resolution");
    }
}

/// Resolves external Rust type paths by looking up textual names through the
/// sealed authority name view.
struct MetaRustTypePaths<'a> {
    name_view: &'a AuthorityNameView,
    overrides: BTreeMap<&'static str, RustTypePath>,
}

impl<'a> MetaRustTypePaths<'a> {
    fn from_name_view(name_view: &'a AuthorityNameView) -> Self {
        let path = |segments: &[&str]| -> RustTypePath {
            RustTypePath::try_new(segments.iter().map(|s| (*s).to_owned()).collect())
                .expect("static Rust type path segments are valid")
        };
        let overrides = BTreeMap::from([
            // Role traits from protos.
            ("Input", path(&["protos", "Input"])),
            ("Output", path(&["protos", "Output"])),
            ("Refusal", path(&["protos", "Refusal"])),
            ("Stream", path(&["protos", "Stream"])),
            // Imported spirit types.
            ("DatabaseMarker", path(&["signal_spirit", "DatabaseMarker"])),
            ("Entry", path(&["signal_spirit", "Entry"])),
            ("RecordIdentifier", path(&["signal_spirit", "RecordIdentifier"])),
            ("RecordCount", path(&["signal_spirit", "RecordCount"])),
        ]);
        Self {
            name_view,
            overrides,
        }
    }
}

impl RustTypePathResolver for MetaRustTypePaths<'_> {
    fn resolve_type_path(&self, encoded_name: &EncodedName) -> Option<&RustTypePath> {
        let metadata: &TextualMetadata = self.name_view.textual_metadata(encoded_name)?;
        self.overrides.get(metadata.textual_name().as_str())
    }
}
