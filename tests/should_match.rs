use std::path::Path;

use rstest::rstest;
use tokio::fs;

#[rstest]
#[tokio::test(flavor = "multi_thread")]
pub async fn should_match(
    #[dirs]
    #[files("tests/data/*")]
    #[by_ref]
    data: &Path,
) {
    let expected = image::open(data.join("expected.png")).unwrap();

    let html = fs::read_to_string(data.join("index.html")).await.unwrap();
    let rendered = html_renderer::render(&html, expected.width(), expected.height())
        .await
        .unwrap();
    let rendered = image::load_from_memory(&rendered).unwrap();

    let score = image_compare::rgba_hybrid_compare(&expected.into_rgba8(), &rendered.into_rgba8())
        .unwrap()
        .score;
    assert!(score >= 0.9);
}
