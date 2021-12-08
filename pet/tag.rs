#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: Option<String>,
}

impl Tag {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TagBuilder {
        TagBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Tag> for TagBuilder {
    fn into(self) -> Tag {
        self.body
    }
}

/// Builder for [`Tag`](./struct.Tag.html) object.
#[derive(Debug, Clone)]
pub struct TagBuilder {
    body: self::Tag,
}

impl TagBuilder {
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
