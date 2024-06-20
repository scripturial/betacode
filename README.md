A fast rust library for conversion to and from betacode.
Includes support for TLG betacode, and standard betacode.

# Examples

Convert Robinson-Pierpont style betacode into unicode Greek:

    use betacode2::{Betacode, Type::Default};
    let word = "Qeo/v".to_greek(Default).unwrap();


Convert TLG style betacode into unicode Greek:

    use betacode2::{Betacode, Type::TLG};
    let word = "*qeo/s".to_greek(TLG).unwrap();

The default converter assumes lowercase ascii letters are lowercase Greek
letters and uppercase ascii letters are uppercase Greek letters. The TLG
converter assumes all letters are always lowercase unless an asterix appears
before the letter.
