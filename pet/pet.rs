
/// A pet is a person's best friend
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Pet<Any> {
    pub category: Option<crate::category::Category>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Option<Vec<String>>,
    pub tags: Option<Vec<crate::tag::Tag>>,
    #[serde(flatten)]
    pub other_fields: Option<std::collections::BTreeMap<String, Any>>,
}

impl<Any: Default> Pet<Any> {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PetBuilder<crate::generics::MissingId, crate::generics::MissingName, Any> {
        PetBuilder {
            body: Default::default(),
            _id: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }

    /// Fetch list of pets
    #[inline]
    pub fn list_pets() -> PetGetBuilder {
        PetGetBuilder
    }

    /// Add a new pet to the store
    #[inline]
    pub fn add_pet() -> PetPostBuilder<crate::generics::MissingXAuth, crate::generics::MissingId, crate::generics::MissingName, Any> {
        PetPostBuilder {
            inner: Default::default(),
            _param_x_auth: core::marker::PhantomData,
            _id: core::marker::PhantomData,
            _name: core::marker::PhantomData,
        }
    }

    /// Find pet by ID
    #[inline]
    pub fn get_pet_by_id() -> PetGetBuilder1<crate::generics::MissingPetId> {
        PetGetBuilder1 {
            inner: Default::default(),
            _param_pet_id: core::marker::PhantomData,
        }
    }
}

impl<Any> Into<Pet<Any>> for PetBuilder<crate::generics::IdExists, crate::generics::NameExists, Any> {
    fn into(self) -> Pet<Any> {
        self.body
    }
}

impl<Any> Into<Pet<Any>> for PetPostBuilder<crate::generics::XAuthExists, crate::generics::IdExists, crate::generics::NameExists, Any> {
    fn into(self) -> Pet<Any> {
        self.inner.body
    }
}

/// Builder for [`Pet`](./struct.Pet.html) object.
#[derive(Debug, Clone)]
pub struct PetBuilder<Id, Name, Any> {
    body: self::Pet<Any>,
    _id: core::marker::PhantomData<Id>,
    _name: core::marker::PhantomData<Name>,
}

impl<Id, Name, Any> PetBuilder<Id, Name, Any> {
    #[inline]
    pub fn category(mut self, value: crate::category::Category) -> Self {
        self.body.category = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PetBuilder<crate::generics::IdExists, Name, Any> {
        self.body.id = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> PetBuilder<Id, crate::generics::NameExists, Any> {
        self.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn photo_urls(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.photo_urls = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn tags(mut self, value: impl Iterator<Item = crate::tag::Tag>) -> Self {
        self.body.tags = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn other_fields(mut self, value: impl Iterator<Item = (String, impl Into<Any>)>) -> Self {
        self.body.other_fields = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }
}

/// Builder created by [`Pet::list_pets`](./struct.Pet.html#method.list_pets) method for a `GET` operation associated with `Pet`.
#[derive(Debug, Clone)]
pub struct PetGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PetGetBuilder {
    type Output = Vec<Pet<serde_yaml::Value>>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/pets".into()
    }
}

/// Builder created by [`Pet::add_pet`](./struct.Pet.html#method.add_pet) method for a `POST` operation associated with `Pet`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PetPostBuilder<XAuth, Id, Name, Any> {
    inner: PetPostBuilderContainer<Any>,
    _param_x_auth: core::marker::PhantomData<XAuth>,
    _id: core::marker::PhantomData<Id>,
    _name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct PetPostBuilderContainer<Any> {
    body: self::Pet<Any>,
    param_x_auth: Option<String>,
    param_x_pet_id: Option<i64>,
}

impl<XAuth, Id, Name, Any> PetPostBuilder<XAuth, Id, Name, Any> {
    #[inline]
    pub fn x_auth(mut self, value: impl Into<String>) -> PetPostBuilder<crate::generics::XAuthExists, Id, Name, Any> {
        self.inner.param_x_auth = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn x_pet_id(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_x_pet_id = Some(value.into());
        self
    }

    #[inline]
    pub fn category(mut self, value: crate::category::Category) -> Self {
        self.inner.body.category = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> PetPostBuilder<XAuth, crate::generics::IdExists, Name, Any> {
        self.inner.body.id = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> PetPostBuilder<XAuth, Id, crate::generics::NameExists, Any> {
        self.inner.body.name = value.into();
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn photo_urls(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.inner.body.photo_urls = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn tags(mut self, value: impl Iterator<Item = crate::tag::Tag>) -> Self {
        self.inner.body.tags = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn other_fields(mut self, value: impl Iterator<Item = (String, impl Into<Any>)>) -> Self {
        self.inner.body.other_fields = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static, Any: serde::Serialize> crate::client::Sendable<Client> for PetPostBuilder<crate::generics::XAuthExists, crate::generics::IdExists, crate::generics::NameExists, Any> {
    type Output = crate::pet::Pet<serde_yaml::Value>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/pets".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        let mut req = req;
        req = req.header("X-Auth", &self.inner.param_x_auth.as_ref().map(std::string::ToString::to_string).expect("missing parameter x_auth?"));
        if let Some(v) = &self.inner.param_x_pet_id.as_ref().map(std::string::ToString::to_string) {
            req = req.header("X-Pet-ID", &v);
        }

        Ok(req
        .header(http::header::CONTENT_TYPE.as_str(), "application/yaml")
        .body_bytes({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.inner.body)?;
            vec
        })
        .header(http::header::ACCEPT.as_str(), "application/yaml"))
    }
}

impl<Any> crate::client::ResponseWrapper<crate::pet::Pet<serde_yaml::Value>, PetPostBuilder<crate::generics::XAuthExists, crate::generics::IdExists, crate::generics::NameExists, Any>> {
    /// Maximum allowed requests in the current period
    #[inline]
    pub fn x_rate_limit(&self) -> Option<i64> {
        self.headers.get("x-rate-limit").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    /// Whether the requests have exceeded for this window.
    #[inline]
    pub fn x_rate_limit_exceeded(&self) -> Option<bool> {
        self.headers.get("x-rate-limit-exceeded").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    /// Remaining requests in the current period
    #[inline]
    pub fn x_rate_limit_remaining(&self) -> Option<i64> {
        self.headers.get("x-rate-limit-remaining").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    /// Time at which rate limit is reset (in UNIX epoch)
    #[inline]
    pub fn x_rate_limit_reset(&self) -> Option<i64> {
        self.headers.get("x-rate-limit-reset").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn x_array(&self) -> Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<f64, crate::util::Ssv>, crate::util::Tsv>, crate::util::Csv>, crate::util::Csv>> {
        self.headers.get("x-array").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
    #[inline]
    pub fn x_string(&self) -> Option<String> {
        self.headers.get("x-string").and_then(|v| String::from_utf8_lossy(v.as_ref()).parse().ok())
    }
}

/// Builder created by [`Pet::get_pet_by_id`](./struct.Pet.html#method.get_pet_by_id) method for a `GET` operation associated with `Pet`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PetGetBuilder1<PetId> {
    inner: PetGetBuilder1Container,
    _param_pet_id: core::marker::PhantomData<PetId>,
}

#[derive(Debug, Default, Clone)]
struct PetGetBuilder1Container {
    param_pet_id: Option<i64>,
}

impl<PetId> PetGetBuilder1<PetId> {
    /// ID of the pet.
    #[inline]
    pub fn pet_id(mut self, value: impl Into<i64>) -> PetGetBuilder1<crate::generics::PetIdExists> {
        self.inner.param_pet_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PetGetBuilder1<crate::generics::PetIdExists> {
    type Output = Pet<serde_yaml::Value>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/pets/{petId}", petId=self.inner.param_pet_id.as_ref().expect("missing parameter pet_id?")).into()
    }
}
