A fast rust library for conversion to and from betacode.
Includes support for TLG betacode, and standard betacode.

# Examples

Convert Robinson-Pierpont style betacode into unicode Greek:

    let word = betacode2::to_greek("qeo/v", betacode2::Type::Default).unwrap();
    assert_eq!(word, "θεός");

Convert TLG style betacode into unicode Greek:

    let word = betacode2::to_greek("qeo/s", betacode2::Type::TLG).unwrap();
    assert_eq!(word, "θεός");

