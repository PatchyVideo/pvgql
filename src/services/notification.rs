use juniper::{graphql_value};
use std::collections::{BTreeMap, HashMap};

use crate::{common::*, context::Context, models::Subscription};
use juniper::{
	graphql_interface,
	GraphQLObject,
	execute,
	parser::{ParseError, ScalarToken, Spanning, Token},
	serde::de,
	EmptyMutation, FieldResult, InputValue, Object, ParseScalarResult, RootNode, ScalarValue,
	Value, Variables,
};
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, VideoItem};

use crate::services::users::{User};
use crate::services::users;

#[derive(GraphQLObject, Clone)]
#[graphql(description="NotificationObject for reply", impl = NotificationObjectValue, Context = Context)]
pub struct ReplyNotificationObject {
	pub _id: ObjectId,
	pub type_: String,
	pub time: DateTime<chrono::Utc>,
	pub read: bool,
	pub content: String,
	pub replied_by: User,
	/// Comment ID
	pub cid: ObjectId,
	/// One of 'forum', 'video', 'playlist'
	pub replied_type: String,
	/// Link to thread
	pub replied_obj: ObjectId
}

#[derive(GraphQLObject, Clone)]
#[graphql(description="base NotificationObject", impl = NotificationObjectValue, Context = Context)]
pub struct BaseNotificationObject {
	pub _id: ObjectId,
	pub type_: String,
	pub time: DateTime<chrono::Utc>,
	pub read: bool,
}

#[graphql_interface(for = [ReplyNotificationObject, BaseNotificationObject], Context = Context)] // enumerating all implementers is mandatory 
pub trait NotificationObject {
	async fn id(&self) -> &ObjectId;
	#[graphql(
		// overwrite the public name
		name = "type"
	)]
	async fn type_(&self) -> &String;
	async fn time(&self) -> &DateTime<chrono::Utc>;
	/// If this notification has been read
	async fn read(&self) -> bool;
}

#[juniper::graphql_interface]
impl NotificationObject for ReplyNotificationObject {
	async fn id(&self) -> &ObjectId {
		&self._id
	}

	async fn type_(&self) -> &String {
		&self.type_
	}

	async fn time(&self) -> &DateTime<Utc> {
		&self.time
	}

	async fn read(&self) -> bool {
		self.read
	}
}

#[juniper::graphql_interface]
impl NotificationObject for BaseNotificationObject {
	async fn id(&self) -> &ObjectId {
		&self._id
	}

	async fn type_(&self) -> &String {
		&self.type_
	}

	async fn time(&self) -> &DateTime<Utc> {
		&self.time
	}

	async fn read(&self) -> bool {
		self.read
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleNotificationResult {
	pub _id: ObjectId,
	#[serde(rename = "type")]
	pub type_: String,
	pub time: BsonDateTime,
	pub read: bool,
	pub to: ObjectId,
	#[serde(flatten)]
	pub other: std::collections::HashMap<String, serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListNotificationResult {
	pub notes: Vec<SingleNotificationResult>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="list notifications required parameters", Context = Context)]
pub struct ListNotificationParameters {
	pub offset: Option<i32>,
	pub limit: Option<i32>,
	// Whether or not to list all notifications, default only list unread
	pub list_all: Option<bool>
}

pub fn fetch_field<'a>(map: &'a HashMap<String, serde_json::Value>, val: &str) -> FieldResult<&'a serde_json::Value> {
	Ok(map.get(val).ok_or(juniper::FieldError::new("INTERNAL_SERVER_ERROR", graphql_value!({format!("Missing field '{}'", val)})))?)
}

pub fn value_to_oid(val: &serde_json::Value) -> Option<ObjectId> {
	match val.get("$oid") {
		Some(value) => {
			match value.as_str() {
				Some(s) => {
					match ObjectId::with_string(s) {
						Ok(oid) => Some(oid),
						Err(_) => None
					}
				}
				None => None
			}
		},
		None => None
	}
}

pub async fn listNotification_impl(context: &Context, para: ListNotificationParameters) -> FieldResult<Vec<NotificationObjectValue>> {
	let list_all = para.list_all.map_or(false, |f| f);
	let result = if list_all {
		postJSON!(ListNotificationResult, format!("{}/notes/list_all.do'", BACKEND_URL), para, context)
	} else {
		postJSON!(ListNotificationResult, format!("{}/notes/list_unread.do'", BACKEND_URL), para, context)
	};
	if result.status == "SUCCEED" {
		let ret = result.data.unwrap();
		let mut result_list = Vec::new();
		for note in ret.notes {
			let item = if note.type_ == "comment_reply" {
				let content = fetch_field(&note.other, "content")?.as_str().unwrap().to_string();
				let cid = value_to_oid(fetch_field(&note.other, "cid")?).unwrap();
				let replied_by_oid = value_to_oid(fetch_field(&note.other, "replied_by")?).unwrap();
				let replied_type = fetch_field(&note.other, "replied_type")?.as_str().unwrap().to_string();
				let replied_obj = value_to_oid(fetch_field(&note.other, "replied_obj")?).unwrap();
				let replied_by = users::getUser_impl(context, users::GetUserParameters {
					uid: replied_by_oid.to_string()
				}).await?;
				ReplyNotificationObject {
					_id: note._id,
					type_: note.type_,
					read: note.read,
					time: note.time.to_UtcDateTime(),
					replied_by: replied_by,
					replied_obj: replied_obj,
					replied_type: replied_type,
					content: content,
					cid: cid
				}.into()
			} else {
				BaseNotificationObject {
					_id: note._id,
					type_: note.type_,
					read: note.read,
					time: note.time.to_UtcDateTime()
				}.into()
			};
			result_list.push(item);
		};
		Ok(result_list)
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
