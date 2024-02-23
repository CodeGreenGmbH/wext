pub trait TextExt {
    fn create(s: impl AsRef<str>) -> Self;
}

impl TextExt for web_sys::Text {
    fn create(s: impl AsRef<str>) -> Self {
        gloo::utils::document().create_text_node(s.as_ref())
    }
}

#[macro_export]
macro_rules! text {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        web_sys::Text::create(s)
    }}
}
pub use text;
