use chrono::{Datelike, TimeZone, Timelike, Utc};
use pulldown_cmark::{
    html, Options, Parser,
};

pub fn display_timestamp_date(timestamp: i64) -> String {
    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();

    format!(
        "{:02}/{:02}/{:04} {:02}:{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year(),
        date_time.hour(),
        date_time.minute()
    )
}

pub fn parse_markdown_to_html(markdown: String) -> String {
    let escaped_html_markdown = html_escape::encode_safe(&markdown);

    let options = Options::empty();
    let parser = Parser::new_ext(&escaped_html_markdown, options);

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    html_buf
}
