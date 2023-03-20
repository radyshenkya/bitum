use chrono::prelude::*;
use pulldown_cmark::{html, Event, Options, Parser};
use rand::{Rng, SeedableRng};

pub fn display_timestamp_date(timestamp: i64) -> String {
    let naive_date_time = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let date_time: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);

    format!(
        "{:02}/{:02}/{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year()
    )
}

pub fn parse_markdown_to_html(markdown: String) -> String {
    let escaped_html_markdown = html_escape::encode_safe(&markdown);

    let mut options = Options::empty();

    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&escaped_html_markdown, options);
    let parser = parser.map(|event| match event {
        Event::SoftBreak => Event::HardBreak,
        _ => event,
    });

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    html_buf
}

// https://singlecolorimage.com/api.html
const SINGLE_COLOR_IMAGE_API_URL: &str = "https://singlecolorimage.com/get/";

pub fn get_random_color_image_url(seed_str: String, width: i32, height: i32) -> String {
    let seed: u64 = seed_str
        .as_bytes()
        .iter()
        .map(|byte| byte.clone() as u64)
        .sum::<u64>();

    let random_color = get_random_rgb_color(Some(seed));
    let random_color = (
        std::cmp::max(random_color.0, 25),
        std::cmp::max(random_color.1, 25),
        std::cmp::max(random_color.1, 25),
    );

    format!(
        "{}{:02x?}{:02x?}{:02x?}/{}x{}.png",
        SINGLE_COLOR_IMAGE_API_URL, random_color.0, random_color.1, random_color.2, width, height
    )
}

pub fn get_random_rgb_color(seed: Option<u64>) -> (u8, u8, u8) {
    if let Some(seed) = seed {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        (rng.gen(), rng.gen(), rng.gen())
    } else {
        let mut rng = rand::thread_rng();
        (rng.gen(), rng.gen(), rng.gen())
    }
}
