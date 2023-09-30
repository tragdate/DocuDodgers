use crate::r#struct::Root;
use openssl::ssl::{SslConnector, SslMethod};
use serde_json::Value;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

fn build_request(cookie: &str, method: &str, endpoint: &str, data: Option<&str>) -> String {
    let mut request = format!(
        "\
        {} {} HTTP/1.1\r\n\
        Host: data.stackexchange.com\r\n\
        User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0\r\n\
        Accept: */*\r\n\
        Accept-Language: en-US,en;q=0.5\r\n\
        Accept-Encoding: gzip\r\n\
        X-Requested-With: XMLHttpRequest\r\n\
        Origin: https://data.stackexchange.com\r\n\
        Connection: keep-alive\r\n\
        Cookie: {}\r\n\
        Sec-Fetch-Dest: empty\r\n\
        Sec-Fetch-Mode: cors\r\n\
        Sec-Fetch-Site: same-origin\r\n",
        method, endpoint, cookie,
    );
    if method.to_uppercase() == "POST" {
        if let Some(data) = data {
            request += &format!(
                "\
            Content-Type: application/x-www-form-urlencoded\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
                data.len(),
                data
            );
        }
    } else {
        request += "\r\n";
    }
    request
}

fn send_request(host: &str, request: String, connector: &SslConnector) -> Vec<u8> {
    let mut stream = connector.connect(host, TcpStream::connect((host, 443)).unwrap()).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    let split_at = response.windows(4).position(|window| window == &[13, 10, 13, 10]).unwrap() + 4;
    response.split_off(split_at)
}

pub fn get_data(cookie: &str) -> Root {
    let mut file = std::fs::File::open(cookie).unwrap();
    let mut cookie = String::new();
    file.read_to_string(&mut cookie).unwrap();
    let cookie = cookie.trim().to_string();
    let query = fs::read_to_string("query.sql").unwrap().trim().to_string().replace("+", "%2B").replace(" ", "+").replace("\n", "+");
    let data = format!("title=&description=&sql={}", query);
    let endpoint = "/query/save/1";
    let request = build_request(&cookie, "POST", endpoint, Some(&data));
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let body = send_request("data.stackexchange.com", request.clone(), &connector);
    let mut body = crate::utils::decompress(&body);
    loop {
        let temp_json: Value = serde_json::from_slice(&body).unwrap();
        if let Some(running) = temp_json.get("running").and_then(|r| r.as_bool()) {
            if running {
                let job_id = temp_json.get("job_id").and_then(|j| j.as_str()).unwrap();
                let job_endpoint = format!("/query/job/{}", job_id);
                let job_request = build_request(&cookie, "GET", &job_endpoint, None);
                body = send_request("data.stackexchange.com", job_request, &connector);
                body = crate::utils::decompress(&body);

                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
        }
        if let Ok(response_json) = serde_json::from_slice::<Root>(&body) {
            return response_json;
        }
    }
}
