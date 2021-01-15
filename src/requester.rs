pub fn requester(url: String) -> String {
    let res = reqwest::blocking::get(&url);

    let content = match res {
        Ok(good_resp) => good_resp,
        Err(_e) => return "failed".to_string(),
    };

    let content = match content.text() {
        Ok(str_val) => str_val,
        Err(_e) => "failed".to_string(),
    };

    content
}