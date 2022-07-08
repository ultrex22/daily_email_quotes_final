#![allow(dead_code)]
#![allow(unused_variables)]

// This code was converted from a python project from 100 Days of Code that I did.

mod _secret;
use _secret::*;

use std::fs;

use chrono::prelude::*;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;
use lettre::{SmtpTransport, Transport};
use rand::prelude::*;

fn main() {
    let now = Local::now();
    let day = now.day();

    let all_quotes = fs::read_to_string("src/quotes.txt").expect("error opening file");
    let clean_quotes: Vec<_> = all_quotes.split("\n").collect();

    let mut random_num_gen = thread_rng();
    let quote = clean_quotes.choose(&mut random_num_gen).unwrap();

    if day == now.day() {
        send_quote(quote);
    }
}

fn send_quote(quote: &str) {
    // send email
    use lettre::message::Message;

    let message = Message::builder()
        .from(Y_EMAIL.parse().expect("error in FROM email"))
        .to(G_EMAIL.parse().expect("error in TO email"))
        .to("darciebrilhante@gmail.com"
            .parse()
            .expect("error in TO email 2"))
        .subject("Weekly Positive Quote -Sent from a Rust program!")
        .body(String::from(quote))
        .expect("error creating body to String");

    // Create TLS transport on port 587 with STARTTLS
    let sender = SmtpTransport::starttls_relay("smtp.mail.yahoo.com")
        .expect("error starting TTLS relay")
        // Add credentials for authentication
        .credentials(Credentials::new(
            Y_EMAIL.to_string(),
            Y_APP_PASS.to_string(),
        ))
        // Configure expected authentication mechanism
        .authentication(vec![Mechanism::Plain])
        // Connection pool settings
        .pool_config(PoolConfig::new().max_size(20))
        .build();

    // Send the email via remote relay
    let result = sender.send(&message).expect("error sending email;");
}
