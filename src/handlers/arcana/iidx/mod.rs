use std::error::Error;

use crate::arcana::iidx::{get_profile, get_profile_using_id, Profile};

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

pub mod music;
pub mod profile;
pub mod recent;

pub use music::music;
pub use profile::profile;
pub use recent::recent;
