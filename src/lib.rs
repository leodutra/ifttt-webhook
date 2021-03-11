#[macro_use]
extern crate lazy_static;

mod error;
mod http;

use error::IftttWebhookError;
use http::HTTP_CLIENT;

use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct IftttWebhook {
    pub event: String,
    pub key: String,
}

pub fn build_hook_url(key: &str, event: &str) -> String {
    format!(
        "https://maker.ifttt.com/trigger/{event}/with/key/{key}",
        event = event,
        key = key,
    )
}

pub async fn trigger_webhook(key: &str, event: &str, body: Option<&HashMap<&str, &str>>) -> Result<(), impl Error> {
    let req = if let Some(body) = body {
        HTTP_CLIENT.post(&build_hook_url(key, event)).json(body)
    } else {
        HTTP_CLIENT.get(&build_hook_url(key, event))
    };
    let res = req.send().await?;
    if res.status() != 200 {
        Err(IftttWebhookError::new(format!(
            "Invalid status {}.\n{:?}",
            res.status(),
            res
        )))
    } else {
        Ok(())
    }
}

impl IftttWebhook {
    pub fn url(&self) -> String {
        build_hook_url(&self.event,&self.key)
    }

    pub async fn trigger(&self, body: Option<&HashMap<&str, &str>>) -> Result<(), impl Error> {
        trigger_webhook(&self.key, &self.event, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::IftttWebhook;
    use std::collections::HashMap;
    use std::env;

    lazy_static! {
        // The Client holds a connection pool internally, so it is advised that you create one and reuse it.
        pub static ref KEY: String = env::var("IFTTT_KEY").unwrap();
    }

    #[tokio::test]
    async fn test_post() {
        // webhook https://maker.ifttt.com/trigger/test/with/key/{key}
        let mut values = HashMap::new();
        values.insert("value1", "value_1_test_value");
        values.insert("value2", "value_2_test_value");
        values.insert("value3", "value_3_test_value");
        let webhook = IftttWebhook {
            key: KEY.clone(),
            event: "test".into(),
        };
        let result = webhook.trigger(Some(&values)).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_get() {
        // webhook https://maker.ifttt.com/trigger/test/with/key/{key}
        let webhook = IftttWebhook {
            key: KEY.clone(),
            event: "test".into(),
        };
        let result = webhook.trigger(None).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_post_error_unauthorized() {
        // webhook https://maker.ifttt.com/trigger/test/with/key/{key}
        let mut values = HashMap::new();
        values.insert("value1", "value_1_test_value");
        values.insert("value2", "value_2_test_value");
        values.insert("value3", "value_3_test_value");
        let webhook = IftttWebhook {
            key: "wrong_key".into(),
            event: "test".into(),
        };
        let result = webhook.trigger(Some(&values)).await;
        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    async fn test_get_error_unauthorized() {
        // webhook https://maker.ifttt.com/trigger/test/with/key/{key}
        let webhook = IftttWebhook {
            key: "wrong_key".into(),
            event: "test".into(),
        };
        let result = webhook.trigger(None).await;
        assert_eq!(result.is_err(), true);
    }
}
