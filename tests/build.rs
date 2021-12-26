use std::{env, io, path::PathBuf};

fn main() -> io::Result<()> {
    prost_build::Config::new()
        .type_attribute(".test", "#[derive(::proptest_derive::Arbitrary)]")
        .field_attribute(
            ".test.WellKnownTypes.timestamp",
            "#[proptest(strategy = \"::proptest::option::of(crate::arbitrary::timestamp())\")]",
        )
        .field_attribute(
            ".test.WellKnownTypes.duration",
            "#[proptest(strategy = \"::proptest::option::of(crate::arbitrary::duration())\")]",
        )
        .field_attribute(
            ".test.WellKnownTypes.struct",
            "#[proptest(strategy = \"::proptest::option::of(crate::arbitrary::struct_())\")]",
        )
        .field_attribute(
            ".test.WellKnownTypes.list",
            "#[proptest(strategy = \"::proptest::option::of(crate::arbitrary::list())\")]",
        )
        .field_attribute(
            ".test.WellKnownTypes.mask",
            "#[proptest(strategy = \"::proptest::option::of(crate::arbitrary::mask())\")]",
        )
        .field_attribute(
            ".test.WellKnownTypes.empty",
            "#[proptest(strategy = \"::proptest::option::of(::proptest::strategy::Just(()))\")]",
        )
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin"),
        )
        .compile_protos(
            &[
                "src/test.proto",
                "src/test2.proto",
                "src/desc.proto",
                "src/desc_no_package.proto",
            ],
            &["src/"],
        )?;
    Ok(())
}
