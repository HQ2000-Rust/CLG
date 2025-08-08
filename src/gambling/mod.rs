pub mod ascii_art;
mod coinflip;
pub mod machine;
mod utils;

pub use utils::GambleResult;

use crate::gambling::machine::machine;
use coinflip::coinflip;
use fastrand;

pub fn rand_gamble() -> std::io::Result<GambleResult> {
    Ok(match fastrand::bool() {
        true => coinflip()?,
        false => machine()?,
    })
}
