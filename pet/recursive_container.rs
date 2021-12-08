#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RecursiveContainer<Any> {
    pub object: Option<Box<crate::recursive_object::RecursiveObject<Any>>>,
}

impl<Any: Default> RecursiveContainer<Any> {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RecursiveContainerBuilder<Any> {
        RecursiveContainerBuilder {
            body: Default::default(),
        }
    }

    #[inline]
    pub fn get() -> RecursiveContainerGetBuilder {
        RecursiveContainerGetBuilder
    }

    #[inline]
    pub fn post_1() -> RecursiveContainerPostBuilder1 {
        RecursiveContainerPostBuilder1
    }
}

impl<Any> Into<RecursiveContainer<Any>> for RecursiveContainerBuilder<Any> {
    fn into(self) -> RecursiveContainer<Any> {
        self.body
    }
}

/// Builder for [`RecursiveContainer`](./struct.RecursiveContainer.html) object.
#[derive(Debug, Clone)]
pub struct RecursiveContainerBuilder<Any> {
    body: self::RecursiveContainer<Any>,
}

impl<Any> RecursiveContainerBuilder<Any> {
    #[inline]
    pub fn object(mut self, value: crate::recursive_object::RecursiveObject<Any>) -> Self {
        self.body.object = Some(value.into());
        self
    }
}

/// Builder created by [`RecursiveContainer::get`](./struct.RecursiveContainer.html#method.get) method for a `GET` operation associated with `RecursiveContainer`.
#[derive(Debug, Clone)]
pub struct RecursiveContainerGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RecursiveContainerGetBuilder {
    type Output = RecursiveContainer<serde_yaml::Value>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/another/route/referring/recursive/object".into()
    }
}

/// Builder created by [`RecursiveContainer::post_1`](./struct.RecursiveContainer.html#method.post_1) method for a `POST` operation associated with `RecursiveContainer`.
#[derive(Debug, Clone)]
pub struct RecursiveContainerPostBuilder1;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for RecursiveContainerPostBuilder1 {
    type Output = RecursiveContainer<serde_yaml::Value>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/route/referring/recursive/object".into()
    }
}
