
/// Namespace for operations that cannot be added to any other modules.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Miscellaneous {}

impl Miscellaneous {
    #[inline]
    pub fn get() -> MiscellaneousGetBuilder {
        MiscellaneousGetBuilder
    }

    #[inline]
    pub fn get_1() -> MiscellaneousGetBuilder1 {
        MiscellaneousGetBuilder1
    }

    #[inline]
    pub fn post_2() -> MiscellaneousPostBuilder2<crate::generics::MissingValues> {
        MiscellaneousPostBuilder2 {
            inner: Default::default(),
            _param_values: core::marker::PhantomData,
        }
    }
}

/// Builder created by [`Miscellaneous::get`](./struct.Miscellaneous.html#method.get) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder {
    type Output = Vec<Vec<crate::test_nested_array_with_object::TestNestedArrayWithObjectItemItem>>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/test/array".into()
    }
}

/// Builder created by [`Miscellaneous::get_1`](./struct.Miscellaneous.html#method.get_1) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder1;


#[async_trait::async_trait]
impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder1 {
    type Output = crate::util::ResponseStream<<<Client as crate::client::ApiClient>::Response as crate::client::Response>::Bytes, <<Client as crate::client::ApiClient>::Response as crate::client::Response>::Error>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        "/test/file".into()
    }

    async fn send(&self, client: &Client) -> Result<crate::client::ResponseWrapper<Self::Output, Self>, crate::client::ApiError<Client::Response>> {
        use crate::client::Response;
        let resp = self.send_raw(client).await?;
        Ok(crate::client::ResponseWrapper::wrap(resp, |r| async {
            Ok(crate::util::ResponseStream(r.stream()))
        }).await.unwrap())
    }
}

/// Builder created by [`Miscellaneous::post_2`](./struct.Miscellaneous.html#method.post_2) method for a `POST` operation associated with `Miscellaneous`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MiscellaneousPostBuilder2<Values> {
    inner: MiscellaneousPostBuilder2Container,
    _param_values: core::marker::PhantomData<Values>,
}

#[derive(Debug, Default, Clone)]
struct MiscellaneousPostBuilder2Container {
    param_values: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<String, crate::util::Pipes>, crate::util::Csv>, crate::util::Ssv>, crate::util::Tsv>>,
    param_x_foobar: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<f64, crate::util::Ssv>, crate::util::Tsv>, crate::util::Csv>, crate::util::Pipes>>,
    param_booya: Option<crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>>,
    param_foo: Option<crate::util::Delimited<crate::util::Delimited<String, crate::util::Csv>, crate::util::Multi>>,
}

impl<Values> MiscellaneousPostBuilder2<Values> {
    #[inline]
    pub fn values(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<String>>>>>) -> MiscellaneousPostBuilder2<crate::generics::ValuesExists> {
        self.inner.param_values = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn x_foobar(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<f64>>>>>) -> Self {
        self.inner.param_x_foobar = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn booya(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<i64>>>) -> Self {
        self.inner.param_booya = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn foo(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<String>>>) -> Self {
        self.inner.param_foo = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousPostBuilder2<crate::generics::ValuesExists> {
    type Output = String;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!("/test/parameter/{values}", values=self.inner.param_values.as_ref().expect("missing parameter values?")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        let mut req = req;
        if let Some(v) = &self.inner.param_x_foobar.as_ref().map(std::string::ToString::to_string) {
            req = req.header("X-foobar", &v);
        }

        Ok(req
        .body_bytes({
            let mut ser = url::form_urlencoded::Serializer::new(String::new());
            if let Some(stuff) = self.inner.param_booya.as_ref() {
                for v in stuff.iter() {
                    ser.append_pair("booya", &v.to_string());
                }
            }
            ser.finish().into_bytes()
        })
        .header(http::header::CONTENT_TYPE.as_str(), "application/x-www-form-urlencoded")
        .query({
            &self.inner.param_foo.as_ref().map(|v| {
                v.iter().map(|v| ("foo", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}
