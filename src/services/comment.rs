use juniper::{graphql_value, meta::ObjectMeta};


use juniper::{FieldResult, ScalarValue};

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;

use super::users::{self, User};

#[derive(Clone, Serialize, Deserialize)]
pub struct Comment {
	pub _id: ObjectId,
	pub thread: Option<ObjectId>,
	pub content: Option<String>,
	pub parent: Option<ObjectId>,
	pub children: Option<Vec<Comment>>,
	pub hidden: bool,
	pub deleted: bool,
	pub pinned: bool,
	pub upvotes: i32,
	pub downvotes: i32,
	pub meta: Meta,
	pub edited: Option<bool>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="A single comment")]
impl Comment {
	pub fn id(&self) -> &ObjectId {
		&self._id
	}
	pub async fn thread(&self, context: &Context) -> FieldResult<Option<Thread>> {
		Ok(match self.thread.as_ref() {
			Some(thread_id) => {
				Some(getThread_impl(context, GetThreadParameters {
					thread_id: thread_id.to_string()
				}).await?)
			},
			None => None
		})
	}
	pub fn content(&self) -> Option<String> {
		self.content.as_ref().map_or(None, |s| if s.len() == 0 {None} else {Some(s.clone())})
	}
	pub fn children(&self) -> Option<Vec<Comment>> {
		match self.children.as_ref() {
			Some(c) => {
				if c.len() > 0 {
					Some(c.clone())
				} else {
					None
				}
			},
			None => None
		}
	}
	pub fn parent(&self, context: &Context) -> &Option<ObjectId> {
		&self.parent
	}
	pub fn hidden(&self) -> bool {
		self.hidden
	}
	pub fn deleted(&self) -> bool {
		self.deleted
	}
	pub fn pinned(&self) -> bool {
		self.pinned
	}
	pub fn upvotes(&self) -> i32 {
		self.upvotes
	}
	pub fn downvotes(&self) -> i32 {
		self.downvotes
	}
	pub fn edited(&self) -> bool {
		self.edited.unwrap_or_default()
	}
	pub fn meta(&self) -> &Meta {
		&self.meta
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Thread {
	pub _id: ObjectId,
	pub count: i32,
	pub owner: ObjectId,
	pub obj_type: String,
	pub comments: Option<Vec<Comment>>
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="A single thread")]
impl Thread {
	pub fn id(&self) -> &ObjectId {
		&self._id
	}
	/// Number of comment in this thread, includes deleted ones but not replies
	pub fn count(&self) -> i32 {
		self.count
	}
	/// Owner of this thread, for video/playlist the owner is the whoever created the video/playlist
	pub async fn owner(&self, context: &Context) -> FieldResult<User> {
		let u = users::getUser_impl(context, users::GetUserParameters {
			uid: self.owner.to_string()
		}).await?;
		Ok(u)
	}
	/// One of 'video', 'playlist', 'user', 'forum'
	pub fn thread_type(&self) -> &String {
		&self.obj_type
	}
	pub async fn comments(&self, context: &Context) -> FieldResult<Option<Vec<Comment>>> {
		match self.comments.as_ref() {
			Some(c) => Ok(Some(c.clone())),
			None => {
				let t2 = getThread_impl(context, GetThreadParameters {
					thread_id: self._id.to_string()
				}).await?;
				Ok(t2.comments)
			}
		}
	}
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for viewing a thread", Context = Context)]
pub struct GetThreadParameters {
	/// ID of thread
    pub thread_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetThreadResponse {
	pub comments: Vec<Comment>,
	pub thread: Thread
}

pub async fn getThread_impl(context: &Context, para: GetThreadParameters) -> FieldResult<Thread> {
	let result = postJSON!(GetThreadResponse, format!("{}/comments/view.do", BACKEND_URL), para, context);
	if result.status == "SUCCEED" {
		let mut ret = result.data.as_ref().unwrap().thread.clone();
		ret.comments = Some(result.data.unwrap().comments);
		Ok(ret)
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
