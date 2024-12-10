use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

impl crate::FireAuth {
    pub async fn sign_up_email(&self, email: Option<&str>, password: Option<&str>, return_secure_token: bool) -> Result<Response, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
            self.api_key,
        );

        let resp = self.create_request(&url)
            .json(&SignUpPayload {
                email,
                password,
                return_secure_token,
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::SignUp(error.message));
        }
        let body = resp.json::<Response>().await?;
        Ok(body)
    }
}

#[derive(Debug, Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignUpPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<&'a str>,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
}
