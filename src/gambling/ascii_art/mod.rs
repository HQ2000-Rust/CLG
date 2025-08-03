pub mod numbers {
    //all made with http://patorjk.com/software/taag/
    //HUGE thanks!

    pub const ZERO: &str = include_str!("./numbers/zero.txt");
    pub const ONE: &str = include_str!("./numbers/one.txt");
    pub const TWO: &str = include_str!("./numbers/two.txt");
    pub const THREE: &str = include_str!("./numbers/three.txt");
    pub const FOUR: &str = include_str!("./numbers/four.txt");
    pub const FIVE: &str = include_str!("./numbers/five.txt");
    pub const SIX: &str = include_str!("./numbers/six.txt");
    pub const SEVEN: &str = include_str!("./numbers/seven.txt");
    pub const EIGHT: &str = include_str!("./numbers/eight.txt");
    pub const NINE: &str = include_str!("./numbers/nine.txt");
    pub const UNKNOWN: &str = include_str!("./numbers/unknown.txt");
    //const for fun :)
    pub const fn digit_to_ascii(digit: u8) -> &'static str {
        match digit {
            0 => ZERO,
            1 => ONE,
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
            _ => UNKNOWN,
        }
    }
}

pub mod coin {
    pub const FRONT: &str= include_str!("./coin/front.txt");
    pub const BACK: &str= include_str!("./coin/back.txt");
    pub const SIDE: &str= include_str!("./coin/side.txt");
}