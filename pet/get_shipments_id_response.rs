#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetShipmentsIdResponse {
    pub address: Option<crate::get_shipments_id_response::GetShipmentsIdResponseAddress>,
    #[serde(rename = "createdOn")]
    pub created_on: Option<String>,
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    #[serde(rename = "shippedOn")]
    pub shipped_on: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetShipmentsIdResponseAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}

impl GetShipmentsIdResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetShipmentsIdResponseBuilder {
        GetShipmentsIdResponseBuilder {
            body: Default::default(),
        }
    }

    /// Fetch shipment by ID
    #[inline]
    pub fn get_shipment() -> GetShipmentsIdResponseGetBuilder<crate::generics::MissingId> {
        GetShipmentsIdResponseGetBuilder {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<GetShipmentsIdResponse> for GetShipmentsIdResponseBuilder {
    fn into(self) -> GetShipmentsIdResponse {
        self.body
    }
}

/// Builder for [`GetShipmentsIdResponse`](./struct.GetShipmentsIdResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseBuilder {
    body: self::GetShipmentsIdResponse,
}

impl GetShipmentsIdResponseBuilder {
    #[inline]
    pub fn address(mut self, value: crate::get_shipments_id_response::GetShipmentsIdResponseAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn created_on(mut self, value: impl Into<String>) -> Self {
        self.body.created_on = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }

    #[inline]
    pub fn shipped_on(mut self, value: impl Into<String>) -> Self {
        self.body.shipped_on = Some(value.into());
        self
    }
}

/// Builder created by [`GetShipmentsIdResponse::get_shipment`](./struct.GetShipmentsIdResponse.html#method.get_shipment) method for a `GET` operation associated with `GetShipmentsIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseGetBuilder<Id> {
    inner: GetShipmentsIdResponseGetBuilderContainer,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GetShipmentsIdResponseGetBuilderContainer {
    param_id: Option<String>,
}

impl<Id> GetShipmentsIdResponseGetBuilder<Id> {
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> GetShipmentsIdResponseGetBuilder<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetShipmentsIdResponseGetBuilder<crate::generics::IdExists> {
    type Output = GetShipmentsIdResponse;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/shipments/{id}", id=self.inner.param_id.as_ref().expect("missing parameter id?")).into()
    }
}

impl GetShipmentsIdResponseAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetShipmentsIdResponseAddressBuilder {
        GetShipmentsIdResponseAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<GetShipmentsIdResponseAddress> for GetShipmentsIdResponseAddressBuilder {
    fn into(self) -> GetShipmentsIdResponseAddress {
        self.body
    }
}

/// Builder for [`GetShipmentsIdResponseAddress`](./struct.GetShipmentsIdResponseAddress.html) object.
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseAddressBuilder {
    body: self::GetShipmentsIdResponseAddress,
}

impl GetShipmentsIdResponseAddressBuilder {
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
