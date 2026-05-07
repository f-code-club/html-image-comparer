const HTML: &str = r#"
<div id="l"></div>
<div id="c"></div>
<div id="r"></div>
<style>
    body {
        background: #191210;
        margin: 0;
        position: relative;
    }

    #l {
        width: 60px;
        height: 30px;
        border-bottom-left-radius: 50px;
        border-bottom-right-radius: 50px;
        border: 20px solid #ECA03D;
        border-top: 0;
        z-index: 1;
        position: absolute;
        top: 50%;
        left: 50%;
        margin: 0 0 0 -150px;
    }

    #c {
        width: 100px;
        height: 100px;
        background: repeating-radial-gradient(circle at 50%, #84271C, #84271C 35%, #191210 35%, #191210 100%);
        border: 20px solid #ECA03D;
        border-radius: 100px;
        position: absolute;
        top: 50%;
        left: 50%;
        margin: -70px 0 0 -70px;
    }

    #r {
        width: 60px;
        height: 30px;
        border-top-left-radius: 50px;
        border-top-right-radius: 50px;
        border: 20px solid #ECA03D;
        border-bottom: 0;
        z-index: 1;
        position: absolute;
        top: 50%;
        left: 50%;
        margin: -50px 0 0 50px;
    }
</style>
"#;
const EXPECTED: &[u8] = include_bytes!("expected.png");

#[tokio::main]
pub async fn main() {
    let expected = image::load_from_memory(EXPECTED).unwrap();
    let rendered = html_renderer::render(HTML, expected.width(), expected.height())
        .await
        .unwrap();
    let rendered = image::load_from_memory(&rendered).unwrap();

    let score = image_compare::rgba_hybrid_compare(&expected.into_rgba8(), &rendered.into_rgba8())
        .unwrap()
        .score;
    println!("Score: {score}");
}
