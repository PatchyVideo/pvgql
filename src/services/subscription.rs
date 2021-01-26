
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::{common::*, context::Context, models::Subscription};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, VideoItem};

#[derive(Clone, Serialize, Deserialize)]
pub struct ListAllSubscriptionResult {
	pub subs: Vec<Subscription>
}

pub async fn listSubscriptions_impl(context: &Context) -> FieldResult<Vec<Subscription>> {
    let result = postJSON!(ListAllSubscriptionResult, format!("https://thvideo.tv/be/subs/all.do"), EmptyJSON {}, context);
    if result.status == "SUCCEED" {
		Ok(result.data.unwrap().subs)
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="list subscripted videos required parameters", Context = Context)]
pub struct ListSubscriptionVideosParameters {
	/// Offset (start from 0)
	pub offset: Option<i32>,
	/// Num of item in a page
	pub limit: Option<i32>,
	/// List order, one of 'latest', 'oldest', 'video_latest', 'video_oldest'
	pub order: Option<String>,
	// Addtional query constraints
	pub additional_constraint: Option<String>,
	/// If true, no placeholder items will be shown
	pub hide_placeholder: Option<bool>,
	/// User language
    pub lang: Option<String>,
    /// Visible subscriptions, list of obejctid
    pub visible: Option<Vec<String>>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListSubscriptionVideosResult {
	pub videos: Vec<Video>,
	pub total: i32,
    pub objs: Vec<Subscription>
}

#[juniper::graphql_object(Context = Context)]
#[juniper::graphql(description="List subscription videos result")]
impl ListSubscriptionVideosResult {
	pub fn videos(&self) -> &Vec<Video> {
		&self.videos
	}
	pub fn count(&self) -> &i32 {
		&self.total
    }
    /// Return subscriptions used
	pub fn subscriptions(&self) -> &Vec<Subscription> {
		&self.objs
	}
}

pub async fn listSubscriptionVideos_impl(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<ListSubscriptionVideosResult> {
    let result = postJSON!(ListSubscriptionVideosResult, format!("https://thvideo.tv/be/subs/list.do"), para, context);
    if result.status == "SUCCEED" {
		Ok(result.data.unwrap())
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}

pub async fn listSubscriptionVideosRandomized_impl(context: &Context, para: ListSubscriptionVideosParameters) -> FieldResult<ListSubscriptionVideosResult> {
    let result = postJSON!(ListSubscriptionVideosResult, format!("https://thvideo.tv/be/subs/list_randomized.do"), para, context);
    if result.status == "SUCCEED" {
		Ok(result.data.unwrap())
	} else {
		Err(
			juniper::FieldError::new(
				result.status,
				graphql_value!({
					"aa"
				}),
			)
		)
	}
}
