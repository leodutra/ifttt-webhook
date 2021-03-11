# IFTTT Webhook

[![crate.io](https://img.shields.io/crates/v/ifttt-webhook)](https://crates.io/crates/ifttt-webhook)

A simple Rust async library for triggering IFTTT events using webhooks.

## Installation

Installation can be performed using [`cargo add`](https://github.com/killercup/cargo-edit):

```sh
cargo add ifttt-webhook
```

## Usage

```rust
use ifttt_webhook::IftttWebhook;
use std::collections::HashMap;

// IFTTT Webhook key, available under "Documentation"
// at https://ifttt.com/maker_webhooks/.
let ifttt_key = 'your_ifttt_webhook_key';

let webhook = IftttWebhook {
    key: ifttt_key,
    event: "event_name",
};

// this will trigger a GET to https://maker.ifttt.com/trigger/{event}/with/key/{key}
webhook.trigger(None).await;
```

The `.trigger` method can be used to pass values as shown in the example below:
```rust
use ifttt_webhook::IftttWebhook;

// IFTTT Webhook key, available under "Documentation"
// at https://ifttt.com/maker_webhooks/.
let ifttt_key = 'your_ifttt_webhook_key';

let mut values = HashMap::new();
values.insert("value1", "value_1_test_value");
values.insert("value2", "value_2_test_value");
values.insert("value3", "value_3_test_value");

let webhook = IftttWebhook {
    key: ifttt_key,
    event: "event_name",
};

// this will trigger a POST to https://maker.ifttt.com/trigger/{event}/with/key/{key}
webhook.trigger(Some(&values)).await;
```

## Bugs and feedback

If you discover a bug please report it [here](https://github.com/leodutra/ifttt-webhook/issues/new).
Express gratitude [here](https://patreon.com/leodutra).

Mail me at leodutra.br+foss@gmail.com, or on twitter [@leodutra](http://twitter.com/leodutra).

## License

MIT @ [Leo Dutra](https://github.com/leodutra)
