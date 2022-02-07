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

    std::env::args().for_each(|arg| {
        hotwatch
            .watch(arg, handle_event)
            .expect("hotwatch failed to initialize!");
    });

    hotwatch.run();
}

fn handle_event(event: hotwatch::Event) -> hotwatch::blocking::Flow {
    if let hotwatch::Event::Create(newfile) = event {
        _handle_event(newfile)
    };

    Flow::Continue
}

fn _handle_event(newfile: PathBuf) {
    let raw_content = match std::fs::read_to_string(&newfile) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Cannot read file '{}': {:?}",
                newfile.to_string_lossy(),
                e
            );
            return;
        }
    };

    let mail_content = match mailparse::parse_mail(raw_content.as_bytes()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Cannot parse file '{}': {:?}",
                newfile.to_string_lossy(),
                e
            );
            return;
        }
    };

    let headers = mail_content.get_headers();

    let from = match headers.get_first_value("From") {
        Some(f) => f,
        None => {
            eprintln!("Cannot parse file '{}'", newfile.to_string_lossy(),);
            return;
        }
    };

    let subject = match headers.get_first_value("Subject") {
        Some(sub) => sub,
        None => mail_content
            .get_body()
            .unwrap_or("".to_string())
            .lines()
            .filter(|line| line.len() != 0)
            .next()
            .unwrap_or("")
            .to_string(),
    };

    Notification::new()
        .summary(&format!("From: {}", from))
        .body(&format!("Subject: {}", subject))
        .icon("mail-unread-symbolic")
        .show()
        .ok();
}
