use candle_transformers::models::bert;

pub use bert::{BertModel, Config, DTYPE};
pub use tokenizers::{PaddingParams, Tokenizer};

extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`

    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}
