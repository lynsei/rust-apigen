#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostShipmentsBody {
    pub address: Option<crate::post_shipments_body::PostShipmentsBodyAddress>,
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostShipmentsBodyAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}

impl PostShipmentsBody {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostShipmentsBodyBuilder {
        PostShipmentsBodyBuilder {
            body: Default::default(),
        }
    }

    /// Create shipment for order
    #[deprecated]
    #[inline]
    pub fn post() -> PostShipmentsBodyPostBuilder {
        PostShipmentsBodyPostBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PostShipmentsBody> for PostShipmentsBodyBuilder {
    fn into(self) -> PostShipmentsBody {
        self.body
    }
}

impl Into<PostShipmentsBody> for PostShipmentsBodyPostBuilder {
    fn into(self) -> PostShipmentsBody {
        self.body
    }
}

/// Builder for [`PostShipmentsBody`](./struct.PostShipmentsBody.html) object.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyBuilder {
    body: self::PostShipmentsBody,
}

impl PostShipmentsBodyBuilder {
    #[inline]
    pub fn address(mut self, value: crate::post_shipments_body::PostShipmentsBodyAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }
}

/// Builder created by [`PostShipmentsBody::post`](./struct.PostShipmentsBody.html#method.post) method for a `POST` operation associated with `PostShipmentsBody`.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyPostBuilder {
    body: self::PostShipmentsBody,
}

impl PostShipmentsBodyPostBuilder {
    #[inline]
    pub fn address(mut self, value: crate::post_shipments_body::PostShipmentsBodyAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostShipmentsBodyPostBuilder {
    type Output = serde_yaml::Value;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/shipments".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .header(http::header::CONTENT_TYPE.as_str(), "application/yaml")
        .body_bytes({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.body)?;
            vec
        })
        .header(http::header::ACCEPT.as_str(), "application/yaml"))
    }
}

impl PostShipmentsBodyAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostShipmentsBodyAddressBuilder {
        PostShipmentsBodyAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PostShipmentsBodyAddress> for PostShipmentsBodyAddressBuilder {
    fn into(self) -> PostShipmentsBodyAddress {
        self.body
    }
}

/// Builder for [`PostShipmentsBodyAddress`](./struct.PostShipmentsBodyAddress.html) object.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyAddressBuilder {
    body: self::PostShipmentsBodyAddress,
}

impl PostShipmentsBodyAddressBuilder {
    #[inline]
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.body.code = Some(value.into());
        self
    }

    #[inline]
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.body.line1 = Some(value.into());
        self
    }

    #[inline]
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.body.line2 = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}
