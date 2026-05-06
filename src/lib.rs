use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

pub struct Renderer {
    pub browser: Browser,
}
