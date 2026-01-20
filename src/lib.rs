use worker::*;

#[event(fetch)]
async fn fetch(
    _req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<HttpResponse> {
    let body = Body::from_stream(futures_util::stream::once(async {
        Ok::<_, worker::Error>("Hello World, are u there??".as_bytes().to_vec())
    }))?;
    
    http::Response::builder()
        .status(200)
        .body(body)
        .map_err(|e| Error::from(e.to_string()))
}