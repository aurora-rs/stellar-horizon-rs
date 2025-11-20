//! Horizon client traits and types.
//!
//! All request paths are joined using relative segments (no leading '/') via [`Url::join`](https://docs.rs/url/latest/url/struct.Url.html#method.join).
//! If your base host includes a path (e.g., https://example.com/horizon/api), ensure it ends
//! with a trailing slash (https://example.com/horizon/api/) so joins append to that path.
use crate::error::{Error, Result};
use crate::headers::HeaderMap;
use crate::horizon_error::HorizonError;
use crate::request::{Request, StreamRequest};
use bytes::Bytes;
use futures::future::{BoxFuture, Future};
use futures::stream::TryStreamExt;
use futures::Stream;
use http_body_util::{BodyExt, Full};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::{connect::HttpConnector, Client, ResponseFuture};
use hyper_util::rt::TokioExecutor;
use std::convert::TryInto;
use std::marker::Unpin;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use url::Url;

/// Horizon Client trait. Send HTTP and stream requests to Horizon.
pub trait HorizonClient {
    /// Send a request `R` to horizon, returns the corresponding response.
    fn request<'a, R: Request + 'a>(
        &'a self,
        req: R,
    ) -> BoxFuture<'a, Result<(HeaderMap, R::Response)>>;
    /// Create a stream request.
    fn stream<'a, R: StreamRequest + 'static>(
        &'a self,
        req: R,
    ) -> Result<Box<dyn Stream<Item = Result<R::Resource>> + 'static + Send + Unpin>>;
}

type HttpClient = Client<TimeoutConnector<HttpsConnector<HttpConnector>>, Full<Bytes>>;

/// Type that implements `HorizonClient` using `hyper` for http.
pub struct HorizonHttpClient {
    inner: Arc<HorizonHttpClientInner>,
}

struct HorizonHttpClientInner {
    inner: HttpClient,
    host: Url,
    client_name: String,
    client_version: String,
    extra_headers: Option<hyper::HeaderMap>,
}

type BoxDecoder = Box<dyn Unpin + Send + Stream<Item = http_types::Result<async_sse::Event>>>;

/// A `Stream` that represents a horizon stream connection.
#[must_use = "Streams are lazy and do nothing unless polled"]
pub struct HorizonHttpStream<R>
where
    R: StreamRequest,
{
    client: Arc<HorizonHttpClientInner>,
    last_id: Option<String>,
    request: R,
    response: Option<ResponseFuture>,
    decoder: Option<BoxDecoder>,
}

impl HorizonHttpClientInner {
    pub fn new(host: Url) -> Result<HorizonHttpClientInner> {
        let https = HttpsConnector::new();
        let mut timeout_connector = TimeoutConnector::new(https);
        let duration = Duration::from_secs(60);

        timeout_connector.set_connect_timeout(Some(duration));
        timeout_connector.set_read_timeout(Some(duration));
        timeout_connector.set_write_timeout(Some(duration));
        let inner =
            Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(timeout_connector);
        let client_name = "aurora-rs/stellar-horizon-rs".to_string();
        let client_version = crate::VERSION.to_string();
        Ok(HorizonHttpClientInner {
            inner,
            host,
            client_name,
            client_version,
            extra_headers: None,
        })
    }

    pub fn new_with_client(
        host: Url,
        client: HttpClient,
        extra_headers: hyper::HeaderMap,
    ) -> Result<HorizonHttpClientInner> {
        let client_name = "aurora-rs/stellar-horizon-rs".to_string();
        let client_version = crate::VERSION.to_string();
        Ok(HorizonHttpClientInner {
            inner: client,
            host,
            client_name,
            client_version,
            extra_headers: Some(extra_headers),
        })
    }

    pub fn with_extra_headers(
        host: Url,
        extra_headers: hyper::HeaderMap,
    ) -> Result<HorizonHttpClientInner> {
        let mut client = HorizonHttpClientInner::new(host)?;
        client.extra_headers = Some(extra_headers);
        Ok(client)
    }

    pub fn request_builder(&self, uri: Url) -> http::request::Builder {
        let mut builder = hyper::Request::builder()
            .uri(uri.to_string())
            .header("X-Client-Name", self.client_name.to_string())
            .header("X-Client-Version", self.client_version.to_string());
        if let Some(extra_headers) = &self.extra_headers {
            builder.headers_mut().unwrap().extend(
                extra_headers
                    .clone()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );
        }
        builder
    }

    fn get(&self, uri: Url) -> http::request::Builder {
        self.request_builder(uri).method(hyper::Method::GET)
    }

    fn raw_request(&self, req: hyper::Request<Full<Bytes>>) -> ResponseFuture {
        self.inner.request(req)
    }
}

impl HorizonHttpClient {
    /// Creates a new horizon client with the specified host url str.
    pub fn new_from_str(host: &str) -> Result<HorizonHttpClient> {
        let host: Url = host.parse().map_err(|_| Error::InvalidHost)?;
        HorizonHttpClient::new(host)
    }

    /// Creates a new horizon client with the specified host url str and extra headers
    pub fn with_extra_headers(
        host: &str,
        extra_headers: hyper::HeaderMap,
    ) -> Result<HorizonHttpClient> {
        let host: Url = host.parse().map_err(|_| Error::InvalidHost)?;
        let inner = HorizonHttpClientInner::with_extra_headers(host, extra_headers)?;
        Ok(HorizonHttpClient {
            inner: Arc::new(inner),
        })
    }

    /// Creates a new horizon client with a custom HTTP client and extra headers
    pub fn new_with_client(
        client: HttpClient,
        host: &str,
        extra_headers: hyper::HeaderMap,
    ) -> Result<HorizonHttpClient> {
        let host: Url = host.parse().map_err(|_| Error::InvalidHost)?;
        let inner = HorizonHttpClientInner::new_with_client(host, client, extra_headers)?;
        Ok(HorizonHttpClient {
            inner: Arc::new(inner),
        })
    }

    /// Creates a new horizon client with the specified host url.
    pub fn new<U>(host: U) -> Result<HorizonHttpClient>
    where
        U: TryInto<Url>,
    {
        let host = host.try_into().map_err(|_| Error::InvalidHost)?;
        let inner = Arc::new(HorizonHttpClientInner::new(host)?);
        Ok(HorizonHttpClient { inner })
    }

    /// Returns a request builder with default headers.
    fn request_builder(&self, uri: Url) -> http::request::Builder {
        self.inner.request_builder(uri)
    }

    /// Performs a request.
    fn raw_request(&self, req: hyper::Request<Full<Bytes>>) -> ResponseFuture {
        self.inner.raw_request(req)
    }
}

impl HorizonClient for HorizonHttpClient {
    fn request<'a, R: Request + 'a>(
        &'a self,
        req: R,
    ) -> BoxFuture<'a, Result<(HeaderMap, R::Response)>> {
        Box::pin(execute_request(self, req))
    }

    fn stream<'a, R: StreamRequest + 'static>(
        &'a self,
        request: R,
    ) -> Result<Box<dyn Stream<Item = Result<R::Resource>> + 'static + Send + Unpin>> {
        Ok(Box::new(HorizonHttpStream {
            client: self.inner.clone(),
            request,
            last_id: None,
            response: None,
            decoder: None,
        }))
    }
}

async fn execute_request<R: Request>(
    client: &HorizonHttpClient,
    req: R,
) -> Result<(HeaderMap, R::Response)> {
    let uri = req.uri(&client.inner.host)?;
    let request_builder = client.request_builder(uri);

    let request = if let Some(body) = req.post_body()? {
        request_builder
            .method(hyper::Method::POST)
            .header(
                hyper::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(Full::new(Bytes::from(body)))?
    } else {
        request_builder
            .method(hyper::Method::GET)
            .body(Full::new(Bytes::new()))?
    };

    let response = match client.raw_request(request).await {
        Ok(r) => r,
        Err(_e) => return Err(Error::HorizonServerError),
    };
    let status = response.status();

    if status.is_success() {
        let headers = response.headers().clone();
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let result: R::Response = serde_json::from_slice(&bytes)?;
        Ok((headers, result))
    } else if status.is_client_error() {
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let result: HorizonError = serde_json::from_slice(&bytes)?;
        Err(Error::HorizonRequestError(result))
    } else {
        Err(Error::HorizonServerError)
    }
}

impl<R> Stream for HorizonHttpStream<R>
where
    R: StreamRequest,
{
    type Item = Result<R::Resource>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            if self.response.is_none() && self.decoder.is_none() {
                let uri = self.request.uri(&self.client.host)?;
                let mut request_builder =
                    self.client.get(uri).header("Accept", "text/event-stream");
                if let Some(last_id) = &self.last_id {
                    request_builder = request_builder.header("Last-Event-Id", last_id.clone());
                }

                let request = request_builder.body(Full::new(Bytes::new()))?;
                let response = self.client.raw_request(request);
                self.response = Some(response);
            }

            if let Some(mut resp) = self.response.take() {
                match Pin::new(&mut resp).poll(cx) {
                    Poll::Pending => {
                        self.response = Some(resp);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(_e)) => {
                        // Map legacy client error to a generic horizon server error.
                        // The legacy error type from hyper-util doesn't implement Into<Error>,
                        // and for our purposes a server-level failure is sufficient.
                        return Poll::Ready(Some(Err(Error::HorizonServerError)));
                    }
                    Poll::Ready(Ok(resp)) => {
                        // TODO(fra): handle non success statuses
                        assert!(resp.status().is_success());
                        let body_stream = resp
                            .into_body()
                            .into_data_stream()
                            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
                            .into_async_read();

                        let decoder = Box::new(async_sse::decode(body_stream));
                        self.decoder = Some(decoder);
                    }
                }
            }

            if let Some(mut decoder) = self.decoder.take() {
                match Pin::new(&mut decoder).poll_next(cx) {
                    Poll::Pending => {
                        self.decoder = Some(decoder);
                        return Poll::Pending;
                    }
                    Poll::Ready(None) => {}
                    Poll::Ready(Some(Err(_))) => {
                        let err = Error::SSEDecoderError;
                        return Poll::Ready(Some(Err(err)));
                    }
                    Poll::Ready(Some(Ok(message))) => {
                        self.decoder = Some(decoder);
                        match message {
                            async_sse::Event::Message(msg) => {
                                if let Some(last_id) = msg.id() {
                                    self.last_id = Some(last_id.to_string());
                                }
                                if msg.name() == "message" {
                                    let result: R::Resource =
                                        serde_json::from_slice(&msg.into_bytes())?;
                                    return Poll::Ready(Some(Ok(result)));
                                }
                            }
                            async_sse::Event::Retry(duration) => {
                                println!("got duration {:?}", duration);
                            }
                        }
                    }
                }
            }
        }
    }
}
