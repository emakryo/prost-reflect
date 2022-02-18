#![doc = include_str!("../../doc/intro.md")]
#![doc = "# Example - decoding"]
#![doc = include_str!("../../doc/decoding.md")]
#![cfg_attr(feature = "serde", doc = "# Example - JSON mapping")]
#![cfg_attr(feature = "serde", doc = include_str!("../../doc/json.md"))]
#![cfg_attr(feature = "derive", doc = "# Deriving [`ReflectMessage`]")]
#![cfg_attr(feature = "serde", doc = include_str!("../../doc/derive.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs)]
#![doc(html_root_url = "https://docs.rs/prost-reflect/0.5.7/")]

#[cfg(feature = "serde1")]
extern crate serde1 as serde;

mod descriptor;
mod dynamic;
mod reflect;

pub use self::descriptor::{
    Cardinality, DescriptorError, EnumDescriptor, EnumValueDescriptor, ExtensionDescriptor,
    FieldDescriptor, FileDescriptor, Kind, MessageDescriptor, MethodDescriptor, OneofDescriptor,
    ServiceDescriptor,
};
pub use self::dynamic::{DynamicMessage, MapKey, Value};
pub use self::reflect::ReflectMessage;

#[cfg(feature = "serde")]
pub use self::dynamic::{DeserializeOptions, SerializeOptions};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use prost_reflect_derive::ReflectMessage;
