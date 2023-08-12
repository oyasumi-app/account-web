use std::time::{Duration, SystemTime};

use api_types::v1::DateTimeUtc;

mod time;
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    pub fn get_unix_timestamp() -> f64;
}

pub fn get_current_time() -> DateTimeUtc {
    #[allow(unused_unsafe)]
    let now = unsafe { get_unix_timestamp() };
    let now = DateTimeUtc::from(SystemTime::UNIX_EPOCH + Duration::from_secs_f64(now));
    now
}
