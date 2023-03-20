use chrono::{Datelike, TimeZone, Timelike, Utc};
use log::info;
use pulldown_cmark::{html, Event, Options, Parser};

pub fn display_timestamp_date(timestamp: i64) -> String {
    info!("{}", timestamp);

    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();

    format!(
        "{:02}/{:02}/{:02} {:02}:{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year(),
        date_time.hour(),
        date_time.minute()
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
