
use juniper::graphql_value;


use juniper::FieldResult;

use crate::{common::*};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, Video, PlaylistMeta};
use crate::context::Context;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rating {
	/// Rating given by current user, null is not logged in or not rated
	pub user_rating: Option<i32>,
	/// Sum of ratings
	pub total_rating: i32,
	// Num of users rated this item
	pub total_user: i32,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Rating")]
impl Rating {
	pub fn user_rating(&self) -> Option<i32> {
		self.user_rating
	}
	pub fn total_rating(&self) -> i32 {
		self.total_rating
	}
	pub fn total_user(&self) -> i32 {
		self.total_user
	}
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get user", Context = Context)]
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

pub async fn getRating_impl(context: &Context, para: GetRatingParameters) -> FieldResult<Option<Rating>> { // TODO: user session ID
	let mut result_opt = None;
	
	if para.pid.is_some() {
		result_opt = Some(postJSON!(GetRatingResult, format!("{}/rating/get_playlist_total.do", BACKEND_URL), para, context));
	};
	if para.vid.is_some() {
		result_opt = Some(postJSON!(GetRatingResult, format!("{}/rating/get_video_total.do", BACKEND_URL), para, context));
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
