use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IftttWebhookError {
    reason: String,
    side: Option<Box<dyn Error>>,
}

impl IftttWebhookError {
    pub fn new(reason: String) -> Self {
        IftttWebhookError { reason, side: None }
    }
}

impl Error for IftttWebhookError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.side.as_ref().map(|e| e.as_ref())
    }
}

impl fmt::Display for IftttWebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<reqwest::Error> for IftttWebhookError {
    fn from(err: reqwest::Error) -> Self {
        IftttWebhookError {
            reason: err.to_string(),
            side: Some(Box::new(err)),
        }
    }
}
