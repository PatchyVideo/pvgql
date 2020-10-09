
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

macro_rules! postJSON {
    ($t:ident, $u:expr, $j:expr) => {
        {
            let client = reqwest::blocking::Client::new();
            let response = client.post(&$u).json(&$j).send()?;
            if response.status().is_success() {
                let obj : RestResult::<$t> = response.json()?;
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

