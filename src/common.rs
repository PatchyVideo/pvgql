
macro_rules! postRawJSON {
	($t:ident , $u:expr, $j:expr) => {
		{
			let client = reqwest::blocking::Client::new();
			let response = client.post(&$u).json(&$j).send()?;
			if response.status().is_success() {
				println!("resp body: {:?}", response);
				let resp_str = response.text().unwrap();
				println!("body: {}", resp_str);
				let obj : $t = serde_json::from_str(resp_str.as_str())?; //response.json()?;
				Ok(obj)
			} else {
				let e: Error = response.json()?;
				Err(
					juniper::FieldError::new(
						e.code,
						graphql_value!({
							e.aux
						}),
					)
				)
			}
		}?
	};
}

use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EmptyJSON {

}

#[cfg(debug_assertions)]
pub const BACKEND_URL: &str = "https://thvideo.tv/be";

#[cfg(not(debug_assertions))]
pub const BACKEND_URL: &str = "http://web:5000";

macro_rules! postJSON {
	($t:ident, $u:expr, $j:expr, $c:ident) => {
		{
			let client = reqwest::Client::new();
			let response = match $c.session.as_ref() {
				Some(sess) => client.post(&$u).header("cookie", format!("session={}", sess)).json(&$j).send().await?,
				None => client.post(&$u).json(&$j).send().await?
			};
			if response.status().is_success() {
				let obj : RestResult::<$t> = response.json().await?;
				Ok(obj)
			} else {
				let e: Error = response.json().await?;
				Err(
					juniper::FieldError::new(
						e.code,
						graphql_value!({
							e.aux
						}),
					)
				)
			}
		}?
	};
}

