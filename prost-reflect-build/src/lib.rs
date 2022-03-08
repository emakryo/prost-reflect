//! `prost-reflect-build` contains [`Builder`] to configure [`prost_build::Config`]
//! to derive [`prost_reflect::ReflectMessage`] for all messages in protocol buffers.
//!
//! The simplest way to generate procotol buffer API:
//!
//! ```no_run
//! // build.rs
//! use prost_reflect_build::Builder;
//!
//! Builder::new()
//!     .compile_protos(&["path/to/protobuf.proto"], &["path/to/include"])
//!     .expect("Failed to compile protos");
//! ```
//!
//! With default configuration, `lib.rs` must include the following lines for reflection.
//!
//! ```ignore
//! static FILE_DESCRIPTOR: Lazy<FileDescriptor> = Lazy::new(|| FileDescriptor::decode(
//!     include_bytes!(concat!(env!("OUT_DIR"), "file_descriptor_set.bin")).as_ref()
//! ).unwrap());
//!
//! // `include!` generated code may appear anywhere in the crate.
//! include!(conca!(env!("OUT_DIR"), "protobuf.rs"));
//!
//! ```

use std::{
    io::Read,
    path::{Path, PathBuf},
};

use prost_reflect::FileDescriptor;

/// Configuration builder for prost-reflect code generation.
///
/// The simplest way to generate prost APIs deriving [`prost_reflect::ReflectMessage`]:
///
/// ```no_run
/// # use prost_reflect_build::Builder;
/// Builder::new()
///     .compile_protos(&["path/to/protobuf.proto"], &["path/to/include"])
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Builder {
    file_descriptor_set_path: PathBuf,
    file_descriptor_expr: String,
}

impl Default for Builder {
    fn default() -> Self {
        let file_descriptor_set_path =
            PathBuf::from(std::env::var("OUT_DIR").unwrap_or_else(|_| String::from(".")))
                .join("file_descriptor_set.bin");

        Self {
            file_descriptor_set_path,
            file_descriptor_expr: "crate::FILE_DESCRIPTOR".into(),
        }
    }
}

impl Builder {
    /// Create a new builder with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the path where the encoded file descriptor set is created.
    /// By default, it is created at `$OUT_DIR/file_descriptor_set.bin`.
    ///
    /// This overrides the path specified by
    /// [`prost_build::Config::file_descriptor_set_path`].
    pub fn file_descriptor_set_path<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.file_descriptor_set_path = path.into();
        self
    }

    /// Set the file descriptor expression for reflection.
    /// By default, `crate::FILE_DESCRIPTOR` is used as the expression.
    /// In that case, `lib.rs` should contain the following lines,
    ///
    /// ```ignore
    /// static FILE_DESCRIPTOR: Lazy<FileDescriptor> = Lazy::new(||
    ///     FileDescriptor::decode(include_bytes!(
    ///         concat!(env!("OUT_DIR"), "file_descriptor_set.bin")
    ///     ).as_ref()).unwrap()
    /// );
    /// ```
    pub fn file_descriptor_expr<P>(&mut self, expr: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.file_descriptor_expr = expr.into();
        self
    }

    /// Configure `config` to derive [`prost_reflect::ReflectMessage`] for all messages included in `protos`.
    /// This method does not generate prost-reflect compatible code,
    /// but `config` may be used later to compile protocol buffers independently of [`Builder`].
    /// `protos` and `includes` should be the same when [`prost_build::Config::compile_protos`] is called on `config`.
    ///
    /// ```ignore
    /// let mut config = Config::new();
    ///
    /// // Customize config here
    ///
    /// Builder::new()
    ///     .configure(&mut config, &["path/to/protobuf.proto"], &["path/to/include"])
    ///     .expect("Failed to configure for reflection");
    ///
    /// // Custom compilation process with `config`
    /// config.compile_protos(&["path/to/protobuf.proto"], &["path/to/includes"])
    ///     .expect("Failed to compile protocol buffers");
    /// ```
    pub fn configure(
        &mut self,
        config: &mut prost_build::Config,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> std::io::Result<()> {
        config
            .file_descriptor_set_path(&self.file_descriptor_set_path)
            .compile_protos(protos, includes)?;

        let mut buf = vec![];
        std::fs::File::open(&self.file_descriptor_set_path)?.read_to_end(&mut buf)?;
        let descriptor = FileDescriptor::decode(buf.as_ref()).expect("Invalid file descriptor");

        for message in descriptor.all_messages() {
            let full_name = message.full_name();
            config
                .type_attribute(full_name, "#[derive(::prost_reflect::ReflectMessage)]")
                .type_attribute(
                    full_name,
                    &format!(
                        r#"#[prost_reflect(file_descriptor = "{}", message_name = "{}")]"#,
                        self.file_descriptor_expr, full_name,
                    ),
                );
        }

        Ok(())
    }

    /// Compile protocol buffers into Rust with given [`prost_build::Config`].
    pub fn compile_protos_with_config(
        &mut self,
        mut config: prost_build::Config,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> std::io::Result<()> {
        self.configure(&mut config, protos, includes)?;

        config.skip_protoc_run().compile_protos(protos, includes)
    }

    /// Compile protocol buffers into Rust.
    pub fn compile_protos(
        &mut self,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> std::io::Result<()> {
        self.compile_protos_with_config(prost_build::Config::new(), protos, includes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let mut config = prost_build::Config::new();
        let mut builder = Builder::new();
        let tmpdir = std::env::temp_dir();
        config.out_dir(tmpdir.clone());

        builder
            .file_descriptor_set_path(tmpdir.join("file_descriptor_est.bin"))
            .compile_protos_with_config(config, &["src/test.proto"], &["src"])
            .unwrap();

        assert!(tmpdir.join("my.test.rs").exists());

        let mut buf = String::new();
        std::fs::File::open(tmpdir.join("my.test.rs"))
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();

        let num_derive = buf
            .lines()
            .filter(|line| line.trim_start() == "#[derive(::prost_reflect::ReflectMessage)]")
            .count();

        assert_eq!(num_derive, 3);
    }
}