#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Discovery {
    pub active: bool,
    pub port: u16,
    #[serde(rename(deserialize = "instanceID"))]
    pub instance_id: String,
    pub window_title: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct APIStateResponse {
    pub active: bool,
    pub v_tube_studio_version: String,
    pub current_session_authenticated: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationToken {
    pub authentication_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub authenticated: bool,
    pub reason: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct APIError {
    #[serde(rename(deserialize = "errorID"))]
    pub error_id: u16,
    pub message: String,
}
