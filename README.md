A fast rust library for conversion to and from betacode.
Includes support for TLG betacode, and standard betacode.

# Examples

Convert Robinson-Pierpont style betacode into unicode Greek:

    let word = betacode2::to_greek("Qeo/v", betacode2::Type::Default).unwrap();
    assert_eq!(word, "Θεός");


Convert TLG style betacode into unicode Greek:

    let word = betacode2::to_greek("*qeo/s", betacode2::Type::TLG).unwrap();
    assert_eq!(word, "Θεός");

The default converter assumes lowercase ascii letters are lowercase Greek
letters and uppercase ascii letters are uppercase Greek letters. The TLG
converter assumes all letters are always lowercase unless an asterix appears
before the letter.
