#### Reference

# Deep links

A deep link opens Grok Bot on your desktop or phone from a shortcut, note, or
another app. The app must be installed and signed in.

## Start a message

```text
grokbot://app/v1/prompt?id=<bot-id>&text=<message>
```

Opens the Bot's chat with `text`, up to 1,000 characters, in the message box.
The text is added only if the message box is empty, and it isn't sent until you
send it. Without `id`, the link uses your
[primary Bot](/grok-bot/bots#your-primary-bot), and does nothing if you haven't
set one. The Bot's ID is the end of its link: click the Bot's name, choose
**Share**, then **Copy link**. Encode `text`, for example `%20` for a space, or
build the link here. The web link opens the app from a browser, so it works where
`grokbot://` links aren't clickable.

