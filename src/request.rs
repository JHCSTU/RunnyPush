use reqwest::{self, Client, redirect};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use select::document::Document;

const QUERY_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>').add(b'?').add(b'[').add(b']');

#[derive(Debug)]
pub struct Request {
    url: String,
    vrf: String,
    username: String,
    password: String,
    session: String,
    obj: Client,
}

impl Request {
    pub fn new() -> Self {
        let client = Client::builder()
            .redirect(redirect::Policy::none()) // 禁止所有重定向
            .build().unwrap();
        Request {
            url: String::from("https://jhc.sunnysport.org.cn/"),
            vrf: String::from(""),
            username: String::from(""),
            password: String::from(""),
            obj: client,
            session: String::from(""),
        }
    }
    pub fn set_data(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;
    }
    pub async fn get_session(&mut self) {
        let resp = self.obj.post("https://jhc.sunnysport.org.cn/login/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36")
            .header("Cookie", format!("sessionid={}", self.session))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(
                &[
                    ("vrf", &self.vrf),
                    ("username", &self.username),
                    ("password", &self.password),
                    ("userType", &String::from("person")),
                    ("agency", &utf8_percent_encode("体育部", QUERY_SET).to_string()),
                ]
            )
            .send().await;
        match resp {
            Ok(resp) => {
                if let Some(cookie) = resp.headers().get("set-cookie") {
                    let cookie = cookie.to_str().unwrap().to_string();
                    let cookie = cookie.split(";").collect::<Vec<&str>>()[0].split("=").collect::<Vec<&str>>()[1];
                    self.session = String::from(cookie);
                }
            }
            Err(_) => {
                println!("Error: Failed to get response.");
            }
        }
    }
    pub async fn get_session_before(&mut self) {
        let resp = self.obj.get("https://jhc.sunnysport.org.cn/login/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send().await;
        match resp {
            Ok(resp) => {
                let mut cookie = String::from("");
                for (key, value) in resp.headers() {
                    if key == "set-cookie" {
                        cookie = value.to_str().unwrap().to_string();
                    }
                }
                let cookie = cookie.split(";").collect::<Vec<&str>>()[0].split("=").collect::<Vec<&str>>()[1];
                self.session = String::from(cookie);

                let body = resp.text().await;
                match body {
                    Ok(body) => {
                        let vrf = body.split("name=\"vrf\" value=\"").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[0];
                        self.vrf = String::from(vrf);
                    }
                    Err(_) => {
                        println!("Error: Failed to get body.");
                    }
                }
            }
            Err(_) => {
                println!("Error: Failed to get response.");
            }
        }
    }
    pub async fn get_sunny_list(&mut self) -> Vec<String> {
        let resp = self.obj.get("https://jhc.sunnysport.org.cn/runner/achievements.html").header(
            "Cookie", format!("sessionid={}", self.session),
        ).send().await;
        if let Ok(resp) = resp {
            let body = resp.text().await;
            match body {
                Ok(body) => {
                    let document = Document::from(body.as_str());
                    let td_list = document.find(select::predicate::Name("td"));
                    let mut sunny_list: Vec<String> = Vec::new();
                    let mut temp_year = String::new();
                    let mut temp_meters = String::new();
                    let mut temp_speed = String::new();
                    let mut temp_ok = false;
                    let mut ok = false;
                    for td in td_list {
                        if td.text().contains("年") {
                            temp_year.push_str(td.text().as_str());
                        }
                        if td.text().contains("m") && !td.text().contains("m/s") {
                            temp_meters.push_str(td.text().as_str());
                        }
                        if td.text().contains("m/s") {
                            temp_speed.push_str(td.text().as_str());
                        }
                        if !temp_year.is_empty() && !temp_meters.is_empty() && !temp_speed.is_empty() {
                            if let Some(span) = td.find(select::predicate::Name("span")).next() {
                                let class = span.attr("class");
                                match class {
                                    Some(class) => {
                                        if class.contains("glyphicon") {
                                            temp_ok = true;
                                            if class.contains("glyphicon-ok") {
                                                ok = true;
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                            if temp_ok {
                                sunny_list.push(format!("{}-{}-{}-{}", temp_year, temp_meters, temp_speed, {
                                    if ok {
                                        "true"
                                    } else {
                                        "false"
                                    }
                                }));
                                temp_year.clear();
                                temp_meters.clear();
                                temp_speed.clear();
                                ok = false;
                                temp_ok = false
                            }
                        }
                    }
                    return sunny_list;
                }
                Err(_) => {
                    println!("Error: Failed to get body.");
                }
            }
        } else {
            println!("Error: Failed to get response.");
        }
        return Vec::new();
    }
    pub async fn get_result(&mut self, username: String, password: String) -> Vec<String> {
        self.set_data(username, password);
        self.get_session_before().await;
        self.get_session().await;
        self.get_sunny_list().await;
        return self.get_sunny_list().await;
    }
}