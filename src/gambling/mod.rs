mod ascii_art;
mod coinflip;
mod utils;

pub use utils::GambleResult;

use coinflip::coinflip;
use fastrand;
pub fn rand_gamble_with_difficulty(difficulty: u8) -> std::io::Result<GambleResult> {
    Ok(match fastrand::u8(..1) {
        0 => coinflip(difficulty)?,
        _ => unreachable!(),
    })
}
