#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RecursiveObject<Any> {
    pub any: Option<Any>,
    pub children: Option<Vec<crate::recursive_object::RecursiveObject<Any>>>,
    pub id: Option<String>,
    pub next: Option<Box<crate::recursive_object::RecursiveObject<Any>>>,
}

impl<Any: Default> RecursiveObject<Any> {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> RecursiveObjectBuilder<Any> {
        RecursiveObjectBuilder {
            body: Default::default(),
        }
    }
}

impl<Any> Into<RecursiveObject<Any>> for RecursiveObjectBuilder<Any> {
    fn into(self) -> RecursiveObject<Any> {
        self.body
    }
}

/// Builder for [`RecursiveObject`](./struct.RecursiveObject.html) object.
#[derive(Debug, Clone)]
pub struct RecursiveObjectBuilder<Any> {
    body: self::RecursiveObject<Any>,
}

impl<Any> RecursiveObjectBuilder<Any> {
    #[inline]
    pub fn any(mut self, value: impl Into<Any>) -> Self {
        self.body.any = Some(value.into());
        self
    }

    #[inline]
    pub fn children(mut self, value: impl Iterator<Item = crate::recursive_object::RecursiveObject<Any>>) -> Self {
        self.body.children = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn next(mut self, value: crate::recursive_object::RecursiveObject<Any>) -> Self {
        self.body.next = Some(value.into());
        self
    }
}
