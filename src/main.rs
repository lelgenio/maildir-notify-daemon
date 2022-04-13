use std::path::PathBuf;

use hotwatch::blocking::{
    Flow,
    Hotwatch,
};
use mailparse::{
    self,
    MailHeaderMap,
};
use notify_rust::Notification;

fn main() {
    let mut hotwatch =
        Hotwatch::new_with_custom_delay(std::time::Duration::from_secs(0))
            .expect("hotwatch failed to initialize!");

    for arg in std::env::args().skip(1) {
        if let Err(e) = hotwatch.watch(&arg, handle_event) {
            eprintln!("Failed to watch arg '{}': {}", &arg, e);
        }
    }

    hotwatch.run();
}

fn handle_event(event: hotwatch::Event) -> hotwatch::blocking::Flow {
    if let hotwatch::Event::Create(newfile) = event {
        if let Err(e) = _handle_event(newfile) {
            eprintln!("{}", e);
        };
    };

    Flow::Continue
}

fn _handle_event(newfile: PathBuf) -> Result<(), String> {
    let raw_content = std::fs::read(&newfile).map_err(|e| {
        format!("Cannot read file '{}': {:?}", newfile.to_string_lossy(), e)
    })?;

    let mail_content = mailparse::parse_mail(&raw_content).map_err(|e| {
        format!("Cannot parse file '{}': {:?}", newfile.to_string_lossy(), e)
    })?;

    let headers = mail_content.get_headers();

    let from = headers.get_first_value("From").ok_or_else(|| {
        format!("Cannot parse file '{}'", newfile.to_string_lossy())
    })?;

    let subject = headers.get_first_value("Subject").unwrap_or_else(|| {
        mail_content
            .get_body()
            .unwrap_or("".to_string())
            .lines()
            .filter(|line| line.len() != 0)
            .next()
            .unwrap_or("")
            .to_string()
    });

    Notification::new()
        .summary(&format!("From: {}", from))
        .body(&format!("Subject: {}", subject))
        .icon("mail-unread-symbolic")
        .show()
        .map(|_| ())
        .map_err(|e| format!("Could not send notification: {}", e))
}
