use clap::{command, Arg, ArgAction};
use futures::stream::{self, StreamExt};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

#[derive(Debug)]
struct Input {
    urls: Vec<String>,
    method: HTTPMethod,
    headers: Vec<String>,
}

#[derive(Debug)]
enum HTTPMethod {
    Post,
    Get,
    Patch,
    Put,
    Delete,
}

pub async fn execute() {
    let input = get_args();

    stream::iter(input.urls.into_iter().map(|url| match input.method {
        HTTPMethod::Get => Client::new().get(url),
        HTTPMethod::Post => Client::new().post(url),
        HTTPMethod::Patch => Client::new().patch(url),
        HTTPMethod::Put => Client::new().put(url),
        HTTPMethod::Delete => Client::new().delete(url),
    }))
    .map(|mut req_builder| {
        for header in input.headers.iter() {
            let (key, value) = header.split_once(':').unwrap();
            req_builder = req_builder.header(key, value);
        }

        req_builder
    })
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
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    Input {
        urls,
        method,
        headers,
    }
}
