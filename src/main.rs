extern crate telescreen;
extern crate slack;
extern crate getopts;

use telescreen::telescreen_handler::TelescreenHandler;
use telescreen::router::Router;
use std::env;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

/*
 * A hack for static building with OpenSSL and musl.
 * https://github.com/clux/muslrust/issues/5#issuecomment-244901775
 */
#[cfg(target_os = "linux")]
fn set_openssl_env() {
    match env::var("SSL_CERT_FILE") {
        Err(_) => env::set_var("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt"),
        Ok(_) => { /* noop */ },
    }
    match env::var("SSL_CERT_DIR") {
        Err(_) => env::set_var("SSL_CERT_DIR", "/etc/ssl/certs"),
        Ok(_) => { /* noop */ },
    }
}

fn main() {
    if cfg!(target_os = "linux") {
        set_openssl_env();
    }

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();

    opts.optopt("a", "api-key", "Slack API key for bot integration", "API_KEY");
    opts.optopt("c", "config", "Path to config file", "FILE");
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) },
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let config_path_string = match matches.opt_str("c") {
        Some(c) => c,
        None => {
            print_usage(&program, opts);
            return;
        },
    };
    let api_key = match matches.opt_str("a") {
        Some(a) => a,
        None => {
            print_usage(&program, opts);
            return;
        },
    };

    let router = Router::new(&String::from(config_path_string));
    let mut handler = TelescreenHandler::new(router);
    let rtm_client = slack::RtmClient::login_and_run(&api_key, &mut handler);
    match rtm_client {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
