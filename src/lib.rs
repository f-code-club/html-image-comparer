use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

pub struct Renderer {
    pub browser: Browser,
}

impl Renderer {
    pub async fn new() -> chromiumoxide::error::Result<Self> {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .new_headless_mode()
                .build()
                .expect("config for headless browser must be valid"),
        )
        .await?;

        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() {
                    break;
                }
            }
        });

        Ok(Self { browser })
    }
}
