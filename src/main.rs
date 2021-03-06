//extern crate imap as eimap;
extern crate native_tls;
extern crate serde;
extern crate serde_yaml;
extern crate corona;
extern crate imap;
extern crate tokio;

pub mod model;

use model::config::Config;
use corona::prelude::*;
//
//fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
//    let domain = "imap.example.com";
//    let tls = native_tls::TlsConnector::builder().build().unwrap();
//
//    // we pass in the domain twice to check that the server's TLS
//    // certificate is valid for the domain we're connecting to.
//    let client = imap::connect((domain, 993), domain, &tls).unwrap();
//
//    // the client we have here is unauthenticated.
//    // to do anything useful with the e-mails, we need to log in
//    let mut imap_session = client
//        .login("me@example.com", "password")
//        .map_err(|e| e.0)?;
//
//    // we want to fetch the first email in the INBOX mailbox
//    imap_session.select("INBOX")?;
//
//    // fetch message number 1 in this mailbox, along with its RFC822 field.
//    // RFC 822 dictates the format of the body of e-mails
//    let messages = imap_session.fetch("1", "RFC822")?;
//    let message = if let Some(m) = messages.iter().next() {
//        m
//    } else {
//        return Ok(None);
//    };
//
//    // extract the message's body
//    let body = message.body().expect("message did not have a body!");
//    let body = std::str::from_utf8(body)
//        .expect("message was not valid utf-8")
//        .to_string();
//
//    // be nice to the server and log out
//    imap_session.logout()?;
//
//    Ok(Some(body))
//}

fn main() {
    let f = std::fs::File::open("config.yaml").unwrap();
    let config: Config = serde_yaml::from_reader(f).unwrap();
    println!("Config {:#?}", config);

    Coroutine::new().run(|| {
        for cfg in config.connections {
            model::imap::start_polling(&cfg);
        }
    });
}
