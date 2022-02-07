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
    let args = std::env::args(); // .collect::<Vec<_>>();
                                 // let path = args.get(1).expect("No path provided, exiting");

    let mut hotwatch =
        Hotwatch::new_with_custom_delay(std::time::Duration::from_secs(0))
            .expect("hotwatch failed to initialize!");

    println!("starting");

    for arg in args {
        hotwatch
            .watch(arg, handle_event)
            .expect("hotwatch failed to initialize!");
    }
    hotwatch.run();
    println!("exiting");
}

fn handle_event(event: hotwatch::Event) -> hotwatch::blocking::Flow {
    _handle_event(event);
    Flow::Continue
}

fn _handle_event(event: hotwatch::Event) {
    let newfile = match dbg! {event} {
        hotwatch::Event::Create(newfile) => newfile,
        _ => return,
    };

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

    eprintln!("getting from field");

    let from = match headers.get_first_value("From") {
        Some(f) => f,
        None => {
            eprintln!("Cannot parse file '{}'", newfile.to_string_lossy(),);
            return;
        }
    };

    eprintln!("getting subject field");

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

    eprintln!("setting subject field to {}", &subject);

    eprintln!("showing notification");

    Notification::new()
        .summary(&format!("From: {}", from))
        .body(&format!("Subject: {}", subject))
        .icon("mail-unread-symbolic")
        .show()
        .ok();
}
