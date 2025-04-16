use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Record {
    pub id: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    pub fields: Fields,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Fields {
    pub eligibility: String,
    #[serde(rename = "OTP")]
    pub otp: String,
    #[serde(rename = "Slack ID", default)]
    pub slack_id: String,
    #[serde(rename = "SlackUsername", default)]
    pub slack_username: String,
    #[serde(rename = "GitHub handle", default)]
    pub github_handle: String,
}
