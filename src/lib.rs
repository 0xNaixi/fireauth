pub mod api;
mod error;

pub use error::Error;
#[derive(Clone, Debug)]
pub struct RequestCustomData {
    pub user_agent: String,
    pub client_data: String,
    pub client_version: String,
    pub firebase_client: String,
    pub firebase_gmpid: String,
}

#[derive(Clone, Debug)]
pub struct FireAuth {
    pub api_key: String, // web api key
    pub request_custom_data: Option<RequestCustomData>,
}

impl FireAuth {
    pub fn new(api_key: String,request_custom_data: Option<RequestCustomData>) -> Self {
        Self{ api_key , request_custom_data }
    }

    fn create_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut request = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json");

        // Add custom headers if request_custom_data is available
        if let Some(custom_data) = &self.request_custom_data {
            request = request
                .header("user-agent", &custom_data.user_agent)
                .header("x-client-data", &custom_data.client_data)
                .header("x-client-version", &custom_data.client_version)
                .header("x-firebase-client", &custom_data.firebase_client)
                .header("x-firebase-gmpid", &custom_data.firebase_gmpid);
        }

        request
    }
}
