pub mod numbers {
    //all made with http://patorjk.com/software/taag/
    //HUGE thanks!

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

    pub const HEIGHT: usize = 7;

    pub fn take_from_bottom(str: &str, number: usize) -> String {
        let mut result = String::new();
        str.lines()
            .into_iter()
            .skip(HEIGHT.saturating_sub(number + 1))
            .for_each(|line| result.push_str([line, "\n"].join("").as_ref()));
        result
    }

    pub fn take_from_top(str: &str, number: usize) -> String {
        let mut result = String::new();
        str.lines()
            .into_iter()
            .take(number)
            .for_each(|line| result.push_str([line, "\n"].join("").as_ref()));
        result
    }

    //take_from_below!!!!!
    //take_from_top!!!!!

    //const for fun :)
    pub const fn digit_to_ascii(digit: usize) -> &'static str {
        match digit {
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
    //pub const SIDE: &[u8] = include_bytes!("coin/side2.ansi");
    // grey "\x1b[48;2;118;118;118m"

    pub const SIDE: &str = concat!(
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        "                                            \n",
        // "                                            \n"
        concat!(
            "\x1b[38;2;118;118;118m",
            "▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄",
            "\x1b[0m",
            "\n"
        ),
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        concat!(
            "\x1b[48;2;249;241;165m",
            "                                            ",
            "\x1b[0m"
        ),
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        "\n",
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        concat!(
            "\x1b[48;2;249;241;165m",
            "                                            ",
            "\x1b[0m"
        ),
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        "\n",
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        concat!(
            "\x1b[48;2;249;241;165m",
            "                                            ",
            "\x1b[0m"
        ),
        concat!("\x1b[48;2;118;118;118m", " ", "\x1b[0m"),
        "\n",
        concat!(
            "\x1b[38;2;118;118;118m",
            "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀",
            "\x1b[0m",
            "\n"
        ),
    );
    pub const HEAD: &[u8] = include_bytes!("coin/head.txt");
    pub const TAILS: &[u8] = include_bytes!("coin/tails.txt");
}
