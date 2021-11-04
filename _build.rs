use std::fs;

const HEADER: &str = "#![doc(html_logo_url = \"https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg\", html_favicon_url = \"https://github.com/lucbf/icomoon_font_icons/raw/master/IcoMoon.svg\")]

//! The Icomoon Font Icons are [redistributed] by [Kordamp] with the [Apache 2.0]
//! license. Please, consider supporting its original creators by purchasing it at
//! [icomoon.io].
//! 
//! This crate contains the [Kordamp] version of IcoMoon, but is under the MIT license,
//! therefore the [Apache 2.0] license only applies to its redistributed font files.
//! 
//! [The IcoMoon Font Icons]: https://kordamp.org/ikonli/cheat-sheet-icomoon.html
//! [icomoon.io]: https://icomoon.io/#icons-icomoon
//! [redistributed]: https://github.com/kordamp/ikonli
//! [Kordamp]: https://kordamp.org/
//! [GPL]: https://www.gnu.org/licenses/gpl-3.0.html
//! [CC-4.0]: https://creativecommons.org/licenses/by/4.0/legalcode
//! [Apache 2.0]: https://www.apache.org/licenses/LICENSE-2.0

#![no_std]

///
/// **[The icons]**. An appropriate font file format is meant to be used together with [IcoMoon].
/// The correct format naturally depends on the program being built or the target os.
/// 
/// [The icons]: https://kordamp.org/ikonli/cheat-sheet-icomoon.html
pub mod font_file {
\tuse core::include_bytes;

/// .ttf font
\tpub static TTF: &[u8] = include_bytes!(\"icomoon.ttf\");

/// .eot font 
\tpub static EOT: &[u8] = include_bytes!(\"icomoon.eot\");

/// .woff font
\tpub static WOFF: &[u8] = include_bytes!(\"icomoon.woff\");
}

/// Represents a char. The conversion is done using `char::from()`.
/// 
/// Examples:
/// ```
/// use icomoon_font_icons::IcoMoon;
/// 
/// let i: char = char::from(IcoMoon::Home);
/// ```
/// 
/// <style type=\"text/css\">
/// @font-face {
/// font-family: 'icomoon';
/// src: url('https://kordamp.org/ikonli/fonts/icomoon.eot?hy0xsg');
/// src: url('https://kordamp.org/ikonli/fonts/icomoon.eot?hy0xsg#iefix') format('embedded-opentype'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.ttf?hy0xsg') format('truetype'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.woff?hy0xsg') format('woff'),
/// url('https://kordamp.org/ikonli/fonts/icomoon.svg?hy0xsg#icomoon') format('svg');
/// font-weight: normal;
/// font-style: normal;
/// }
///
/// .icomoon {
/// font-family: 'icomoon' !important;
/// font-size: 28px;
/// color: #212121;
/// }
/// </style>
/// 
/// You can also click in a button below to copy the character to the clipboard,
/// but this is not recommended because your code becomes less transparent to the
/// reader.
/// ```
/// let i: char = '\\u{e900}';//IcoMoon::Home
/// ```
#[repr(C)]
pub enum IcoMoon{
";


const FOOTER: &str = "}

impl From<IcoMoon> for char{
    fn from(icon: IcoMoon) -> char {
        unsafe{char::from_u32_unchecked(icon as u32)}
    }
}
";

struct Find<'a> {
    str: &'a str,
    pos: usize
}

fn enum_field_from_line(line: &str) -> (String, String) {
    let mut icm_ = Find{
        str: "ICM_",
        pos: 0
    };

    let mut to_lowercase = false;
    let mut past_open = false;
    let mut past_single_quote = false;

    let mut field_name = String::new();
    let mut field_char = String::new();

    for char in line.chars() {
        if icm_.pos < icm_.str.len() {
            match icm_.str.find(char) {
                None => {
                    icm_.pos = 0;
                }
                Some(pos) => {
                    if pos == icm_.pos {
                        icm_.pos += 1;
                    } else {
                        icm_.pos = 0;
                    }
                }
            }
        } else if !past_open{
            if char == '(' {
                past_open = true;
            } else {
                if to_lowercase {
                    if char == '_' {
                        to_lowercase = false;
                    } else {
                        field_name.push((char as u8 + 32) as char);
                    }
                } else {
                    field_name.push(char);
                    to_lowercase = true;
                }
            }
        } else if !past_single_quote{
            if char == '\'' {
                past_single_quote = true;
            } else {
                continue;
            }
        } else if char == '\''{
            break;
        } else {
            field_char.push(char);
        }
    }

    (field_name, field_char)
}

fn main() {
    let parse = fs::read_to_string("src/icons.txt").unwrap();
    let mut module = String::from(HEADER);

    for line in parse.split('\n') {
        let lr = enum_field_from_line(line);

        let unicode_str = lr.1.split_at(2);
        let unicode = u32::from_str_radix(unicode_str.1, 16).unwrap();
        

        //rustdoc
        module += "/// <button class='icomoon' id='\\u{";
        module += unicode_str.1;
        module += "}' onclick='navigator.clipboard.writeText(this.id)'>";
        module.push(unsafe{char::from_u32_unchecked(unicode)});
        module+= "</button>";
        module.push('\n');

        //rust
        module.push('\t');
        module += lr.0.as_str();
        module += " = ";
        module += unicode.to_string().as_str();
        module += ",\n";
    }

    module += FOOTER;

    fs::write("src/lib.rs", module).unwrap();
}