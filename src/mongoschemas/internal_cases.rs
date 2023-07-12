use super::core::Model;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalCases {
    #[serde(rename = "_id")]
    /// The unique ID of the document in the database
    pub id: ObjectId,

    /// The user ID of the user who the case was created for
    pub user: String,

    /// The guild ID of the guild where the case was created
    pub guild: String,

    /// The reason for the case
    pub reason: String,

    /// The action taken for the case
    pub action: String,

    /// The moderator ID of the moderator who created the case
    pub moderator: String,

    /// The case ID
    pub case: u64,

    #[serde(deserialize_with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime::deserialize")]
    pub created: chrono::DateTime<chrono::Utc>,

    #[serde(skip)]
    /// The time of the case
    pub time: String,

    /// The duration of the case, e.g. 3 Months
    pub duration: String,

    /// Level of the infraction
    pub level: String,

    #[serde(skip)]
    /// Start of the infraction
    pub start: String,

    #[serde(skip)]
    /// End of the infraction
    pub end: String,
}

impl Model for InternalCases {
    fn collection_name() -> &'static str {
        "internalcases"
    }
}
