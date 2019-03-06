extern crate notify;
extern crate log;
extern crate simple_logger;
extern crate clap;


use log::{debug, info,error};
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use std::process;


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
            clap::Arg::with_name("WEBHOOK_URLS")
                .long("webhook_urls")
                .short("u")
                .default_value("")
                .help("the HTTP method url to use to send the webhook")
        )
        .arg(
            clap::Arg::with_name("WEBHOOK_METHOD")
                .long("webhook_method")
                .short("m")
                .default_value("POST")
                .help("the HTTP method url to use to send the webhook")
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


    let webhook_urls: Vec<_> = matches.values_of("WEBHOOK_URLS").unwrap().collect();

    if webhook_urls.len() == 0 {
        error!("webhook_urls can be empty!");
        process::exit(1);
    }


    let webhook_method: reqwest::Method = matches
        .value_of("WEBHOOK_METHOD")
        .unwrap()
        .parse()
        .expect("unable to parse http method");

    debug!("webhook method is {}", webhook_method);

    let webhook_status_code: i32 = matches
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
                        let mut easy = Easy::new();

                        for _url in webhook_urls.iter() {
                            easy.me
                            easy.url(_url).unwrap();

                            easy.perform().unwrap();

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