#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestNestedArrayWithObjectItemItem {
    pub bar: Option<crate::test_nested_array_with_object::TestNestedArrayWithObjectItemItemBar>,
    pub foo: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestNestedArrayWithObjectItemItemBar {
    pub baz: Option<i64>,
}

impl TestNestedArrayWithObjectItemItem {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TestNestedArrayWithObjectItemItemBuilder {
        TestNestedArrayWithObjectItemItemBuilder {
            body: Default::default(),
        }
    }
}

impl Into<TestNestedArrayWithObjectItemItem> for TestNestedArrayWithObjectItemItemBuilder {
    fn into(self) -> TestNestedArrayWithObjectItemItem {
        self.body
    }
}

/// Builder for [`TestNestedArrayWithObjectItemItem`](./struct.TestNestedArrayWithObjectItemItem.html) object.
#[derive(Debug, Clone)]
pub struct TestNestedArrayWithObjectItemItemBuilder {
    body: self::TestNestedArrayWithObjectItemItem,
}

impl TestNestedArrayWithObjectItemItemBuilder {
    #[inline]
    pub fn bar(mut self, value: crate::test_nested_array_with_object::TestNestedArrayWithObjectItemItemBar) -> Self {
        self.body.bar = Some(value.into());
        self
    }

    #[inline]
    pub fn foo(mut self, value: impl Into<String>) -> Self {
        self.body.foo = Some(value.into());
        self
    }
}

impl TestNestedArrayWithObjectItemItemBar {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> TestNestedArrayWithObjectItemItemBarBuilder {
        TestNestedArrayWithObjectItemItemBarBuilder {
            body: Default::default(),
        }
    }
}

impl Into<TestNestedArrayWithObjectItemItemBar> for TestNestedArrayWithObjectItemItemBarBuilder {
    fn into(self) -> TestNestedArrayWithObjectItemItemBar {
        self.body
    }
}

/// Builder for [`TestNestedArrayWithObjectItemItemBar`](./struct.TestNestedArrayWithObjectItemItemBar.html) object.
#[derive(Debug, Clone)]
pub struct TestNestedArrayWithObjectItemItemBarBuilder {
    body: self::TestNestedArrayWithObjectItemItemBar,
}

impl TestNestedArrayWithObjectItemItemBarBuilder {
    #[inline]
    pub fn baz(mut self, value: impl Into<i64>) -> Self {
        self.body.baz = Some(value.into());
        self
    }
}
