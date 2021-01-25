
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::{common::*, models::{Rating, User}};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, PlaylistMeta};

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get user")]
pub struct GetRatingParameters {
	/// ID of playlist
    pub pid: Option<String>,
	/// ID of video
    pub vid: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetRatingResult {
	pub user_rating: Option<i32>,
	pub total_rating: i32,
	pub total_user: i32,
}

pub async fn getRating_impl(para: GetRatingParameters) -> FieldResult<Option<Rating>> { // TODO: user session ID
    let mut result_opt = None;
    
    if para.pid.is_some() {
        result_opt = Some(postJSON!(GetRatingResult, format!("https://thvideo.tv/be/rating/get_playlist_total.do"), para));
    };
    if para.vid.is_some() {
        result_opt = Some(postJSON!(GetRatingResult, format!("https://thvideo.tv/be/rating/get_video_total.do"), para));
    }
    if result_opt.is_none() {
        return Err(
            juniper::FieldError::new(
                "INCORRECT_REQUEST",
                graphql_value!({
                    "At least one of pid or vid must be set"
                }),
            )
        );
    }

    let result = result_opt.unwrap();
    
    if result.status == "SUCCEED" {
        let r = result.data.unwrap();
		Ok(Some(Rating {
            user_rating: r.user_rating,
            total_rating: r.total_rating,
            total_user: r.total_user
        }))
	} else {
		Ok(None)
	}
}
