[![crates.io](https://img.shields.io/crates/v/prost-reflect.svg)](https://crates.io/crates/prost-reflect/)
[![docs.rs](https://docs.rs/prost-reflect/badge.svg)](https://docs.rs/prost-reflect/)
[![deps.rs](https://deps.rs/crate/prost-reflect/0.5.7/status.svg)](https://deps.rs/crate/prost-reflect)
![MSRV](https://img.shields.io/badge/rustc-1.53+-blue.svg)
[![Continuous integration](https://github.com/andrewhickman/prost-reflect/actions/workflows/ci.yml/badge.svg)](https://github.com/andrewhickman/prost-reflect/actions/workflows/ci.yml)
[![codecov.io](https://codecov.io/gh/andrewhickman/prost-reflect/branch/main/graph/badge.svg?token=E2OITYXO7M)](https://codecov.io/gh/andrewhickman/prost-reflect)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

# prost-reflect

A protobuf library extending [`prost`](https://crates.io/crates/prost) with reflection support and dynamic messages.

## Usage

This crate provides support for dynamic protobuf messages. These are useful when the
protobuf type definition is not known ahead of time.

The main entry points into the API of this crate are:

- [`FileDescriptor`] wraps a [`FileDescriptorSet`][prost_types::FileDescriptorSet] output by 
  the protobuf compiler to provide an API for inspecting type definitions.
- [`DynamicMessage`] provides encoding, decoding and reflection of an arbitrary protobuf 
  message definition described by a [`MessageDescriptor`].

### Example - decoding

`DynamicMessage` does not implement [`Default`] since it needs a message descriptor to
function. To decode a protobuf byte stream into an instance of this type, use [`DynamicMessage::decode`]
to create a default value for the `MessageDescriptor` instance and merge into it:

```rust
use prost::Message;
use prost_types::FileDescriptorSet;
use prost_reflect::{DynamicMessage, FileDescriptor, Value};

let file_descriptor_set = FileDescriptorSet::decode(include_bytes!("file_descriptor_set.bin").as_ref()).unwrap();
let file_descriptor = FileDescriptor::new(file_descriptor_set).unwrap();
let message_descriptor = file_descriptor.get_message_by_name("package.MyMessage").unwrap();

let dynamic_message = DynamicMessage::decode(message_descriptor, b"\x08\x96\x01".as_ref()).unwrap();

assert_eq!(dynamic_message.get_field_by_name("foo").unwrap().as_ref(), &Value::I32(150));
```

### Example - JSON mapping

When the `serde` feature is enabled, `DynamicMessage` can be deserialized to and from the
[canonical JSON mapping](https://developers.google.com/protocol-buffers/docs/proto3#json)
defined for protobuf messages.

```rust
use prost::Message;
use prost_reflect::{DynamicMessage, FileDescriptor, Value};
use serde_json::de::Deserializer;

let file_descriptor = FileDescriptor::decode(include_bytes!("file_descriptor_set.bin").as_ref()).unwrap();
let message_descriptor = file_descriptor.get_message_by_name("package.MyMessage").unwrap();

let json = r#"{ "foo": 150 }"#;
let mut deserializer = Deserializer::from_str(json);
let dynamic_message = DynamicMessage::deserialize(message_descriptor, &mut deserializer).unwrap();
deserializer.end().unwrap();

assert_eq!(dynamic_message.get_field_by_name("foo").unwrap().as_ref(), &Value::I32(150));
```

## Minimum Supported Rust Version

Rust **1.53** or higher.

The minimum supported Rust version may be changed in the future, but it will be
done with a minor version bump.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`FileDescriptor`]: https://docs.rs/prost-reflect/0.5.7/prost_reflect/struct.FileDescriptor.html
[`DynamicMessage`]: https://docs.rs/prost-reflect/0.5.7/prost_reflect/struct.DynamicMessage.html
[`MessageDescriptor`]: https://docs.rs/prost-reflect/0.5.7/prost_reflect/struct.MessageDescriptor.html
[`MessageDescriptor`]: https://docs.rs/prost-reflect/0.5.7/prost_reflect/struct.MessageDescriptor.html
[`DynamicMessage::decode`]: https://docs.rs/prost-reflect/0.5.7/prost_reflect/struct.DynamicMessage.html#method.decode

[`Default`]: https://doc.rust-lang.org/stable/core/default/trait.Default.html
[prost_types::FileDescriptorSet]: https://docs.rs/prost-types/latest/prost_types/struct.FileDescriptorSet.html
