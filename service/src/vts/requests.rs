    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthToken<'a> {
        pub plugin_name: &'a str,
        pub plugin_developer: &'a str,
        pub plugin_icon: Option<&'a str>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Auth<'a> {
        pub plugin_name: &'a str,
        pub plugin_developer: &'a str,
        pub authentication_token: &'a str,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ParameterCreation {
        pub parameter_name: String,
        pub explanation: String,
        pub min: f64,
        pub max: f64,
        pub default_value: f64,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct TrackingParam<'a> {
        pub id: &'a str,
        pub weight: Option<f64>,
        pub value: f64, // -1000000 | 1000000
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct InjectParams<'a> {
        pub face_found: bool,
        pub mode: &'a str,
        pub parameter_values: Vec<TrackingParam<'a>>,
    }