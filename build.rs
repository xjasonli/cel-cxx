fn main() {
    #[cfg(feature = "prost")]
    {
        prost_build::Config::new()
            .enable_type_names()
            .type_attribute("test.SimpleMessage", "#[derive(::cel_cxx::ProstValue)]")
            .type_attribute("test.Address", "#[derive(::cel_cxx::ProstValue)]")
            .type_attribute("test.Person", "#[derive(::cel_cxx::ProstValue)]")
            .type_attribute("test.Item", "#[derive(::cel_cxx::ProstValue)]")
            .type_attribute("test.Order", "#[derive(::cel_cxx::ProstValue)]")
            .compile_protos(&["tests/fixtures/test.proto"], &["tests/fixtures/"])
            .expect("prost-build failed");
    }

    #[cfg(feature = "protobuf-legacy")]
    {
        use protobuf_codegen::{Codegen, Customize, CustomizeCallback};

        struct DeriveProtobufLegacyValue;

        impl CustomizeCallback for DeriveProtobufLegacyValue {
            fn message(
                &self,
                _message: &protobuf::reflect::MessageDescriptor,
            ) -> Customize {
                Customize::default().before("#[derive(::cel_cxx::ProtobufLegacyValue)]")
            }
        }

        Codegen::new()
            .cargo_out_dir("protobuf_legacy_gen")
            .input("tests/fixtures/test.proto")
            .include("tests/fixtures/")
            .customize_callback(DeriveProtobufLegacyValue)
            .run_from_script();

        // Strip inner attributes (#![...]) and inner doc comments (//!) so the
        // generated file can be included inside a `mod { }` block in tests.
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let gen_path = format!("{out_dir}/protobuf_legacy_gen/test.rs");
        let content = std::fs::read_to_string(&gen_path).expect("failed to read generated file");

        let mut result = String::new();
        for line in content.lines() {
            if line.starts_with("#!") || line.starts_with("//!") {
                result.push_str("// (stripped) ");
                result.push_str(line);
            } else {
                result.push_str(line);
            }
            result.push('\n');
        }

        std::fs::write(&gen_path, result).expect("failed to write patched file");
    }
}
