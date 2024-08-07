use serde::Deserialize;
use validator::Validate;

const FUNDING_MIN: u16 =  1_000;
const FUNDING_MAX: u16 = 50_000;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[must_use]
pub struct LetsStartForm {
    #[validate(email(message = "Incorrect address"))]
    pub email: String,

    #[validate(range(
        min = "FUNDING_MIN",
        exclusive_max = "FUNDING_MAX",
        message = "The value of the field goes out of range"
    ))]
    pub funding_min: u16,

    #[validate(range(
        min = "FUNDING_MIN",
        max = "FUNDING_MAX",
        message = "The value of the field goes out of range"
    ))]
    pub funding_max: u16,

    #[validate(length(
        min = 2,
        max = 32,
        message = "The length of the field goes out of bounds"
    ))]
    pub name: String,

    #[validate(length(
        min = 64,
        max = 512,
        message = "The length of the field goes out of bounds"
    ))]
    pub project_description: String,

    #[validate(length(
        min = 2,
        max = 32,
        message = "The length of the field goes out of bounds"
    ))]
    pub referral_source: String,

    #[validate(length(
        min = 2,
        max = 32,
        message = "The length of the field goes out of bounds"
    ))]
    pub represent: String,
}
