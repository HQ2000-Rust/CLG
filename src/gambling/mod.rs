mod coinflip;
pub mod gamble_result;

use coinflip::coinflip;
use rand::random;
pub fn rand_gamble_with_difficulty(difficulty: u8) -> GambleResult {
    match random::<u32>() % 1 {
        0 => coinflip(difficulty),
        _ => unreachable!(),
    }
}

