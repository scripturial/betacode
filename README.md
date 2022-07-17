A fast rust and strict library for conversion to and from betacode.
Includes support for TLG betacode, and standard betacode.

    let result = betacode2::to_greek("qeo/v", betacode2::Type::Default).unwrap();
    assert_eq!(result, "θεός");

    let result = betacode2::to_greek("qeo/s", betacode2::Type::TLG).unwrap();
    assert_eq!(result, "θεός");
