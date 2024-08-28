# Chatty

Chatty is a pet-project I am creating to learn Rust while solving a problem
that I face often. I will be on my desktop or in the terminal working while also
having a conversation with someone in Messages. Constantly, I have to shift
my focus to my phone, laptop, or the Messages app. Chatty is intended to fix
that by providing a TUI into Messages.

Will Chatty ever get that far? Possibly not. Upon further investigation, it
seems that Apple may have removed some of the AppleScript tie-ins that I planned
on leveraging to send messages. It may be possible to send individual messages
however advanced features such as group messages, editing, and so on may not
be in the picture. At the bare miniumum, I plan to create "mock" backends
to mimic the functionality (and also allow for testing).

## Architecture & Design

The `[ratatui](https://ratatui.rs/)` crate is used for the TUI. The overall
architecture follows the [Flux](https://ratatui.rs/concepts/application-patterns/flux-architecture/)
pattern and is heavily inspired by this [rust IRC client](https://github.com/Yengas/rust-chat-server/tree/main/tui)
using both `ratatui` and the Flux pattern.

## Issues & Feedback

Under the Issues I track both issues in the code that I need to fix in the
future as well as knowledge gaps where something worked, was required, etc.
that I don't completely understand and want to revisit later.

I am always open to feedback and improving my code - this is meant to be an
educational project after all! If you have any thoughts, code critiques, or
questions please feel free to open an Issue!
