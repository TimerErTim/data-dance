use crate::objects::job_state::JobStates;
use leptos::ServerFnError;

pub async fn job_status() -> Result<JobStates, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        return Ok(JobStates {
            restore: None,
            backup: None,
        });
    };
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::*;
        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init("/api/jobs/status", &opts)
            .map_err(|val| std::io::Error::other("Failed to create request"))?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|val| std::io::Error::other("Failed to execute request"))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|_| std::io::Error::other("Failed to convert to response"))?;
        let json_future = resp
            .text()
            .map_err(|_| std::io::Error::other("Failed to convert JSON future"))?;

        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(json_future)
            .await
            .map_err(|_| std::io::Error::other("Failed to read JSON"))?;

        return Ok(serde_json::from_str(
            json.as_string()
                .ok_or(std::io::Error::other("Failed to read JSON"))?
                .as_str(),
        )?);
    };
}
