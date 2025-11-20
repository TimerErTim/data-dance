//pub mod jobs;

use crate::{context::DataDanceContext, objects::job_state::JobStates};
use poem::Endpoint;
use poem::web::Data;
use poem_openapi::payload::Response;
use poem_openapi::{OpenApi, OpenApiService, payload::Json};
use std::sync::Arc;
use poem::Result;

pub struct DataDanceApi;

#[OpenApi]
impl DataDanceApi {
    #[oai(path = "/jobs", method = "get")]
    async fn get_jobs(&self, context: Result<Data<&Arc<DataDanceContext>>>) -> Result<Json<JobStates>> {
        match context {
            Ok(ctx) => {
                println!("Getting jobs...");
                Ok(Json(ctx.executor.active_jobs()))
            }
            Err(err) => {
                println!("Error: {:?}", err);
                Err(err)
            }
        }
    }
}

pub fn api_service() -> OpenApiService<impl OpenApi + use<>, ()> {
    OpenApiService::new(
        DataDanceApi, "DataDance API", 
        env!("CARGO_PKG_VERSION")
    )

    // Route::new()
    //     .nest("/jobs", jobs_router(context))
    //     .with_state(Arc::clone(context))
    //     .fallback(async || StatusCode::NOT_FOUND)
}
