use juniper::{graphql_value};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};

use crate::{common::*, context::Context, services::{pvsubscription::PVSubscription, users}};
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

pub struct LeaderboardResultItem {
	pub user_id: String,
	pub count: i32
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="LeaderboardResultItem")]
impl LeaderboardResultItem {
	pub fn count(&self) -> i32 {
		self.count
	}
	pub async fn user(&self, context: &Context) -> FieldResult<User> {
		let u = users::getUser_impl(context, users::GetUserParameters {
			uid: self.user_id.clone()
		}).await?;
		Ok(u)
	}
}

pub struct LeaderboardResult {
	pub items: Vec<LeaderboardResultItem>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="LeaderboardResult")]
impl LeaderboardResult {
	pub fn items(&self) -> &Vec<LeaderboardResultItem> {
		&self.items
	}
}

#[derive(Deserialize)]
pub struct LeaderboardResultRestItem {
	pub _id: ObjectId,
	pub count: i32
}

#[derive(Deserialize)]
pub struct LeaderboardResultRest {
	pub data: Vec<LeaderboardResultRestItem>
}

pub async fn getLeaderboard_impl(context: &Context, hrs: i32, k: i32) -> FieldResult<LeaderboardResult> {
	let req = json!({
		"hrs": hrs,
		"size": k
	});
	type A = Vec<LeaderboardResultRestItem>;
	let result = postJSON!(A, format!("{}/ranking/tag_contributor.do", BACKEND_URL), req, context);
	if result.status == "SUCCEED" {
		let result = result.data.unwrap();
		let items = result
			.into_iter()
			.map(|o| LeaderboardResultItem { user_id: o._id.to_string(), count: o.count })
			.collect::<Vec<_>>();
		Ok(LeaderboardResult {
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

