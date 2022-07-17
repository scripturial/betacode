//! A fast rust and strict library for conversion to and from betacode.
//! Includes support for TLG betacode, and standard betacode.
//!
//! # Examples
//!
//! Convert Robinson-Pierpont style betacode into unicode Greek:
//!
//! ```
//! let result = betacode2::to_greek("qeo/v", betacode2::Type::Default).unwrap();
//! assert_eq!(result, "θεός");
//! ```
//!
//! Convert TLG style betacode into unicode Greek:
//!
//! ```
//! let result = betacode2::to_greek("qeo/s", betacode2::Type::TLG).unwrap();
//! assert_eq!(result, "θεός");
//! ```
//!

use std::slice;

/// Choose which betacode format to convert.
#[derive(Copy, Clone)]
pub enum Type {
    Default = 0,
    TLG = 1,
}

/// Conversion fails when an unexpected character is found.
#[derive(Debug)]
pub enum ConversionError {
    /// Returns the invalid character, and its position in the string.
    UnexpectedCharacter(char, usize),
    /// Returns the character that has an invalid accent, and its position
    /// in the string.
    UnexpectedAccent(char, usize),
}

/// Convert a betacode ascii string into a Greek unicode string.
///
/// Space or punctuation characters should not appear at the start or end of
/// the string. Unrecognised punctuation, ascii or unicode character cause
/// an error to be returned.
///
/// # Examples
///
/// Convert Robinson-Pierpont style betacode into unicode Greek:
///
/// ```
/// let result = betacode2::to_greek("qeo/v", betacode2::Type::Default).unwrap();
/// assert_eq!(result, "θεός");
/// ```
///
/// Convert TLG style betacode into unicode Greek:
///
/// ```
/// let result = betacode2::to_greek("qeo/s", betacode2::Type::TLG).unwrap();
/// assert_eq!(result, "θεός");
/// ```
///
pub fn to_greek(input: &str, version: Type) -> Result<String, ConversionError> {
    let mut word: String = String::new();

    unsafe {
        let mut i: usize = 0;
        let mut size: usize = input.len();
        if size == 0 {
            return Ok("".to_string());
        }

        // Walk over the u8 bytes inside the str instead of
        // copying the data into a u8 array.
        let ptr = input.as_ptr();
        let text = slice::from_raw_parts(ptr, size);

        // Trim whitespace off start
        loop {
            if i == size {
                return Ok("".to_string());
            }
            if !is_ascii_whitespace(text[i]) {
                break;
            }
            i += 1;
        }

        // Ignore whitespace at the end
        loop {
            if i == size {
                return Ok("".to_string());
            }
            if !is_ascii_whitespace(text[size - 1]) {
                break;
            }
            size -= 1;
            continue;
        }

        // Read a character and any accents following it
        let mut current: char = 0 as char;
        let mut current_index: usize = 0;
        let mut accents: u16 = 0;

        loop {
            if i == size {
                break;
            }
            let c = text[i];
            if c == b'*' {
                // For now ignore asterix before letter
                i += 1;
                continue;
            }
            if c > 127 {
                // Unicode sequences should not appear
                // in ascii betacode sequences
                return Err(ConversionError::UnexpectedCharacter(c as char, i));
            }
            let l = lookup_greek_letter(c, version);
            if l != 0 as char {
                if current != 0 as char {
                    // We encountered the next letter, if we just read a previous
                    // letter, push it onto the return string.
                    let e = apply_accent(current, accents);
                    if e > 0 as char {
                        word.push(e)
                    } else {
                        return Err(ConversionError::UnexpectedAccent(
                            current as char,
                            current_index,
                        ));
                    }
                }
                // The start of a letter sequence
                current = l;
                current_index = i;
                accents = 0;
                i += 1;
                continue;
            }
            if is_ascii_whitespace(c) {
                break;
            }
            let valid = is_valid_betacode_symbol(c);
            if valid > 0 {
                if current == 0 as char {
                    // We see a betacode accent character, but
                    // not a greek letter just before it.
                    return Err(ConversionError::UnexpectedCharacter(
                        c as char,
                        current_index,
                    ));
                }
                accents = accents | valid;
                i += 1;
                continue;
            }
            // This character is not an alphabetic letter, not a
            // whitespace, and not a valid betacode symbol.
            break;
        }

        // When the end of string is reached, a final character
        // may be waiting to be pushed onto the result string.
        if current != 0 as char {
            println!("apply accent {} {} {}", word, current, accents);
            let e = apply_accent(current, accents);
            if accents == 0 && current == 'σ' {
                word.push('ς')
            } else if e > 0 as char {
                word.push(e)
            } else {
                return Err(ConversionError::UnexpectedAccent(
                    current as char,
                    current_index,
                ));
            }
        }

        if i < size && text[i] == b'\'' {
            word.push('᾽');
            i += 1
        }

        loop {
            if i == size {
                break;
            }
            if is_ascii_whitespace(text[i]) {
                i += 1;
                continue;
            }
            // Unexpected character
            return Err(ConversionError::UnexpectedCharacter(current as char, i));
        }
    }
    Ok(word)
}

// test if a character is a valid accentuation for a greek character.
//
// See: https://stephanus.tlg.uci.edu/encoding/BCM.pdf
fn is_valid_betacode_symbol(c: u8) -> u16 {
    match c {
        b'/' => ASCII_ACUTE,
        b'\\' => ASCII_GRAVE,
        b'(' => ASCII_ROUGH,
        b')' => ASCII_SMOOTH,
        b'|' => ASCII_IOTA,
        b'+' => ASCII_DIAERESIS,
        b'=' => ASCII_CIRCUMFLEX,
        b'^' => ASCII_CIRCUMFLEX,
        b'1' => ASCII_SIGMA1,
        b'2' => ASCII_SIGMA2,
        b'3' => ASCII_SIGMA3,
        _ => 0,
    }
}

const ASCII_ACUTE: u16 = 0x1;
const ASCII_GRAVE: u16 = 0x2;
const ASCII_CIRCUMFLEX: u16 = 0x4;
const ASCII_DIAERESIS: u16 = 0x8;
const ASCII_ROUGH: u16 = 0x10;
const ASCII_SMOOTH: u16 = 0x20;
const ASCII_IOTA: u16 = 0x40;
const ASCII_SIGMA1: u16 = 0x80;
const ASCII_SIGMA2: u16 = 0x100;
const ASCII_SIGMA3: u16 = 0x200;

const ASCII_SMOOTH_ACUTE: u16 = ASCII_SMOOTH + ASCII_ACUTE;
const ASCII_SMOOTH_GRAVE: u16 = ASCII_SMOOTH + ASCII_GRAVE;
const ASCII_ROUGH_ACUTE: u16 = ASCII_ROUGH + ASCII_ACUTE;
const ASCII_ROUGH_GRAVE: u16 = ASCII_ROUGH + ASCII_GRAVE;
const ASCII_CIRCUMFLEX_ROUGH: u16 = ASCII_ROUGH + ASCII_CIRCUMFLEX;
const ASCII_CIRCUMFLEX_SMOOTH: u16 = ASCII_SMOOTH + ASCII_CIRCUMFLEX;
const ASCII_DIAERESIS_ACUTE: u16 = ASCII_DIAERESIS + ASCII_ACUTE;
const ASCII_DIAERESIS_GRAVE: u16 = ASCII_DIAERESIS + ASCII_GRAVE;

fn is_ascii_whitespace(c: u8) -> bool {
    if c == b' ' || c == b'\r' || c == b'\n' || c == b'\t' || c == 0 {
        return true;
    }
    return false;
}

fn lookup_greek_letter(c: u8, version: Type) -> char {
    let o = match c {
        b'a' => 'α',
        b'b' => 'β',
        b'd' => 'δ',
        b'e' => 'ε',
        b'f' => 'φ',
        b'g' => 'γ',
        b'h' => 'η',
        b'i' => 'ι',
        b'k' => 'κ',
        b'l' => 'λ',
        b'm' => 'μ',
        b'n' => 'ν',
        b'o' => 'ο',
        b'p' => 'π',
        b'q' => 'θ',
        b'r' => 'ρ',
        b's' => 'σ',
        b't' => 'γ',
        b'u' => 'υ',
        b'w' => 'ω',
        b'y' => 'ψ',
        b'z' => 'ζ',
        b'A' => 'α',
        b'B' => 'Β',
        b'D' => 'Δ',
        b'E' => 'Ε',
        b'F' => 'Φ',
        b'G' => 'Γ',
        b'H' => 'Η',
        b'I' => 'Ι',
        b'K' => 'Κ',
        b'L' => 'Λ',
        b'M' => 'Μ',
        b'N' => 'Ν',
        b'O' => 'Ο',
        b'Q' => 'Θ',
        b'R' => 'Ρ',
        b'S' => 'Σ',
        b'T' => 'Γ',
        b'U' => 'Υ',
        b'W' => 'Ω',
        b'Y' => 'Ψ',
        b'Z' => 'Ζ',
        _ => 0 as char,
    };
    if o != 0 as char {
        return o;
    }

    match version {
        // Who uses these mpapings
        Type::Default => {
            let o = match c {
                b'v' => 'σ',
                b'V' => 'Σ',
                b'j' => 'ς', // Some betacode systems use j for final sigma
                b'J' => 'Σ', // Some betacode systems use j for final sigma
                b'c' => 'χ',
                b'C' => 'χ',
                _ => 0 as char,
            };
            if o != 0 as char {
                return o;
            }
        }
        Type::TLG => {
            let o = match c {
                b'v' => 'ϝ',
                b'V' => 'Ϝ',
                b'c' => 'ξ',
                b'C' => 'Ξ',
                b'x' => 'χ',
                b'X' => 'Χ',
                _ => 0 as char,
            };
            if o != 0 as char {
                return o;
            }
        }
    }

    0 as char
}

fn apply_accent(c: char, accents: u16) -> char {
    if accents == 0 {
        return c;
    }

    match (c, accents) {
        ('α', ASCII_SMOOTH) => 'ἀ',
        ('ε', ASCII_SMOOTH) => 'ἐ',
        ('ι', ASCII_SMOOTH) => 'ἰ',
        ('η', ASCII_SMOOTH) => 'ἠ',
        ('o', ASCII_SMOOTH) => 'ὀ',
        ('ω', ASCII_SMOOTH) => 'ὠ',
        ('υ', ASCII_SMOOTH) => 'ὐ',
        ('Α', ASCII_SMOOTH) => 'Ἀ',
        ('Ε', ASCII_SMOOTH) => 'Ἐ',
        ('Ι', ASCII_SMOOTH) => 'Ἰ',
        ('Η', ASCII_SMOOTH) => 'Ἠ',
        ('O', ASCII_SMOOTH) => 'Ὀ',
        ('Ω', ASCII_SMOOTH) => 'Ὠ',
        ('Υ', ASCII_SMOOTH) => 'ὐ',
        ('α', ASCII_ROUGH) => 'ἁ',
        ('ε', ASCII_ROUGH) => 'ἑ',
        ('ι', ASCII_ROUGH) => 'ἱ',
        ('η', ASCII_ROUGH) => 'ἡ',
        ('o', ASCII_ROUGH) => 'ὁ',
        ('ω', ASCII_ROUGH) => 'ὡ',
        ('υ', ASCII_ROUGH) => 'ὑ',
        ('ρ', ASCII_ROUGH) => 'ῥ',
        ('Α', ASCII_ROUGH) => 'Ἁ',
        ('Ε', ASCII_ROUGH) => 'Ἑ',
        ('Ι', ASCII_ROUGH) => 'Ἱ',
        ('Η', ASCII_ROUGH) => 'Ἡ',
        ('O', ASCII_ROUGH) => 'Ὁ',
        ('Ω', ASCII_ROUGH) => 'Ὡ',
        ('Υ', ASCII_ROUGH) => 'Ὑ',
        ('Ρ', ASCII_ROUGH) => 'Ῥ',
        ('α', ASCII_ACUTE) => 'ά',
        ('ε', ASCII_ACUTE) => 'έ',
        ('ι', ASCII_ACUTE) => 'ί',
        ('η', ASCII_ACUTE) => 'ή',
        ('ο', ASCII_ACUTE) => 'ό',
        ('ω', ASCII_ACUTE) => 'ώ',
        ('υ', ASCII_ACUTE) => 'ύ',
        ('Α', ASCII_ACUTE) => 'Ά',
        ('Ε', ASCII_ACUTE) => 'Έ',
        ('Ι', ASCII_ACUTE) => 'Ί',
        ('Η', ASCII_ACUTE) => 'Ή',
        ('O', ASCII_ACUTE) => 'Ό',
        ('Ω', ASCII_ACUTE) => 'Ώ',
        ('Υ', ASCII_ACUTE) => 'Ύ',
        ('α', ASCII_GRAVE) => 'ὰ',
        ('ε', ASCII_GRAVE) => 'ὲ',
        ('ι', ASCII_GRAVE) => 'ὶ',
        ('η', ASCII_GRAVE) => 'ὴ',
        ('o', ASCII_GRAVE) => 'ὸ',
        ('ω', ASCII_GRAVE) => 'ὼ',
        ('υ', ASCII_GRAVE) => 'ὺ',
        ('Α', ASCII_GRAVE) => 'Ὰ',
        ('Ε', ASCII_GRAVE) => 'Ὲ',
        ('Ι', ASCII_GRAVE) => 'Ὶ',
        ('Η', ASCII_GRAVE) => 'Ὴ',
        ('O', ASCII_GRAVE) => 'Ὸ',
        ('Ω', ASCII_GRAVE) => 'Ὼ',
        ('Υ', ASCII_GRAVE) => 'Ὺ',
        ('α', ASCII_CIRCUMFLEX) => 'ᾶ',
        ('ι', ASCII_CIRCUMFLEX) => 'ῖ',
        ('η', ASCII_CIRCUMFLEX) => 'ῆ',
        ('ω', ASCII_CIRCUMFLEX) => 'ῶ',
        ('υ', ASCII_CIRCUMFLEX) => 'ῦ',
        ('α', ASCII_IOTA) => 'ᾳ',
        ('η', ASCII_IOTA) => 'ῃ',
        ('ω', ASCII_IOTA) => 'ῳ',
        ('α', ASCII_SMOOTH_GRAVE) => 'ἂ',
        ('ε', ASCII_SMOOTH_GRAVE) => 'ἔ',
        ('ι', ASCII_SMOOTH_GRAVE) => 'ἲ',
        ('η', ASCII_SMOOTH_GRAVE) => 'ἢ',
        ('o', ASCII_SMOOTH_GRAVE) => 'ὂ',
        ('ω', ASCII_SMOOTH_GRAVE) => 'ὢ',
        ('υ', ASCII_SMOOTH_GRAVE) => 'ὒ',
        ('Α', ASCII_SMOOTH_GRAVE) => 'Ἂ',
        ('Ε', ASCII_SMOOTH_GRAVE) => 'Ἒ',
        ('Ι', ASCII_SMOOTH_GRAVE) => 'Ἲ',
        ('Η', ASCII_SMOOTH_GRAVE) => 'Ἢ',
        ('O', ASCII_SMOOTH_GRAVE) => 'Ὂ',
        ('Ω', ASCII_SMOOTH_GRAVE) => 'Ὤ',
        //('Υ', ASCII_SMOOTH_GRAVE) => '῍Υ', // Not possible to type on OS/X
        ('α', ASCII_ROUGH_GRAVE) => 'ἃ',
        ('ε', ASCII_ROUGH_GRAVE) => 'ἓ',
        ('ι', ASCII_ROUGH_GRAVE) => 'ἳ',
        ('η', ASCII_ROUGH_GRAVE) => 'ἣ',
        ('o', ASCII_ROUGH_GRAVE) => 'ὃ',
        ('ω', ASCII_ROUGH_GRAVE) => 'ὣ',
        ('υ', ASCII_ROUGH_GRAVE) => 'ὓ',
        ('Α', ASCII_ROUGH_GRAVE) => 'Ἃ',
        ('Ε', ASCII_ROUGH_GRAVE) => 'Ἒ',
        ('Ι', ASCII_ROUGH_GRAVE) => 'Ἳ',
        ('Η', ASCII_ROUGH_GRAVE) => 'Ἣ',
        ('O', ASCII_ROUGH_GRAVE) => 'Ὃ',
        ('Ω', ASCII_ROUGH_GRAVE) => 'Ὣ',
        ('Υ', ASCII_ROUGH_GRAVE) => 'Ὓ',
        ('α', ASCII_SMOOTH_ACUTE) => 'ἄ',
        ('ε', ASCII_SMOOTH_ACUTE) => 'ἔ',
        ('ι', ASCII_SMOOTH_ACUTE) => 'ἴ',
        ('η', ASCII_SMOOTH_ACUTE) => 'ἤ',
        ('o', ASCII_SMOOTH_ACUTE) => 'ὄ',
        ('ω', ASCII_SMOOTH_ACUTE) => 'ὤ',
        ('υ', ASCII_SMOOTH_ACUTE) => 'ὔ',
        ('Α', ASCII_SMOOTH_ACUTE) => 'Ἄ',
        ('Ε', ASCII_SMOOTH_ACUTE) => 'Ἔ',
        ('Ι', ASCII_SMOOTH_ACUTE) => 'Ἴ',
        ('Η', ASCII_SMOOTH_ACUTE) => 'Ἤ',
        ('O', ASCII_SMOOTH_ACUTE) => 'Ὄ',
        ('Ω', ASCII_SMOOTH_ACUTE) => 'Ὤ',
        //('Υ', ASCII_SMOOTH_ACUTE) => '῎Υ', // Seems not possible to compose
        ('α', ASCII_ROUGH_ACUTE) => 'ἅ',
        ('ε', ASCII_ROUGH_ACUTE) => 'ἕ',
        ('ι', ASCII_ROUGH_ACUTE) => 'ἵ',
        ('η', ASCII_ROUGH_ACUTE) => 'ἥ',
        ('o', ASCII_ROUGH_ACUTE) => 'ὅ',
        ('ω', ASCII_ROUGH_ACUTE) => 'ὥ',
        ('υ', ASCII_ROUGH_ACUTE) => 'ὕ',
        ('Α', ASCII_ROUGH_ACUTE) => 'Ἅ',
        ('Ε', ASCII_ROUGH_ACUTE) => 'Ἕ',
        ('Ι', ASCII_ROUGH_ACUTE) => 'Ἵ',
        ('Η', ASCII_ROUGH_ACUTE) => 'Ἥ',
        ('O', ASCII_ROUGH_ACUTE) => 'Ὅ',
        ('Ω', ASCII_ROUGH_ACUTE) => 'Ὥ',
        ('Υ', ASCII_ROUGH_ACUTE) => 'Ὕ',
        ('ι', ASCII_DIAERESIS) => 'ϊ',
        ('υ', ASCII_DIAERESIS) => 'ϋ',
        ('Ι', ASCII_DIAERESIS) => 'Ϊ',
        ('Υ', ASCII_DIAERESIS) => 'Ϋ',
        ('ι', ASCII_DIAERESIS_GRAVE) => 'ῒ',
        ('υ', ASCII_DIAERESIS_GRAVE) => 'ῢ',
        ('Ι', ASCII_DIAERESIS_GRAVE) => 'ῒ',
        ('Υ', ASCII_DIAERESIS_GRAVE) => 'ῢ',
        ('ι', ASCII_DIAERESIS_ACUTE) => 'ΐ',
        ('υ', ASCII_DIAERESIS_ACUTE) => 'ΰ',
        ('Ι', ASCII_DIAERESIS_ACUTE) => 'ΐ',
        ('Υ', ASCII_DIAERESIS_ACUTE) => 'ΰ',
        ('α', ASCII_CIRCUMFLEX_SMOOTH) => 'ἆ',
        ('η', ASCII_CIRCUMFLEX_SMOOTH) => 'ἦ',
        ('ι', ASCII_CIRCUMFLEX_SMOOTH) => 'ἶ',
        ('ω', ASCII_CIRCUMFLEX_SMOOTH) => 'ὦ',
        ('υ', ASCII_CIRCUMFLEX_SMOOTH) => 'ὖ',
        ('Α', ASCII_CIRCUMFLEX_SMOOTH) => 'Ἆ',
        ('Η', ASCII_CIRCUMFLEX_SMOOTH) => 'Ἦ',
        ('Ι', ASCII_CIRCUMFLEX_SMOOTH) => 'Ἶ',
        ('Ω', ASCII_CIRCUMFLEX_SMOOTH) => 'Ὦ',
        ('Υ', ASCII_CIRCUMFLEX_SMOOTH) => 'ὖ',
        ('α', ASCII_CIRCUMFLEX_ROUGH) => 'ἇ',
        ('η', ASCII_CIRCUMFLEX_ROUGH) => 'ἧ',
        ('ι', ASCII_CIRCUMFLEX_ROUGH) => 'ἷ',
        ('ω', ASCII_CIRCUMFLEX_ROUGH) => 'ὧ',
        ('υ', ASCII_CIRCUMFLEX_ROUGH) => 'ὗ',
        ('Α', ASCII_CIRCUMFLEX_ROUGH) => 'Ἇ',
        ('Η', ASCII_CIRCUMFLEX_ROUGH) => 'Ἧ',
        ('Ι', ASCII_CIRCUMFLEX_ROUGH) => 'Ἷ',
        ('Ω', ASCII_CIRCUMFLEX_ROUGH) => 'Ὧ',
        ('Υ', ASCII_CIRCUMFLEX_ROUGH) => 'Ὗ',
        ('σ', ASCII_SIGMA1) => 'σ',
        ('σ', ASCII_SIGMA2) => 'ς',
        ('σ', ASCII_SIGMA3) => 'ϲ',
        ('Σ', ASCII_SIGMA1) => 'Σ',
        ('Σ', ASCII_SIGMA2) => 'Σ',
        ('Σ', ASCII_SIGMA3) => 'Ϲ',
        (_, _) => 0 as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_default_encoding() {

    assert_eq!(to_greek("", Type::Default).unwrap(), "");
        assert_eq!(to_greek(" ", Type::Default).unwrap(), "");
        assert_eq!(to_greek("  ", Type::Default).unwrap(), "");
        assert_eq!(to_greek("a", Type::Default).unwrap(), "α");
        assert_eq!(to_greek("a)", Type::Default).unwrap(), "ἀ");
        assert_eq!(to_greek("s", Type::Default).unwrap(), "ς");
        assert_eq!(to_greek("es", Type::Default).unwrap(), "ες");
        assert_eq!(to_greek("es1", Type::Default).unwrap(), "εσ");
        assert_eq!(to_greek("es2", Type::Default).unwrap(), "ες");
        assert_eq!(to_greek("es3", Type::Default).unwrap(), "εϲ");
        assert_eq!(to_greek("sos", Type::Default).unwrap(), "σος");
        assert_eq!(to_greek("a)bba", Type::Default).unwrap(), "ἀββα");
        assert_eq!(to_greek("a)p'", Type::Default).unwrap(), "ἀπ᾽");
        assert_eq!(to_greek(" d' ", Type::Default).unwrap(), "δ᾽");
        assert_eq!(to_greek(" a(ll", Type::Default).unwrap(), "ἁλλ");
        assert_eq!(to_greek("kai\\ ", Type::Default).unwrap(), "καὶ");
        assert_eq!(to_greek("cri", Type::Default).unwrap(), "χρι");
        assert_eq!(to_greek("criv", Type::Default).unwrap(), "χρις");
        assert_eq!(to_greek("qeo/v", Type::Default).unwrap(), "θεός");
        assert_eq!(to_greek("qeo/s3", Type::Default).unwrap(), "θεόϲ");
    }

    #[test]
    fn invalid_default_encoding() {
        assert!(to_greek("a\\b'a", Type::Default).is_err());
        assert!(to_greek("dε", Type::Default).is_err());
        assert!(to_greek("dε ", Type::Default).is_err());
        assert!(to_greek(" dε", Type::Default).is_err());
        assert!(to_greek(")a", Type::Default).is_err());
        assert!(to_greek("(a", Type::Default).is_err());
        assert!(to_greek("\\a", Type::Default).is_err());
        assert!(to_greek("xri", Type::Default).is_err());
    }

    #[test]
    fn valid_tlg_encoding() {
        assert_eq!(to_greek("qeo/s", Type::TLG).unwrap(), "θεός");
        assert_eq!(to_greek("xri", Type::TLG).unwrap(), "χρι");
        assert_eq!(to_greek("qeo/s1", Type::TLG).unwrap(), "θεόσ");
        assert_eq!(to_greek("qeo/s2", Type::TLG).unwrap(), "θεός");
        assert_eq!(to_greek("qeo/s3", Type::TLG).unwrap(), "θεόϲ");
    }
}

#[test]
fn invalid_tlg_encoding() {
    assert!(to_greek("a\\b'a", Type::TLG).is_err());
    assert!(to_greek("dε", Type::TLG).is_err());
}
