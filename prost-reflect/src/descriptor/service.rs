use std::{borrow::Borrow, fmt, sync::Arc};

use prost_types::{FileDescriptorProto, MethodDescriptorProto, ServiceDescriptorProto};

use crate::descriptor::{debug_fmt_iter, FileDescriptorInner};

use super::{
    make_full_name, parse_name, parse_namespace, ty, DescriptorError, FileDescriptor,
    MessageDescriptor,
};

/// A protobuf service definition.
#[derive(Clone)]
pub struct ServiceDescriptor<I = Arc<FileDescriptorInner>> {
    file_descriptor: FileDescriptor<I>,
    index: usize,
}

pub(super) struct ServiceDescriptorInner {
    full_name: Box<str>,
    methods: Box<[MethodDescriptorInner]>,
}

/// A method definition for a [`ServiceDescriptor`].
#[derive(Clone)]
pub struct MethodDescriptor<I = Arc<FileDescriptorInner>> {
    service: ServiceDescriptor<I>,
    index: usize,
}

struct MethodDescriptorInner {
    full_name: Box<str>,
    request_ty: ty::TypeId,
    response_ty: ty::TypeId,
    server_streaming: bool,
    client_streaming: bool,
}

impl<I> ServiceDescriptor<I>
where
    I: Borrow<FileDescriptorInner> + Clone,
{
    /// Create a new [`ServiceDescriptor`] referencing the service at `index` within the given [`FileDescriptor`].
    ///
    /// # Panics
    ///
    /// Panics if `index` is out-of-bounds.
    pub fn new(file_descriptor: FileDescriptor<I>, index: usize) -> Self {
        debug_assert!(index < file_descriptor.services().len());
        ServiceDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Returns the index of this [`ServiceDescriptor`] within the parent [`FileDescriptor`].
    pub fn index(&self) -> usize {
        self.index
    }

    /// Gets a reference to the [`FileDescriptor`] this service is defined in.
    pub fn parent_file(&self) -> &FileDescriptor<I> {
        &self.file_descriptor
    }

    /// Gets the short name of the service, e.g. `MyService`.
    pub fn name(&self) -> &str {
        parse_name(self.full_name())
    }

    /// Gets the full name of the service, e.g. `my.package.Service`.
    pub fn full_name(&self) -> &str {
        &self.inner().full_name
    }

    /// Gets the name of the package this service is defined in, e.g. `my.package`.
    ///
    /// If no package name is set, an empty string is returned.
    pub fn package_name(&self) -> &str {
        parse_namespace(self.full_name())
    }

    /// Gets an iterator yielding a [`MethodDescriptor`] for each method defined in this service.
    pub fn methods(&self) -> impl ExactSizeIterator<Item = MethodDescriptor<I>> {
        let this = self.clone();
        (0..self.inner().methods.len()).map(move |index| MethodDescriptor {
            service: this.clone(),
            index,
        })
    }

    pub fn borrow(&self) -> ServiceDescriptor<&'_ FileDescriptorInner> {
        ServiceDescriptor {
            file_descriptor: self.file_descriptor.borrow(),
            index: self.index,
        }
    }

    fn inner(&self) -> &ServiceDescriptorInner {
        &self.parent_file().inner.borrow().services[self.index]
    }
}

impl ServiceDescriptorInner {
    pub(super) fn from_raw(
        raw_file: &FileDescriptorProto,
        raw_service: &ServiceDescriptorProto,
        type_map: &ty::TypeMap,
    ) -> Result<ServiceDescriptorInner, DescriptorError> {
        let full_name = make_full_name(raw_file.package(), raw_service.name());
        let methods = raw_service
            .method
            .iter()
            .map(|raw_method| {
                MethodDescriptorInner::from_raw(
                    &full_name,
                    raw_file,
                    raw_service,
                    raw_method,
                    type_map,
                )
            })
            .collect::<Result<_, DescriptorError>>()?;
        Ok(ServiceDescriptorInner { full_name, methods })
    }
}

impl<I> fmt::Debug for ServiceDescriptor<I>
where
    I: Borrow<FileDescriptorInner> + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceDescriptor")
            .field("name", &self.name())
            .field("full_name", &self.full_name())
            .field("index", &self.index())
            .field("methods", &debug_fmt_iter(self.methods()))
            .finish()
    }
}

impl<I> PartialEq for ServiceDescriptor<I>
where
    I: Borrow<FileDescriptorInner>,
{
    fn eq(&self, other: &Self) -> bool {
        self.file_descriptor == other.file_descriptor && self.index == other.index
    }
}

impl<I> Eq for ServiceDescriptor<I> where I: Borrow<FileDescriptorInner> {}

impl<I> MethodDescriptor<I>
where
    I: Borrow<FileDescriptorInner> + Clone,
{
    /// Create a new [`MethodDescriptor`] referencing the method at `index` within the [`ServiceDescriptor`].
    ///
    /// # Panics
    ///
    /// Panics if `index` is out-of-bounds.
    pub fn new(service: ServiceDescriptor<I>, index: usize) -> Self {
        debug_assert!(index < service.methods().len());
        MethodDescriptor { service, index }
    }

    /// Gets the index of the method within the parent [`ServiceDescriptor`].
    pub fn index(&self) -> usize {
        self.index
    }

    /// Gets a reference to the [`ServiceDescriptor`] this method is defined in.
    pub fn parent_service(&self) -> &ServiceDescriptor<I> {
        &self.service
    }

    /// Gets a reference to the [`FileDescriptor`] this method is defined in.
    pub fn parent_file(&self) -> &FileDescriptor<I> {
        self.service.parent_file()
    }

    /// Gets the short name of the method, e.g. `method`.
    pub fn name(&self) -> &str {
        parse_name(self.full_name())
    }

    /// Gets the full name of the method, e.g. `my.package.MyService.my_method`.
    pub fn full_name(&self) -> &str {
        &self.inner().full_name
    }

    /// Gets the [`MessageDescriptor`] for the input type of this method.
    pub fn input(&self) -> MessageDescriptor<I> {
        MessageDescriptor::new(self.parent_file().clone(), self.inner().request_ty)
    }

    /// Gets the [`MessageDescriptor`] for the output type of this method.
    pub fn output(&self) -> MessageDescriptor<I> {
        MessageDescriptor::new(self.parent_file().clone(), self.inner().response_ty)
    }

    /// Returns `true` if the client streams multiple messages.
    pub fn is_client_streaming(&self) -> bool {
        self.inner().client_streaming
    }

    /// Returns `true` if the server streams multiple messages.
    pub fn is_server_streaming(&self) -> bool {
        self.inner().server_streaming
    }

    pub fn borrow(&self) -> MethodDescriptor<&'_ FileDescriptorInner> {
        MethodDescriptor {
            service: self.service.borrow(),
            index: self.index,
        }
    }

    fn inner(&self) -> &MethodDescriptorInner {
        &self.service.inner().methods[self.index]
    }
}

impl MethodDescriptorInner {
    fn from_raw(
        namespace: &str,
        _raw_file: &FileDescriptorProto,
        _raw_service: &ServiceDescriptorProto,
        raw_method: &MethodDescriptorProto,
        type_map: &ty::TypeMap,
    ) -> Result<MethodDescriptorInner, DescriptorError> {
        let request_ty = type_map.resolve_type_name(namespace, raw_method.input_type())?;
        let response_ty = type_map.resolve_type_name(namespace, raw_method.output_type())?;

        Ok(MethodDescriptorInner {
            full_name: make_full_name(namespace, raw_method.name()),
            request_ty,
            response_ty,
            client_streaming: raw_method.client_streaming(),
            server_streaming: raw_method.server_streaming(),
        })
    }
}

impl<I> fmt::Debug for MethodDescriptor<I>
where
    I: Borrow<FileDescriptorInner> + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MethodDescriptor")
            .field("name", &self.name())
            .field("full_name", &self.full_name())
            .field("index", &self.index())
            .field("input", &self.input())
            .field("output", &self.output())
            .field("is_client_streaming", &self.is_client_streaming())
            .field("is_server_streaming", &self.is_server_streaming())
            .finish()
    }
}

impl<I> PartialEq for MethodDescriptor<I>
where
    I: Borrow<FileDescriptorInner>,
{
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service && self.index == other.index
    }
}

impl<I> Eq for MethodDescriptor<I> where I: Borrow<FileDescriptorInner> {}
