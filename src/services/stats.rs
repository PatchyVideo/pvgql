
use juniper::graphql_value;


use juniper::FieldResult;

use crate::models::TagWithPopularity;
use crate::{common::*};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::convert::{TryFrom, TryInto};
use crate::models::*;
use crate::context::Context;

#[derive(Clone, Serialize, Deserialize)]
pub struct StatsTags {
    pub id: i32,
    pub count: i32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
	pub users: i32,
	pub top_tags: Vec<StatsTags>,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description="Stats")]
impl Stats {
	/// Num of users
	pub fn users(&self) -> i32 {
		self.users
	}
	// Top 20 tags
	pub async fn top_tags(&self, context: &Context) -> FieldResult<Option<Vec<TagWithPopularity>>> {
		let tagids = self.top_tags.iter().map(|k| k.id).collect::<Vec<_>>();
        let mut tagobjs: Vec<TagObjectValue> = super::editTags::getTagObjectsBatch_impl(context, super::editTags::GetTagObjectsBatchParameters {
            tagid: tagids
        }).await?;
        Ok(Some(self.top_tags.iter().zip(tagobjs.iter_mut()).map(|(k, v)| {
            TagWithPopularity {
                popluarity: k.count as _,
                tag: v.clone()
            }
        }).collect::<Vec<_>>()))
	}
}


pub async fn getStats_impl(context: &Context) -> FieldResult<Stats> {
    let result = postJSON!(Stats, format!("{}/stats.do", BACKEND_URL), EmptyJSON {}, context);
    
    let r = result.data.unwrap();
    Ok(r)
}
