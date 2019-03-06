extern crate notify;
extern crate log;
extern crate simple_logger;
extern crate clap;
extern crate reqwest;


use log::{debug, info,error};
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use std::process;
use std::str::FromStr;



fn main() {

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("VOLUME_PATH")
                .long("path")
                .short("p")
                .default_value("")
                .help("the config map volume directory to watch for updates")
        )
        .arg(
            clap::Arg::with_name("WEBHOOK_URL")
                .long("webhook_url")
                .short("u")
                .default_value("")
                .help("the HTTP method url to use to send the webhook")
        )
        .arg(
            clap::Arg::with_name("WEBHOOK_METHOD")
                .long("webhook_method")
                .short("m")
                .default_value("POST")
                .help("the HTTP method url to use to send the webhook: GET|POST")
        )
        .arg(
            clap::Arg::with_name("WEBHOOK_STATUS_CODE")
                .long("webhook_status_code")
                .short("c")
                .default_value("200")
                .help("the HTTP status code indicating successful triggering of reload")
        )
        .arg(
            clap::Arg::with_name("LOG_LEVEL")
                .long("log_level")
                .short("l")
                .default_value("info")
                .help("log level: error|warn|info|debug|trace")
        )
        .get_matches();


    let log_level: log::Level = matches
        .value_of("LOG_LEVEL")
        .unwrap()
        .parse()
        .expect("unable to parse log level");

    debug!("log level is {}", log_level);

    simple_logger::init_with_level(log_level).unwrap();

    let volume_path = matches
        .value_of("VOLUME_PATH")
        .unwrap();


   if volume_path == "" {
       error!("volume_path can be empty!");
       process::exit(1);
   }

    debug!("watch path is {}", volume_path);


    let webhook_url = matches
        .value_of("WEBHOOK_URL")
        .unwrap();


    if webhook_url == "" {
        error!("webhook_urls can be empty!");
        process::exit(1);
    }

    debug!("webhook_url is {}", webhook_url);


    let webhook_method_str = matches
        .value_of("WEBHOOK_METHOD")
        .unwrap();

    let webhook_method = reqwest::Method::from_str(webhook_method_str).unwrap();


    match webhook_method {
        reqwest::Method::GET | reqwest::Method::POST => debug!("webhook method is {}", webhook_method),
        _ => {
            error!("GET | POST method can  support for now!");
            process::exit(1);
        },
    }


    let webhook_status_code: reqwest::StatusCode  = matches
        .value_of("WEBHOOK_STATUS_CODE")
        .unwrap()
        .parse()
        .expect("unable to parse http method");

    debug!("webhook status code  is {}", webhook_status_code);

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(volume_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(op), cookie: _}) => {
                if op == notify::op::CREATE {
                    if file_base(path.to_str().unwrap()) == "..data" {
                        info!("{}", "config map updated");
                        let client = reqwest::Client::new();
                        match client.request(webhook_method.clone(), webhook_url).send() {
                            Ok(res) => {
                                if res.status() != webhook_status_code {
                                    error!("error: Received response code {}, expected: {} ", res.status(), webhook_status_code);
                                }
                            },
                            Err(e) => error!("webhook error: {:?}", e),
                        }

                        info!("{}", "successfully triggered reload")
                    }
                }
            },
            Ok(event) => debug!("broken event: {:?}", event),
            Err(e) => error!("watch error: {:?}", e),
        }
    }
}

fn file_base(_path: &str) -> &str {
    if _path == "" {
        return ".";
    }
    let v: Vec<&str> = _path.split("/").collect();

    return  v[v.len() -1 ];
}