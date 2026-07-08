use std::{env, path::PathBuf};

use schema_rust::build::{
    CargoSchemaMetadata, DependencySchema, GenerationDriver, GenerationPlan, ModuleEmission,
};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/meta-signal.schema");
        println!("cargo:rerun-if-changed=src/schema/meta_signal.rs");

        let signal_domain =
            DependencySchema::from_cargo_metadata("signal-domain", "signal-domain", "0.1.0")
                .expect("read signal-domain schema metadata");
        let Some(signal_spirit) =
            DependencySchema::from_cargo_metadata("signal-spirit", "signal-spirit", "0.13.0")
                .expect("read signal-spirit schema metadata")
        else {
            return;
        };

        let plan = GenerationPlan::new(&self.crate_root, "meta-signal-spirit", "0.7.1")
            .with_optional_dependency_schema(signal_domain)
            .with_dependency_schema(signal_spirit)
            .with_module(ModuleEmission::wire_contract_module("meta-signal"));

        GenerationDriver::new(plan)
            .generate()
            .expect("generate meta-signal-spirit schema artifacts")
            .write_or_check("META_SIGNAL_SPIRIT_UPDATE_SCHEMA_ARTIFACTS")
            .expect("checked-in meta-signal-spirit schema artifacts are fresh");
        CargoSchemaMetadata::new("meta-signal-spirit").emit_schema_directory(&self.crate_root);
    }
}
