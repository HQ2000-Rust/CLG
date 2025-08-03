mod coinflip;
mod gamble_result;
mod ascii_art;

pub use gamble_result::GambleResult;

use coinflip::coinflip;
use rand::random;
pub fn rand_gamble_with_difficulty(difficulty: u8) -> GambleResult {
    match random::<u32>() % 1 {
        0 => coinflip(difficulty),
        _ => unreachable!(),
    }
}
