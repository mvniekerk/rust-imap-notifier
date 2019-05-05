use crate::model::config::EmailConfig;

use std::time::Duration;

use corona::prelude::*;
use tokio::clock;
use tokio::prelude::*;
use tokio::runtime::current_thread;
use tokio::timer::Delay;

pub fn start_polling(config: &EmailConfig) {

    let config = config.clone();
    corona::spawn(move || {
        loop {
            println!("Before wait {:#?}", config.email.refresh);
            let timeout = Delay::new(clock::now() + Duration::from_secs(config.email.refresh));
            println!("Timeout {:#?}", timeout);
            timeout.coro_wait().unwrap();
            println!("After");
//            let tls = native_tls::TlsConnector::builder().build().unwrap();
//            let domain = config.email.domain.as_str();
//            let client = imap::connect((domain, 993), domain, &tls).unwrap();
//            let mut imap_session = client
//                .login(config.email.username.as_str(), config.email.password.as_str())
//                .map_err(|e| e.0).unwrap();

            for folder in config.folders.clone() {
                println!("Handling folder {:#?}", folder);
            }
        }


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
        println!("Should be start polling {:#?}", config);
    });

}