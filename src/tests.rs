use crate::fix_without_formatting;

#[test]
fn test_static_values() {
    assert_eq!(String::from("true"), fix_without_formatting("true"));
    assert_eq!(String::from("false"), fix_without_formatting("false"));
    assert_eq!(String::from("null"), fix_without_formatting("null"));

    // Partial / incomplete values
    assert_eq!(String::from("true"), fix_without_formatting("tru"));
    assert_eq!(String::from("false"), fix_without_formatting("fals"));
    assert_eq!(String::from("null"), fix_without_formatting("nul"));

    // Wrong casing
    assert_eq!(String::from("true"), fix_without_formatting("True"));
    assert_eq!(String::from("false"), fix_without_formatting("FALSE"));
    assert_eq!(String::from("null"), fix_without_formatting("NuLl"));

    // Extra characters
    assert_eq!(String::from("true"), fix_without_formatting("truefalse01"));
    assert_eq!(String::from("false"), fix_without_formatting("false123"));
    assert_eq!(String::from("null"), fix_without_formatting("null,bla"));

    // Leading/trailing whitespace
    assert_eq!(String::from("true"), fix_without_formatting("  true  "));
    assert_eq!(String::from("false"), fix_without_formatting("\nfalse\t"));
    assert_eq!(String::from("null"), fix_without_formatting("null "));
}

#[test]
fn test_numbers() {
    // Integers
    assert_eq!(String::from("0"), fix_without_formatting("0"));
    assert_eq!(String::from("1"), fix_without_formatting("1"));
    assert_eq!(String::from("12"), fix_without_formatting("12"));
    assert_eq!(String::from("1230000"), fix_without_formatting("1230000"));

    // Leading zeros
    assert_eq!(String::from("0"), fix_without_formatting("00"));
    assert_eq!(String::from("123"), fix_without_formatting("000123"));

    // Negatives
    assert_eq!(String::from("-1"), fix_without_formatting("-1"));
    assert_eq!(String::from("-123"), fix_without_formatting("-123"));

    // Floats
    assert_eq!(String::from("123.01"), fix_without_formatting("123.01"));
    assert_eq!(String::from("123.0001"), fix_without_formatting("123.0001"));
    assert_eq!(String::from("0.5"), fix_without_formatting(".5"));
    assert_eq!(String::from("123.0"), fix_without_formatting("123."));
    assert_eq!(String::from("-123.0"), fix_without_formatting("-123."));
    assert_eq!(
        String::from("123.0001"),
        fix_without_formatting("123.00.01")
    );

    // Exponents
    assert_eq!(String::from("123e1"), fix_without_formatting("123e1"));
    assert_eq!(String::from("123e2"), fix_without_formatting("123E2"));
    assert_eq!(String::from("1e+2"), fix_without_formatting("1e+2"));
    assert_eq!(String::from("1e-2"), fix_without_formatting("1e-2"));
    assert_eq!(String::from("1e-10"), fix_without_formatting("1E-10"));
    assert_eq!(String::from("1"), fix_without_formatting("1e"));
    assert_eq!(String::from("100"), fix_without_formatting("100e+"));
}

#[test]
fn test_strings() {
    assert_eq!(String::from("\"test\""), fix_without_formatting("\"test\""));

    // missing closing quote
    assert_eq!(String::from("\"test\""), fix_without_formatting("\"test"));

    // newline escape
    assert_eq!(
        String::from("\"line1\nline2\""),
        fix_without_formatting("\"line1\nline2\"")
    );
}

#[test]
fn test_arrays() {
    assert_eq!(String::from("[]"), fix_without_formatting("[]"));
    assert_eq!(String::from("[1]"), fix_without_formatting("[1]"));
    assert_eq!(String::from("[1,2]"), fix_without_formatting("[1,2]"));
    assert_eq!(String::from("[1,2,3]"), fix_without_formatting("[1,2, 3]"));
    assert_eq!(
        String::from("[1,\"test\",1.5,null,true]"),
        fix_without_formatting("[1,\"test\", 1.5, null, true]")
    );

    // unclosed
    assert_eq!(String::from("[]"), fix_without_formatting("["));
    assert_eq!(String::from("[1]"), fix_without_formatting("[1"));
    assert_eq!(String::from("[1,2]"), fix_without_formatting("[1, 2"));

    // trailing comma
    assert_eq!(String::from("[]"), fix_without_formatting("[,"));
    assert_eq!(String::from("[1]"), fix_without_formatting("[1,"));
    assert_eq!(String::from("[1,2]"), fix_without_formatting("[1, 2,"));

    // nested
    assert_eq!(String::from("[[[]]]"), fix_without_formatting("[[[]]]"));
    assert_eq!(String::from("[[[]]]"), fix_without_formatting("[[[],]"));
}

#[test]
fn test_objects() {
    assert_eq!(String::from("{}"), fix_without_formatting("{}"));
    assert_eq!(
        String::from("{\"k\":1}"),
        fix_without_formatting("{\"k\":1}")
    );
    assert_eq!(
        String::from("{\"nums\":[1,2,3]}"),
        fix_without_formatting("{\"nums\":[1,2,3]}")
    );
    assert_eq!(
        String::from("{\"list\":[{\"id\":1},{\"id\":2}]}"),
        fix_without_formatting("{\"list\":[{\"id\":1},{\"id\":2}]}")
    );
    assert_eq!(
        String::from("{\"grid\":[[1,2],[3,4]]}"),
        fix_without_formatting("{\"grid\":[[1,2],[3,4]]}")
    );
    assert_eq!(
        String::from("{\"a\":{\"b\":2}}"),
        fix_without_formatting("{\"a\": {\"b\":2}")
    );
    assert_eq!(
        String::from("{\"a\":{\"b\":{\"c\":{\"d\":42}}}}"),
        fix_without_formatting("{\"a\":{\"b\":{\"c\":{\"d\":42}}}}")
    );

    // unclosed
    assert_eq!(String::from("{}"), fix_without_formatting("{"));
    assert_eq!(
        String::from("{\"k\":1}"),
        fix_without_formatting("{\"k\":1")
    );

    // trailing comma
    assert_eq!(
        String::from("{\"a\":1}"),
        fix_without_formatting("{\"a\":1,")
    );
    assert_eq!(
        String::from("{\"a\":1}"),
        fix_without_formatting("{\"a\":1,},,")
    );

    // duplicated keys
    assert_eq!(
        String::from("{\"a\":2}"),
        fix_without_formatting("{\"a\":1,\"a\":2}")
    );
}
