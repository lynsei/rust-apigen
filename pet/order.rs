#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Order {
    pub address: Option<crate::order::OrderAddress>,
    pub id: Option<i64>,
    pub list: Option<Vec<crate::order::OrderListItem>>,
    pub status: Option<crate::order::OrderStatus>,
    #[serde(rename = "test-string-enum")]
    pub test_string_enum: Option<crate::order::OrderTestStringEnum>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderListItem {
    #[serde(rename = "petId")]
    pub pet_id: Option<i64>,
    pub quantity: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OrderStatus {
    #[serde(rename = "paymentPending")]
    PaymentPending,
    #[serde(rename = "orderPlaced")]
    OrderPlaced,
    #[serde(rename = "shipped")]
    Shipped,
    #[serde(rename = "fulfilled")]
    Fulfilled,
}
impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::PaymentPending
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OrderTestStringEnum {
    #[serde(rename = "booya")]
    Booya,
    #[serde(rename = "72.9")]
    Number_72_9,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "-53")]
    Number__53,
}
impl Default for OrderTestStringEnum {
    fn default() -> Self {
        OrderTestStringEnum::Booya
    }
}

impl Order {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderBuilder {
        OrderBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Order> for OrderBuilder {
    fn into(self) -> Order {
        self.body
    }
}

/// Builder for [`Order`](./struct.Order.html) object.
#[derive(Debug, Clone)]
pub struct OrderBuilder {
    body: self::Order,
}

impl OrderBuilder {
    #[inline]
    pub fn address(mut self, value: crate::order::OrderAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn list(mut self, value: impl Iterator<Item = crate::order::OrderListItem>) -> Self {
        self.body.list = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn status(mut self, value: crate::order::OrderStatus) -> Self {
        self.body.status = Some(value.into());
        self
    }

    #[inline]
    pub fn test_string_enum(mut self, value: crate::order::OrderTestStringEnum) -> Self {
        self.body.test_string_enum = Some(value.into());
        self
    }
}

impl OrderAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderAddressBuilder {
        OrderAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<OrderAddress> for OrderAddressBuilder {
    fn into(self) -> OrderAddress {
        self.body
    }
}

/// Builder for [`OrderAddress`](./struct.OrderAddress.html) object.
#[derive(Debug, Clone)]
pub struct OrderAddressBuilder {
    body: self::OrderAddress,
}

impl OrderAddressBuilder {
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

impl OrderListItem {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderListItemBuilder {
        OrderListItemBuilder {
            body: Default::default(),
        }
    }
}

impl Into<OrderListItem> for OrderListItemBuilder {
    fn into(self) -> OrderListItem {
        self.body
    }
}

/// Builder for [`OrderListItem`](./struct.OrderListItem.html) object.
#[derive(Debug, Clone)]
pub struct OrderListItemBuilder {
    body: self::OrderListItem,
}

impl OrderListItemBuilder {
    #[inline]
    pub fn pet_id(mut self, value: impl Into<i64>) -> Self {
        self.body.pet_id = Some(value.into());
        self
    }

    #[inline]
    pub fn quantity(mut self, value: impl Into<i64>) -> Self {
        self.body.quantity = Some(value.into());
        self
    }
}


