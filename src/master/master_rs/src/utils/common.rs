// macros
macro_rules! get_env {
    ($var:expr, $default:expr) => {
        std::env::var($var).unwrap_or_else(|_| $default.to_string())
    };

    ($var:expr, $default:expr, $type:ty) => {
        std::env::var($var)
            .ok()
            .and_then(|v| v.parse::<$type>().ok())
            .unwrap_or($default)
    };
}

pub(crate) use get_env;

// functions
pub fn round_dur(dur: f32, ndigits: Option<usize>) -> f32 {

    let big = (dur / 10.0).floor() * 10.0;
    let small = dur - big;
    let factor = 10_u32.pow(ndigits.unwrap_or(2) as u32) as f32;
    return big + (small * factor).floor() / factor;
}
