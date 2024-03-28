use clap::{command, Arg, ArgAction};

pub fn execute() {
    let input = get_args();

    println!("{:#?}", input);
}

#[derive(Debug)]
struct Input {
    url: Vec<String>,
    method: HTTPMethod,
    headers: Vec<String>,
}

#[derive(Debug)]
enum HTTPMethod {
    POST,
    GET,
    PATCH,
    PUT,
    DELETE,
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

    let url = matches
        .get_many::<String>("url")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let method = match &matches.get_one::<String>("method").unwrap()[..] {
        "GET" => HTTPMethod::GET,
        "POST" => HTTPMethod::POST,
        "PATCH" => HTTPMethod::PATCH,
        "PUT" => HTTPMethod::PUT,
        "DELETE" => HTTPMethod::DELETE,
        _ => panic!("Inaproppriate request method"),
    };

    let headers = matches
        .get_many::<String>("headers")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    Input {
        url,
        method,
        headers,
    }
}
