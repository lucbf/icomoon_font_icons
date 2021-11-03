use std::fs;

const HEADER: &str = "#![no_std]
use core::include_bytes;
//! [`The IcoMoon-Free Font Icons`]
//! 
//! 
//! \tThis crate is a redistribution of the IcoMoon TTF Font Icon,
//! It is thus under the same free license.
//! 
//! \tThe IcoMoon font is distributed through the [`GPL`] / [`CC-4.0`] License,
//! which can also be obtained at [`icomoon.io`]. There you can also find the 
//! icons in vector and image formats.
//! [`The IcoMoon-Free Font Icons`]: https://kordamp.org/ikonli/cheat-sheet-icomoon.html
//! [`icomoon.io]: https://icomoon.io/#icons-icomoon
//! [`GPL`]: https://www.gnu.org/licenses/gpl-3.0.html
//! [`CC-4.0`]: https://creativecommons.org/licenses/by/4.0/legalcode



/// 
/// The font file as an array of bytes. It is meant to be used together with [IcoMoon]
static ICOMOON_TTF_FONT: &[u8] = include_bytes!(\"IcoMoon-Free.ttf\");

/// Represents a char.
/// 
/// Examples:
/// ```
/// let i = IcoMoon.HOME as char;
/// ```
/// ```
/// let i = char.from(IcoMoon.HOME);
/// ```
enum IcoMoon{
";


const FOOTER: &str = "};

impl from<IcoMoon> for char{
    fn from(icon: IcoMoon) -> char {
        icon
    }
}
";
fn main() {
    let parse = fs::read_to_string("src/icons.txt").unwrap();
    let mut module = String::from(HEADER);

    for line in parse.split('\n') {
        let lr = line.split_at(line.find(' ').unwrap());

        module.push('\t');
        module += lr.0.to_uppercase().as_str();
        module += " = ";
        module += lr.1;
        module += ",\n";
    }

    module += FOOTER;

    fs::write("src/lib.rs", module).unwrap();
}