#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Status {
    pub status: Option<String>,
}

impl Status {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> StatusBuilder {
        StatusBuilder {
            body: Default::default(),
        }
    }

    /// Delete multiple pets
    #[inline]
    pub fn delete() -> StatusDeleteBuilder<crate::generics::MissingPetId> {
        StatusDeleteBuilder {
            inner: Default::default(),
            _param_pet_id: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn put_1() -> StatusPutBuilder1<crate::generics::MissingSomeDataFile, crate::generics::MissingFoobar> {
        StatusPutBuilder1 {
            inner: Default::default(),
            _param_some_data_file: core::marker::PhantomData,
            _param_foobar: core::marker::PhantomData,
        }
    }
}

impl Into<Status> for StatusBuilder {
    fn into(self) -> Status {
        self.body
    }
}

/// Builder for [`Status`](./struct.Status.html) object.
#[derive(Debug, Clone)]
pub struct StatusBuilder {
    body: self::Status,
}

impl StatusBuilder {
    #[inline]
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.body.status = Some(value.into());
        self
    }
}

/// Builder created by [`Status::delete`](./struct.Status.html#method.delete) method for a `DELETE` operation associated with `Status`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusDeleteBuilder<PetId> {
    inner: StatusDeleteBuilderContainer,
    _param_pet_id: core::marker::PhantomData<PetId>,
}

#[derive(Debug, Default, Clone)]
struct StatusDeleteBuilderContainer {
    param_pet_id: Option<crate::util::Delimited<i64, crate::util::Csv>>,
}

impl<PetId> StatusDeleteBuilder<PetId> {
    #[inline]
    pub fn pet_id(mut self, value: impl Iterator<Item = impl Into<i64>>) -> StatusDeleteBuilder<crate::generics::PetIdExists> {
        self.inner.param_pet_id = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StatusDeleteBuilder<crate::generics::PetIdExists> {
    type Output = Status;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/pets/{petId}", petId=self.inner.param_pet_id.as_ref().expect("missing parameter pet_id?")).into()
    }
}

/// Builder created by [`Status::put_1`](./struct.Status.html#method.put_1) method for a `PUT` operation associated with `Status`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusPutBuilder1<SomeDataFile, Foobar> {
    inner: StatusPutBuilder1Container,
    _param_some_data_file: core::marker::PhantomData<SomeDataFile>,
    _param_foobar: core::marker::PhantomData<Foobar>,
}

#[derive(Debug, Default, Clone)]
struct StatusPutBuilder1Container {
    param_some_data_file: Option<std::path::PathBuf>,
    param_some_other_file: Option<std::path::PathBuf>,
    param_foobar: Option<String>,
    param_booya: Option<crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>>,
}

impl<SomeDataFile, Foobar> StatusPutBuilder1<SomeDataFile, Foobar> {
    #[inline]
    pub fn some_data_file(mut self, value: impl AsRef<std::path::Path>) -> StatusPutBuilder1<crate::generics::SomeDataFileExists, Foobar> {
        self.inner.param_some_data_file = Some(value.as_ref().into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn some_other_file(mut self, value: impl AsRef<std::path::Path>) -> Self {
        self.inner.param_some_other_file = Some(value.as_ref().into());
        self
    }

    #[inline]
    pub fn foobar(mut self, value: impl Into<String>) -> StatusPutBuilder1<SomeDataFile, crate::generics::FoobarExists> {
        self.inner.param_foobar = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn booya(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<i64>>>) -> Self {
        self.inner.param_booya = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StatusPutBuilder1<crate::generics::SomeDataFileExists, crate::generics::FoobarExists> {
    type Output = Status;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/test/file".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .multipart_form_data({
            use crate::client::Form;
            let mut form = <Client::Request as Request>::Form::new();
            if let Some(v) = self.inner.param_some_data_file.as_ref() {
                form = form.file("someDataFile", v)?;
            }
            if let Some(v) = self.inner.param_some_other_file.as_ref() {
                form = form.file("someOtherFile", v)?;
            }
            if let Some(v) = self.inner.param_foobar.as_ref() {
                form = form.text("foobar", v.to_string());
            }
            if let Some(stuff) = self.inner.param_booya.as_ref() {
                for v in stuff.iter() {
                    form = form.text("booya", v.to_string());
                }
            }
            form
        }))
    }
}
