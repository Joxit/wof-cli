mod json;
mod path;
pub use crate::utils::json::{JsonUtils, JsonValue, Object as JsonObject};
pub use crate::utils::path::{
  id_to_data_path_folder, id_to_data_path_geojson, id_to_path_folder, id_to_path_geojson,
};

pub fn get_available_country_codes() -> Vec<String> {
  vec![
    "ad", "ae", "af", "ag", "ai", "al", "am", "an", "ao", "aq", "ar", "as", "at", "au", "aw", "ax",
    "az", "ba", "bb", "bd", "be", "bf", "bg", "bh", "bi", "bj", "bl", "bm", "bn", "bo", "bq", "br",
    "bs", "bt", "bw", "by", "bz", "ca", "cc", "cd", "cf", "cg", "ch", "ci", "ck", "cl", "cm", "cn",
    "co", "cr", "cu", "cv", "cw", "cx", "cy", "cz", "de", "dj", "dk", "dm", "dn", "do", "dz", "ec",
    "ee", "eg", "eh", "er", "es", "et", "fi", "fj", "fk", "fm", "fo", "fr", "ga", "gb", "gd", "ge",
    "gf", "gg", "gh", "gi", "gl", "gm", "gn", "gp", "gq", "gr", "gs", "gt", "gu", "gw", "gy", "hk",
    "hm", "hn", "hr", "ht", "hu", "id", "ie", "il", "im", "in", "io", "iq", "ir", "is", "it", "je",
    "jm", "jo", "jp", "ke", "kg", "kh", "ki", "km", "kn", "ko", "kp", "kr", "kw", "ky", "kz", "la",
    "lb", "lc", "li", "lk", "lr", "ls", "lt", "lu", "lv", "ly", "ma", "mc", "md", "me", "mf", "mg",
    "mh", "mk", "ml", "mm", "mn", "mo", "mp", "mq", "mr", "ms", "mt", "mu", "mv", "mw", "mx", "my",
    "mz", "na", "nc", "ne", "nf", "ng", "ni", "nl", "no", "np", "nr", "nu", "nz", "om", "pa", "pe",
    "pf", "pg", "ph", "pk", "pl", "pm", "pn", "pr", "ps", "pt", "pw", "py", "qa", "re", "ro", "rs",
    "ru", "rw", "sa", "sb", "sc", "sd", "se", "sg", "sh", "si", "sj", "sk", "sl", "sm", "sn", "so",
    "sr", "ss", "st", "sv", "sx", "sy", "sz", "tc", "td", "tf", "tg", "th", "tj", "tk", "tl", "tm",
    "tn", "to", "tr", "tt", "tu", "tv", "tw", "tz", "ua", "ug", "uk", "um", "un", "us", "uy", "uz",
    "va", "vc", "ve", "vg", "vi", "vn", "vu", "wf", "ws", "xk", "xn", "xs", "xx", "xy", "xz", "ye",
    "yt", "za", "zm", "zw",
  ]
  .into_iter()
  .map(|s| s.to_string())
  .collect()
}

pub fn get_available_us_venues_codes() -> Vec<String> {
  vec![
    "us-ak", "us-al", "us-ar", "us-az", "us-ca", "us-co", "us-ct", "us-dc", "us-de", "us-fl",
    "us-ga", "us-hi", "us-ia", "us-id", "us-il", "us-in", "us-ks", "us-ky", "us-la", "us-ma",
    "us-md", "us-me", "us-mi", "us-mn", "us-mo", "us-ms", "us-mt", "us-nc", "us-nd", "us-ne",
    "us-nh", "us-nj", "us-nm", "us-nv", "us-ny", "us-oh", "us-ok", "us-or", "us-pa", "us-pr",
    "us-ri", "us-sc", "us-sd", "us-tn", "us-tx", "us-ut", "us-va", "us-vt", "us-wa", "us-wi",
    "us-wv", "us-wy",
  ]
  .into_iter()
  .map(|s| s.to_string())
  .collect()
}
