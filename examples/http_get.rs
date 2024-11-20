use std::error::Error;
use wstd::http::{Client, HeaderValue, Method, Request};
use wstd::io::AsyncRead;

#[wstd::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut request = Request::new(Method::GET, "https://postman-echo.com/get".parse()?);
    request
        .headers_mut()
        .insert("my-header", HeaderValue::from_str("my-value")?);

    let mut response = Client::new().send(request).await?;

    let content_type = response
        .headers()
        .get("Content-Type")
        .ok_or_else(|| "response expected to have Content-Type header")?;
    assert_eq!(content_type, "application/json; charset=utf-8");

    let mut body_buf = Vec::new();
    let _body_len = response.body().read_to_end(&mut body_buf).await?;

    let val: serde_json::Value = serde_json::from_slice(&body_buf)?;
    let body_url = val
        .get("url")
        .ok_or_else(|| "body json has url")?
        .as_str()
        .ok_or_else(|| "body json url is str")?;
    assert!(
        body_url.contains("postman-echo.com/get"),
        "expected body url to contain the authority and path, got: {body_url}"
    );

    assert_eq!(
        val.get("headers")
            .ok_or_else(|| "body json has headers")?
            .get("my-header")
            .ok_or_else(|| "headers contains my-header")?
            .as_str()
            .ok_or_else(|| "my-header is a str")?,
        "my-value"
    );

    Ok(())
}
