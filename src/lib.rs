use clap::{command, Arg, ArgAction};
use futures::stream::{self, StreamExt};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

struct Input {
    urls: Vec<String>,
    method: HTTPMethod,
    headers: Vec<String>,
    data: Option<String>,
}

enum HTTPMethod {
    Post,
    Get,
    Patch,
    Put,
    Delete,
}

pub async fn execute() {
    let input = get_args();

    make_request(input).await
}

fn get_args() -> Input {
    let matches = command!()
    .arg(
        Arg::new("method")
        .help("Specifies a custom request method to use when communicating with the HTTP server.")
        .short('X')
        .long("request")
        .default_value("GET")
    )
    .arg(
        Arg::new("headers")
        .help("Extra header to include in the request when sending HTTP to a server. \
        You may specify any number of extra headers.")
        .short('H')
        .long("header")
        .action(ArgAction::Append)
    )
    .arg(
        Arg::new("data")
        .help("Sends the specified data in a POST request to the HTTP server.")
        .short('d')
        .long("data")
    )
    .arg(
      Arg::new("url")
          .help("url to be use")
          .action(ArgAction::Append)
  )
    .get_matches();

    let urls = matches
        .get_many::<String>("url")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let method = match &matches.get_one::<String>("method").unwrap()[..] {
        "GET" => HTTPMethod::Get,
        "POST" => HTTPMethod::Post,
        "PATCH" => HTTPMethod::Patch,
        "PUT" => HTTPMethod::Put,
        "DELETE" => HTTPMethod::Delete,
        _ => panic!("Inappropriate request method"),
    };

    let headers = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let data = matches.get_one::<String>("data").cloned();

    Input {
        urls,
        method,
        headers,
        data,
    }
}

async fn make_request(input: Input) {
    let match_method = |url| match input.method {
        HTTPMethod::Get => Client::new().get(url),
        HTTPMethod::Post => Client::new().post(url),
        HTTPMethod::Patch => Client::new().patch(url),
        HTTPMethod::Put => Client::new().put(url),
        HTTPMethod::Delete => Client::new().delete(url),
    };

    let set_each_header = |mut req_builder: RequestBuilder| {
        for header in input.headers.iter() {
            let (key, value) = header.split_once(':').unwrap();
            req_builder = req_builder.header(key, value);
        }

        req_builder
    };

    let set_data_body = |mut req_builder: RequestBuilder| match input.data.to_owned() {
        None => req_builder,
        Some(data) => {
            req_builder = req_builder.body(data);
            req_builder
        }
    };

    stream::iter(input.urls.into_iter().map(match_method))
        .map(set_each_header)
        .map(set_data_body)
        .for_each_concurrent(None, |req_builder| async move {
            execute_request(req_builder).await;
        })
        .await;
}

async fn execute_request(req_builder: RequestBuilder) {
    let response = req_builder
        .send()
        .await
        .expect("Failed to execute request.");

    let response_body = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");

    println!("{}", response_body);
}

#[cfg(test)]
mod tests {
    use crate::{make_request, HTTPMethod, Input};
    use wiremock::matchers::{body_json, header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn make_get_request() {
        let mock_server = MockServer::start().await;
        let input = Input {
            urls: vec![mock_server.uri()],
            method: HTTPMethod::Get,
            headers: vec![],
            data: None,
        };

        let expected_response = serde_json::json!(
            {
                "args": {},
                "headers": {
                  "Accept": "*/*",
                  "Content-Length": "0",
                  "Host": "httpbin.org",
                  "X-Amzn-Trace-Id": "Root=1-6606bb7c-47f2b4960cd65d50161aa61d"
                },
                "origin": "179.54.218.77",
                "url": "https://httpbin.org/get"
              }
        );

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        make_request(input).await
    }

    #[tokio::test]
    async fn make_multiple_requests() {
        let mock_server = MockServer::start().await;
        let input = Input {
            urls: vec![mock_server.uri(), mock_server.uri()],
            method: HTTPMethod::Get,
            headers: vec![],
            data: None,
        };

        let expected_response = serde_json::json!(
            {
                "args": {},
                "headers": {
                  "Accept": "*/*",
                  "Content-Length": "0",
                  "Host": "httpbin.org",
                  "X-Amzn-Trace-Id": "Root=1-6606bb7c-47f2b4960cd65d50161aa61d"
                },
                "origin": "179.54.218.77",
                "url": "https://httpbin.org/get"
              }
        );

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .expect(2)
            .mount(&mock_server)
            .await;

        make_request(input).await
    }

    #[tokio::test]
    async fn make_post_request_with_headers_and_body() {
        let mock_server = MockServer::start().await;
        let input = Input {
            urls: vec![mock_server.uri()],
            method: HTTPMethod::Post,
            headers: vec![
                "Content-Type: application/json".into(),
                "Authorization: FakeBearer d6a715d502462ee00e67c4457d872d72ffa34c00".into(),
            ],
            data: Some(r#"{"key1":"value1", "key2":"value2"}"#.into()),
        };

        let expected_response = serde_json::json!(
            {
                "args": {},
                "data": "{\"key1\":\"value1\", \"key2\":\"value2\"}",
                "headers": {
                  "Accept": "*/*",
                  "Content-Length": "0",
                  "Content-Type": "application/json",
                  "Authorization": "FakeBearer d6a715d502462ee00e67c4457d872d72ffa34c00",
                  "Host": "httpbin.org",
                  "X-Amzn-Trace-Id": "Root=1-6606bb7c-47f2b4960cd65d50161aa61d"
                },
                "json": {
                    "key1": "value1",
                    "key2": "value2"
                  },
                "origin": "179.54.218.77",
                "url": "https://httpbin.org/post"
              }
        );

        Mock::given(header("Content-Type", "application/json"))
            .and(header(
                "Authorization",
                "FakeBearer d6a715d502462ee00e67c4457d872d72ffa34c00",
            ))
            .and(method("POST"))
            .and(body_json(
                serde_json::json!({"key1": "value1", "key2": "value2"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        make_request(input).await
    }
}
