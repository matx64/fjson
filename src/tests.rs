use crate::fix;

#[test]
fn test_static_values() {
    assert_eq!(String::from("true"), fix("true"));
    assert_eq!(String::from("false"), fix("false"));
    assert_eq!(String::from("null"), fix("null"));

    // Partial / incomplete values
    assert_eq!(String::from("true"), fix("tru"));
    assert_eq!(String::from("false"), fix("fals"));
    assert_eq!(String::from("null"), fix("nul"));

    // Wrong casing
    assert_eq!(String::from("true"), fix("True"));
    assert_eq!(String::from("false"), fix("FALSE"));
    assert_eq!(String::from("null"), fix("NuLl"));

    // Extra characters
    assert_eq!(String::from("true"), fix("truefalse01"));
    assert_eq!(String::from("false"), fix("false123"));
    assert_eq!(String::from("null"), fix("null,bla"));

    // Leading/trailing whitespace
    assert_eq!(String::from("true"), fix("  true  "));
    assert_eq!(String::from("false"), fix("\nfalse\t"));
    assert_eq!(String::from("null"), fix("null "));
}
