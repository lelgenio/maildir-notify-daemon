# Maildir Notification Daemon

Waits for new files on provided dirs, parse them as email and send a notification with author and subject.

```sh
maildir-notify-daemon ~/.local/share/mail/*/*/new/
```

## Try it without sending emails

```sh
cp ~/.local/share/mail/personal/INBOX/cur/[ANY_MAIL_FILE] ~/.local/share/mail/personal/INBOX/new/
```
