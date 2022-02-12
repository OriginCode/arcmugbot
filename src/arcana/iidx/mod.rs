use lazy_static::lazy_static;
use reqwest::Url;

lazy_static! {
    static ref IIDX_URL: Url = Url::parse("https://arcana.nu/api/v1/iidx/").unwrap();
}

pub mod music;
pub mod profile;
