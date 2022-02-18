use std::error::Error;

use crate::arcana::iidx::{Profile, get_profile, get_profile_using_id};

async fn get_profiles(
    version: u32,
    param: &str,
) -> Result<Vec<Profile>, Box<dyn Error + Send + Sync>> {
    let dj_name_profiles = get_profile(version, param).await?;

    Ok(if !dj_name_profiles.is_empty() {
        dj_name_profiles
    } else {
        get_profile_using_id(version, param).await?
    })
}

pub mod profile;
pub mod music;
pub mod recent;

pub use profile::profile;
pub use music::music;
pub use recent::recent;