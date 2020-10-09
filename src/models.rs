
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub code: String,
    pub aux: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestResult<T> {
    pub status: String,
    pub data: Option<T>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Meta {
    pub created_at: BsonDateTime,
    pub created_by: ObjectId,
    pub modified_at: Option<BsonDateTime>,
    pub modified_by: Option<ObjectId>
}

#[juniper::object]
#[graphql(description="Video Item")]
impl Meta {

}


// #[derive(Serialize, Deserialize)]
// pub struct VideoItemRest {
//     pub cover_image: String,
//     pub title: String,
//     pub desc: String,
//     pub placeholder: bool,
//     pub rating: i32,
//     pub repost_type: String,
//     pub copies: Vec<ObjectId>,
//     pub series: Vec<ObjectId>,
//     pub site: String,
//     pub thumbnail_url: String,
//     pub unique_id: String,
//     pub upload_time: DateTime,
//     pub url: String,
//     pub user_space_urls: Option<Vec<String>>,
//     pub utags: Vec<String>,
//     pub views: i32
// }

// #[derive(Serialize, Deserialize)]
// pub struct VideoRest {
//     pub clearence: i32,
//     pub item: VideoItemRest,
//     pub meta: MetaRest,
//     pub tag_count: i32,
//     pub tags: Vec<i32>,
//     pub tags_readable: Option<Vec<String>>
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct BsonDateTime
{
    #[serde(rename = "$date")]
    pub ts: i64
}
use chrono::offset::TimeZone;
impl BsonDateTime {
    pub fn to_UtcDateTime(&self) -> DateTime<chrono::Utc> {
        let mut num_secs = self.ts / 1000;
        let mut num_millis = self.ts % 1000;

        // The chrono API only lets us create a DateTime with an i64 number of seconds
        // and a u32 number of nanoseconds. In the case of a negative timestamp, this
        // means that we need to turn the negative fractional part into a positive and
        // shift the number of seconds down. For example:
        //
        //     date       = -4300 ms
        //     num_secs   = date / 1000 = -4300 / 1000 = -4
        //     num_millis = date % 1000 = -4300 % 1000 = -300
        //
        // Since num_millis is less than 0:
        //     num_secs   = num_secs -1 = -4 - 1 = -5
        //     num_millis = num_nanos + 1000 = -300 + 1000 = 700
        //
        // Instead of -4 seconds and -300 milliseconds, we now have -5 seconds and +700
        // milliseconds, which expresses the same timestamp, but in a way we can create
        // a DateTime with.
        if num_millis < 0 {
            num_secs -= 1;
            num_millis += 1000;
        };

        chrono::Utc.timestamp(num_secs, num_millis as u32 * 1_000_000)
            .into()
    }
}
