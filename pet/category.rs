#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl Category {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Category> for CategoryBuilder {
    fn into(self) -> Category {
        self.body
    }
}

/// Builder for [`Category`](./struct.Category.html) object.
#[derive(Debug, Clone)]
pub struct CategoryBuilder {
    body: self::Category,
}

impl CategoryBuilder {
    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}
