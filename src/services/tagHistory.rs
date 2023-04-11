use juniper::{graphql_value};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};

use crate::{common::*, context::Context, services::{pvsubscription::PVSubscription, users, editTags}, models::TagObjectValue};
use juniper::{
	graphql_interface,
	GraphQLObject, FieldResult
};
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, Video, VideoItem};

use super::users::User;

#[derive(Deserialize)]
pub struct RawTagHistoryRestItem {
    #[serde(rename = "tags")]
	pub tag_ids: Vec<i64>,
    #[serde(rename = "add")]
	pub add_tag_ids: Vec<i64>,
    #[serde(rename = "del")]
	pub del_tag_ids: Vec<i64>,
    pub user_id: String,
    pub video_obj: Video,
    pub time: bson::DateTime
}
#[derive(Deserialize)]
pub struct RawTagHistoryRest {
    pub items: Vec<RawTagHistoryRestItem>
}

pub struct RawTagHistoryItem {
	pub add_tag_ids: Vec<i64>,
	pub del_tag_ids: Vec<i64>,
    pub user_id: String,
    pub video_obj: Video,
    pub time: bson::DateTime
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="RawTagHistoryItem")]
impl RawTagHistoryItem {
    pub fn time(&self) -> bson::DateTime {
        self.time
    }
	pub async fn added_tags(&self, context: &Context) -> FieldResult<Vec<TagObjectValue>> {
		editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: self.add_tag_ids.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
		}).await
	}
	pub async fn removed_tags(&self, context: &Context) -> FieldResult<Vec<TagObjectValue>> {
		editTags::getTagObjectsBatch_impl(context, editTags::GetTagObjectsBatchParameters {
			tagid: self.del_tag_ids.iter().filter(|&n| { *n < 2_147_483_647i64 }).map(|&n| n as i32).collect::<Vec<_>>()
		}).await
	}
	pub async fn user(&self, context: &Context) -> FieldResult<User> {
		let u = users::getUser_impl(context, users::GetUserParameters {
			uid: self.user_id.clone()
		}).await?;
		Ok(u)
	}
    pub fn video(&self) -> FieldResult<&Video> {
        Ok(&self.video_obj)
    }
}

pub struct RawTagHistoryResult {
    pub items: Vec<RawTagHistoryItem>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="RawTagHistoryResult")]
impl RawTagHistoryResult {
    pub fn items(&self) -> &Vec<RawTagHistoryItem> {
		&self.items
	}
}

pub async fn getRawTagHistory_impl(context: &Context, offset: i32, limit: i32) -> FieldResult<RawTagHistoryResult> {
	let req = json!({
		"offset": offset,
		"limit": limit
	});
	let result = postJSON!(RawTagHistoryRest, format!("{}/video/raw_tagid_log.do", BACKEND_URL), req, context);
	if result.status == "SUCCEED" {
		let result = result.data.unwrap();
		let items = result
        .items
			.into_iter()
			.map(|o| RawTagHistoryItem {
                add_tag_ids: o.add_tag_ids, 
                del_tag_ids: o.del_tag_ids, 
                user_id: o.user_id, 
                video_obj: o.video_obj, 
                time: o.time 
            })
			.collect::<Vec<_>>();
		Ok(RawTagHistoryResult {
			items: items
		})
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

