use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn wc(
    files: Vec<PathBuf>,
    mut newlines: bool,
    mut words: bool,
    mut bytes: bool,
) -> std::io::Result<Option<String>> {
    if files.is_empty() {
        return Ok(None);
    }
    if !newlines && !words && !bytes {
        newlines = true;
        words = true;
        bytes = true;
    }
    let mut result = String::new();
    let mut total = CountResult::init(newlines, words, bytes);
    for path in files {
        let file = File::open(path.clone())?;
        let counts = count(file, bytes, newlines, words)?;
        result.push_str(format!("{}\t{}\n",
                                counts,
                                path.file_name()
                                    .expect("Fie at this path just got opened, so it would've thrown an error before if it was an invalid file")
                                    .to_string_lossy()
            ).as_str()
        );
        total.add(&counts);
    }
    result.push_str(format!("{}\t{}\n", total, "total").as_str());
    Ok(Some(result))
}

#[derive(Debug)]
struct CountResult {
    pub bytes: Option<u32>,
    pub newlines: Option<u32>,
    pub words: Option<u32>,
}

impl CountResult {
    fn init(newlines: bool, words: bool, bytes: bool) -> CountResult {
        //just an ergonomic
        #[inline]
        fn switch_option(switch: bool) -> Option<u32> {
            if switch { Some(0) } else { None }
        }
        Self {
            bytes: switch_option(bytes),
            newlines: switch_option(newlines),
            words: switch_option(words),
        }
    }
    fn add_word(&mut self) {
        self.words = match self.words {
            Some(x) => Some(x + 1),
            None => None,
        };
    }
    fn add_newline(&mut self) {
        self.newlines = match self.newlines {
            Some(x) => Some(x + 1),
            None => None,
        };
    }
    fn add(&mut self, other: &CountResult) {
        //another ergonomic
        #[inline]
        fn sum_option(one: Option<u32>, other: Option<u32>) -> Option<u32> {
            Some(
                one.expect("Flags are everywhere the same every execution")
                    + other.expect("Flags are everywhere the same every execution"),
            )
        }
        *self = Self {
            newlines: sum_option(self.newlines, other.newlines),
            words: sum_option(self.words, other.words),
            bytes: sum_option(self.bytes, other.bytes),
        };
    }
}

impl Display for CountResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //...
        #[inline]
        fn optional(option: Option<u32>) -> String {
            option
                .and_then(|count| Some(count.to_string()))
                .unwrap_or("".to_string())
        }
        //I'm also not proud of this, will change it later maybe
        write!(
            fmt,
            "{}\t{}\t{}",
            optional(self.newlines),
            optional(self.words),
            optional(self.bytes),
        )
    }
}

fn count(mut file: File, bytes: bool, newlines: bool, words: bool) -> std::io::Result<CountResult> {
    let mut result = CountResult::init(bytes, newlines, words);
    let mut contents = Vec::new();
    //I know this is a crime for big files, but this project isn't around efficient, but working implementations
    file.read_to_end(&mut contents)?;
    //again, possibly lossy conversation, but at the targeted scale negligible
    result.bytes = Some(contents.len() as u32);
    //this could be any char in the array down here
    let mut last_char = '\n';
    //here again, this isn't a fully-fledged toolkit
    let mut string = String::from_utf8(contents).expect("Invalid UTF-8");
    //adding a whitespace at the end for the word counting algo
    string.push(' ');
    let chars = string.chars();
    for char in chars {
        if char == '\n' {
            result.add_newline();
        }
        //sorry, that's my (irrational) perfectionism
        if [' ', '\n'].contains(&char)
            && ![
                '\n', '\r', '\t', ' ', '.', ':', ',', '.', ';', '!', '\"', '§', '%', '&', '/', '(',
                ')', '=', '?', '\\', '{', '}', '[', ']', '>', '<', '|', '°', '^', '-', '_', '@',
                '€', '*', '+', '~', '\'', '#',
            ]
            .contains(&last_char)
        {
            result.add_word()
        }
        last_char = char;
    }

    Ok(result)
}
