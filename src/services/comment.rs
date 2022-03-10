use juniper::{graphql_value, meta::ObjectMeta};


use juniper::{FieldResult, ScalarValue, GraphQLEnum};
use serde_json::json;

use crate::common::*;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
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

#[derive(juniper::GraphQLEnum, Clone, Serialize, Deserialize)]
pub enum CommentType {
	Video,
	Playlist
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters posting a comment", Context = Context)]
pub struct PostCommentParameters {
	/// Target vid, pid or comment_id (ObjectId)
	pub target_id: String,
	/// Type of comment
    pub comment_type: CommentType,
	/// To filter or not
	pub filter: bool,
	/// Content
	pub content: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommentAndThread {
	pub comment: Comment,
	pub thread: Thread
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="A comment and a thread")]
impl CommentAndThread {
	pub fn comment(&self) -> &Comment {
		&self.comment
	}
	pub fn thread(&self) -> &Thread {
		&self.thread
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PostCommentResponse {
	pub thread_id: String,
	pub cid: String
}
#[juniper::graphql_object(Context = Context)]
#[graphql(description="PostCommentResponse")]
impl PostCommentResponse {
	pub fn comment_id(&self) -> ObjectId {
		bson::oid::ObjectId::from_str(&self.cid).unwrap()
	}
	pub async fn thread(&self, context: &Context) -> FieldResult<Thread> {
		getThread_impl(context, GetThreadParameters { thread_id: self.thread_id.clone() }).await
	}
}

pub async fn postComment_impl(context: &Context, para: PostCommentParameters) -> FieldResult<PostCommentResponse> {
	let result = match para.comment_type {
		CommentType::Video => {
			let req = json!({
				"vid": para.target_id,
				"text": para.content
			});
			if para.filter {
				postJSON!(PostCommentResponse, format!("{}/comments/add_to_video.do", BACKEND_URL), req, context)
			} else {
				postJSON!(PostCommentResponse, format!("{}/comments/add_to_video_unfiltered.do", BACKEND_URL), req, context)
			}
		},
		CommentType::Playlist => {
			let req = json!({
				"vid": para.target_id,
				"text": para.content
			});
			if para.filter {
				postJSON!(PostCommentResponse, format!("{}/comments/add_to_playlist.do", BACKEND_URL), req, context)
			} else {
				postJSON!(PostCommentResponse, format!("{}/comments/add_to_playlist_unfiltered.do", BACKEND_URL), req, context)
			}
		},
	};
	if result.status == "SUCCEED" {
		let mut ret = result.data.as_ref().unwrap().clone();
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

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for posting a reply", Context = Context)]
pub struct PostReplyParameters {
	/// Target comment_id (ObjectId)
	pub reply_to: String,
	/// To filter or not
	pub filter: bool,
	/// Content
	pub text: String
}

pub async fn postReply_impl(context: &Context, para: PostReplyParameters) -> FieldResult<bool> {
	let result = if para.filter {
		postJSON!(EmptyJSON, format!("{}/comments/reply.do", BACKEND_URL), para, context)
	} else {
		postJSON!(EmptyJSON, format!("{}/comments/reply_unfiltered.do", BACKEND_URL), para, context)
	};
	if result.status == "SUCCEED" {
		Ok(true)
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
#[graphql(description="required parameters for editing a comment", Context = Context)]
pub struct EditCommentParameters {
	/// Target comment_id (ObjectId)
	pub cid: String,
	/// To filter or not
	pub filter: bool,
	/// Content
	pub text: String
}

pub async fn editComment_impl(context: &Context, para: EditCommentParameters) -> FieldResult<bool> {
	let result = if para.filter {
		postJSON!(EmptyJSON, format!("{}/comments/edit.do", BACKEND_URL), para, context)
	} else {
		postJSON!(EmptyJSON, format!("{}/comments/edit_unfiltered.do", BACKEND_URL), para, context)
	};
	if result.status == "SUCCEED" {
		Ok(true)
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

pub enum EditCommentOp {
	Del,
	Hide,
	Pin(bool)
}

pub async fn editCommentOp_impl(context: &Context, para: EditCommentOp, cid: String) -> FieldResult<bool> {
	let result = match para {
		EditCommentOp::Del => {
			let req = json!({
				"cid": cid
			});
			postJSON!(EmptyJSON, format!("{}/comments/del.do", BACKEND_URL), req, context)
		},
		EditCommentOp::Hide => {
			let req = json!({
				"cid": cid
			});
			postJSON!(EmptyJSON, format!("{}/comments/hide.do", BACKEND_URL), req, context)
		},
		EditCommentOp::Pin(pinned) => {
			let req = json!({
				"cid": cid,
				"pinned": pinned
			});
			postJSON!(EmptyJSON, format!("{}/comments/pin.do", BACKEND_URL), req, context)
		},
	};
	if result.status == "SUCCEED" {
		Ok(true)
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
