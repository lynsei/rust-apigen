
#[macro_use]
extern crate serde;

pub mod category {
    include!("./category.rs");
}

pub mod get_shipments_id_response {
    include!("./get_shipments_id_response.rs");
}

pub mod miscellaneous {
    include!("./miscellaneous.rs");
}

pub mod order {
    include!("./order.rs");
}

pub mod pet {
    include!("./pet.rs");
}

pub mod post_shipments_body {
    include!("./post_shipments_body.rs");
}

pub mod recursive_container {
    include!("./recursive_container.rs");
}

pub mod recursive_object {
    include!("./recursive_object.rs");
}

pub mod status {
    include!("./status.rs");
}

pub mod tag {
    include!("./tag.rs");
}

pub mod test_enum {
    include!("./test_enum.rs");
}

pub mod test_nested_array_with_object {
    include!("./test_nested_array_with_object.rs");
}

pub mod client {
    use futures::Stream;
    use parking_lot::Mutex;

    use std::borrow::Cow;
    use std::fmt::Debug;
    use std::path::Path;

    /// Common API errors.
    #[derive(Debug, thiserror::Error)]
    pub enum ApiError<R: Debug + Send + 'static> {
        #[error("API request failed for path: {} (code: {})", _0, _1)]
        Failure(String, http::status::StatusCode, Mutex<R>),
        #[error("Unsupported media type in response: {}", _0)]
        UnsupportedMediaType(String, Mutex<R>),
        #[error("An error has occurred while performing the API request: {}", _0)]
        Reqwest(reqwest::Error),
        #[error("I/O error: {}", _0)]
        Io(std::io::Error),
        #[error("Error en/decoding \"application/json\" data: {}", _0)]
        ApplicationJson(serde_json::Error),
        #[error("Error en/decoding \"application/yaml\" data: {}", _0)]
        ApplicationYaml(serde_yaml::Error),
    }

    /// Form object for building multipart request body.
    pub trait Form: Sized {
        /// Creates a new builder.
        fn new() -> Self;

        /// Adds the given key and value as text.
        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>;

        /// Adds the file from the given path for streaming.
        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>;
    }

    /// HTTP Request.
    pub trait Request {
        type Form: Form;

        /// Sets the header with the given key and value.
        fn header(self, name: &'static str, value: &str) -> Self;

        /// Sets body using the given vector of bytes.
        ///
        /// **NOTE:** Appropriate `Content-Type` header must be set
        /// after calling this method.
        fn body_bytes(self, body: Vec<u8>) -> Self;

        /// Sets JSON body based on the given value.
        fn json<T: serde::Serialize>(self, value: &T) -> Self;

        /// Sets `multipart/form-data` body using the given form.
        fn multipart_form_data(self, form: Self::Form) -> Self;

        /// Sets/adds query parameters based on the given value.
        ///
        /// **NOTE:** This method must be called only once. It's unspecified
        /// as to whether this appends/replaces query parameters.
        fn query<T: serde::Serialize>(self, params: &T) -> Self;
    }

    impl Form for reqwest::multipart::Form {
        fn new() -> Self {
            reqwest::multipart::Form::new()
        }

        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>
        {
            reqwest::multipart::Form::text(self, key, value)
        }

        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>
        {
            use reqwest::multipart::{Form, Part};
            use tokio_util::codec::{BytesCodec, FramedRead};

            let fd = std::fs::File::open(path)?;
            let reader = tokio::fs::File::from_std(fd);
            let bytes_stream = FramedRead::new(reader, BytesCodec::new());
            let part = Part::stream(reqwest::Body::wrap_stream(bytes_stream));
            Ok(Form::part(self, key, part))
        }
    }

    impl Request for reqwest::RequestBuilder {
        type Form = reqwest::multipart::Form;

        fn header(self, name: &'static str, value: &str) -> Self {
            reqwest::RequestBuilder::header(self, name, value)
        }

        fn multipart_form_data(self, form: Self::Form) -> Self {
            self.multipart(form)
        }

        fn body_bytes(self, body: Vec<u8>) -> Self {
            self.body(body)
        }

        fn json<T: serde::Serialize>(self, value: &T) -> Self {
            <reqwest::RequestBuilder>::json(self, value)
        }

        fn query<T: serde::Serialize>(self, params: &T) -> Self {
            reqwest::RequestBuilder::query(self, params)
        }
    }

    /// HTTP Response.
    #[async_trait::async_trait]
    pub trait Response: Debug + Send + Sized {
        type Bytes: AsRef<[u8]>;
        type Error;

        /// Gets the value for the given header name, if any.
        fn header(&self, name: &'static str) -> Option<&str>;

        /// Takes all headers from the response.
        fn take_headers(&mut self) -> http::header::HeaderMap;

        /// Status code for this response.
        fn status(&self) -> http::status::StatusCode;

        /// Media type for this response body (if any).
        fn media_type(&self) -> Option<mime::MediaType>;

        /// Response body as a stream.
        fn stream(self) -> Box<dyn Stream<Item=Result<Self::Bytes, Self::Error>> + Unpin>;

        /// Vector of bytes from the response body.
        async fn body_bytes(self) -> Result<Self::Bytes, ApiError<Self>>;
    }

    #[async_trait::async_trait]
    impl Response for reqwest::Response {
        type Bytes = bytes::Bytes;
        type Error = reqwest::Error;

        fn header(&self, name: &'static str) -> Option<&str> {
            self.headers().get(name).and_then(|v| v.to_str().ok())
        }

        fn take_headers(&mut self) -> http::header::HeaderMap {
            std::mem::replace(self.headers_mut(), http::header::HeaderMap::new())
        }

        fn status(&self) -> http::status::StatusCode {
            reqwest::Response::status(self)
        }

        fn media_type(&self) -> Option<mime::MediaType> {
            self.header(http::header::CONTENT_TYPE.as_str())
                .and_then(|v| v.parse().ok())
        }

        fn stream(self) -> Box<dyn Stream<Item=Result<Self::Bytes, Self::Error>> + Unpin> {
            Box::new(self.bytes_stream()) as Box<_>
        }

        async fn body_bytes(self) -> Result<Self::Bytes, ApiError<Self>> {
            Ok(self.bytes().await.map_err(ApiError::Reqwest)?)
        }
    }

    /// Represents an API client.
    #[async_trait::async_trait]
    pub trait ApiClient {
        type Request: Request + Send;
        type Response: Response;

        /// Consumes a method and a relative path and produces a request builder for a single API call.
        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request;

        /// Performs the HTTP request using the given `Request` object
        /// and returns a `Response` future.
        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>>;
    }

    #[async_trait::async_trait]
    impl ApiClient for reqwest::Client {
        type Request = reqwest::RequestBuilder;
        type Response = reqwest::Response;

        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
            let mut u = String::from("https://pets.com:8888/api");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }

        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
            let req = req.build().map_err(ApiError::Reqwest)?;
            let resp = self.execute(req).await.map_err(ApiError::Reqwest)?;
            Ok(resp)
        }
    }

    /// A trait for indicating that the implementor can send an API call.
    #[async_trait::async_trait]
    pub trait Sendable<Client>
    where
        Client: ApiClient + Sync + 'static,
        Self: Sized
    {
        /// The output object from this API request.
        type Output: serde::de::DeserializeOwned;

        /// HTTP method used by this call.
        const METHOD: http::Method;

        /// Relative URL for this API call formatted appropriately with parameter values.
        ///
        /// **NOTE:** This URL **must** begin with `/`.
        fn rel_path(&self) -> std::borrow::Cow<'static, str>;

        /// Modifier for this object. Builders override this method if they
        /// wish to add query parameters, set body, etc.
        fn modify(&self, req: Client::Request) -> Result<Client::Request, ApiError<Client::Response>> {
            Ok(req)
        }

        /// Sends the request and returns a future for the response object.
        async fn send(&self, client: &Client) -> Result<ResponseWrapper<Self::Output, Self>, ApiError<Client::Response>> {
            let resp = self.send_raw(client).await?;
            let media = resp.media_type();
            if let Some(ty) = media {
                if media_types::M_0.matches(&ty) {
                    return ResponseWrapper::wrap(resp, |r| async {
                        let bytes = r.body_bytes().await?;
                        serde_json::from_reader(bytes.as_ref()).map_err(ApiError::from)
                    }).await
                }
                else if media_types::M_1.matches(&ty) {
                    return ResponseWrapper::wrap(resp, |r| async {
                        let bytes = r.body_bytes().await?;
                        serde_yaml::from_reader(bytes.as_ref()).map_err(ApiError::from)
                    }).await
                }
            }

            let ty = resp.header(http::header::CONTENT_TYPE.as_str())
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
                .unwrap_or_default();
            Err(ApiError::UnsupportedMediaType(ty, Mutex::new(resp)))
        }

        /// Convenience method for returning a raw response after sending a request.
        async fn send_raw(&self, client: &Client) -> Result<Client::Response, ApiError<Client::Response>> {
            let rel_path = self.rel_path();
            let req = self.modify(client.request_builder(Self::METHOD, &rel_path))?;
            let resp = client.make_request(req).await?;
            if resp.status().is_success() {
                Ok(resp)
            } else {
                Err(ApiError::Failure(rel_path.into_owned(), resp.status(), Mutex::new(resp)))
            }
        }
    }

    /// Wrapper containing response-related information.
    pub struct ResponseWrapper<T, B> {
        /// Response object
        pub object: T,
        /// Response headers
        pub headers: http::HeaderMap,
        /// Response status code
        pub status: http::status::StatusCode,
        _builder: core::marker::PhantomData<B>,
    }

    impl<T, B> ResponseWrapper<T, B> {
        pub(crate) async fn wrap<F, R>(mut resp: R, f: impl FnOnce(R) -> F) -> Result<Self, ApiError<R>>
            where F: std::future::Future<Output=Result<T, ApiError<R>>>,
                  R: Response + 'static
        {
            let status = resp.status();
            let headers = resp.take_headers();
            Ok(ResponseWrapper {
                object: f(resp).await?,
                headers,
                status,
                _builder: core::marker::PhantomData,
            })
        }
    }

    impl<'de, T, B> serde::de::Deserialize<'de> for ResponseWrapper<T, B> {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>
        {
            unimplemented!("ResponseWrapper is not supposed to be deserialized.");
        }
    }

    impl<T, B> std::ops::Deref for ResponseWrapper<T, B> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.object
        }
    }

    impl<T, B> std::ops::DerefMut for ResponseWrapper<T, B> {
        fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
            &mut self.object
        }
    }

    pub mod media_types {
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref M_0: mime::MediaRange =
                mime::MediaRange::parse("application/json").expect("cannot parse \"application/json\" as media range");
            pub static ref M_1: mime::MediaRange =
                mime::MediaRange::parse("application/yaml").expect("cannot parse \"application/yaml\" as media range");
        }
    }

    impl<R: Response + 'static> From<std::io::Error> for ApiError<R> {
        fn from(e: std::io::Error) -> Self {
            ApiError::Io(e)
        }
    }

    impl<R: Response + 'static> From<serde_json::Error> for ApiError<R> {
        fn from(e: serde_json::Error) -> Self {
            ApiError::ApplicationJson(e)
        }
    }

    impl<R: Response + 'static> From<serde_yaml::Error> for ApiError<R> {
        fn from(e: serde_yaml::Error) -> Self {
            ApiError::ApplicationYaml(e)
        }
    }
}

pub mod generics {
    include!("./generics.rs");
}

pub mod util {
    include!("./util.rs");
}
