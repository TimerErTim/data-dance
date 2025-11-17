use crate::context::DataDanceContext;
use poem::{Endpoint, Request, RequestBuilder, Route, http::StatusCode};
use rust_embed::RustEmbed;
use std::sync::Arc;

pub fn ui_router(context: &Arc<DataDanceContext>) -> impl Endpoint + use<> {
    #[cfg(not(debug_assertions))]
    {
        use poem::{
            EndpointExt,
            endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint},
            error::NotFoundError,
        };

        #[derive(RustEmbed, Clone)]
        #[folder = "target/site/"]
        struct StaticUi;

        let files_endpoint =
            EmbeddedFilesEndpoint::<StaticUi>::new().around(|ep, req| async move {
                let req_clone = clone_request_parts(&req);
                let result = ep.call(req).await;
                match result {
                    Ok(response) => Ok(response),
                    Err(err) if err.status() == StatusCode::NOT_FOUND => {
                        EmbeddedFileEndpoint::<StaticUi>::new("404.html")
                            .call(req_clone)
                            .await
                    }
                    Err(err) => Err(err),
                }
            });

        return files_endpoint;
    }

    #[cfg(debug_assertions)]
    {
        return Route::new();
    }
}

fn clone_request_parts(req: &Request) -> Request {
    let mut builder = Request::builder()
        .method(req.method().clone())
        .uri(req.uri().clone())
        .version(req.version());

    for (key, value) in req.headers().iter() {
        builder = builder.header(key.clone(), value.clone());
    }

    builder.finish()
}
