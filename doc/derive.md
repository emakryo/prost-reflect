The [`ReflectMessage`] trait provides a `.descriptor()` method to get type information for a message.

When the `derive` feature is enabled, it can be derived for [`Message`][prost::Message] implementations. The
derive macro takes the following parameters:

| Name            | Value |
|-----------------|-------|
| file_descriptor | An expression that resolves to a [`FileDescriptor`] containing the message type. The descriptor should be cached to avoid re-building it. |
| message_name    | The full name of the message, used to look it up within [`FileDescriptor`]. |

```rust
use prost::Message;
use prost_reflect::{FileDescriptor, ReflectMessage};
use once_cell::sync::Lazy;

static FILE_DESCRIPTOR: Lazy<FileDescriptor>
    = Lazy::new(|| FileDescriptor::decode(include_bytes!("file_descriptor_set.bin").as_ref()).unwrap());

#[derive(Message, ReflectMessage)]
#[prost_reflect(file_descriptor = "FILE_DESCRIPTOR", message_name = "package.MyMessage")]
pub struct MyMessage {}

let message = MyMessage {};
assert_eq!(message.descriptor().full_name(), "package.MyMessage");
```

If you are using `prost-build`, it can be configured to generate [`ReflectMessage`] implementations
for messages:

```rust,no_run
use prost_build::Config;

Config::new()
    .file_descriptor_set_path("file_descriptor_set.bin")
    .type_attribute(".package.MyMessage", "#[derive(::prost_reflect::ReflectMessage)]")
    .type_attribute(".package.MyMessage", "#[prost_reflect(file_descriptor = \"FILE_DESCRIPTOR\", message_name = \"package.MyMessage\")]")
    .compile_protos(&["src/package.proto"], &["src"])
    .unwrap();
```