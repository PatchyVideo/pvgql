
use juniper::graphql_value;
use std::collections::BTreeMap;

use juniper::FieldResult;

use crate::{common::*, models::User};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::{Meta, Error, RestResult, BsonDateTime, Video, PlaylistMeta};
use crate::context::Context;

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="required parameters for get user", Context = Context)]
pub struct GetUserParameters {
	/// ID of user
    pub uid: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserProfile {
	pub bind_qq: Option<bool>,
	pub desc: String,
	pub username: String,
	pub image: String,
	pub email: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetProfileResult {
    pub profile: UserProfile,
    pub _id: ObjectId,
    pub meta: Meta,
}

pub async fn getUser_impl(context: &Context, para: GetUserParameters) -> FieldResult<User> {
    let result = postJSON!(GetProfileResult, format!("{}/user/profile.do", BACKEND_URL), para, context);
    if result.status == "SUCCEED" {
        let r = result.data.unwrap();
		Ok(User {
            _id: r._id,
            bind_qq: r.profile.bind_qq,
            desc: r.profile.desc,
            username: r.profile.username,
            email: r.profile.email,
            image: r.profile.image,
            meta: r.meta
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

pub async fn whoami_impl(context: &Context) -> FieldResult<String> {
    let result = postJSON!(String, format!("{}/user/whoami", BACKEND_URL), EmptyJSON {}, context);
    if result.status == "SUCCEED" {
        let r = result.data.unwrap();
		Ok(r)
	} else {
		Ok("NOT_LOGGED_IN".to_string())
	}
}

