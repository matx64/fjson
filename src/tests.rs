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

#[test]
fn test_numbers() {
    // Integers
    assert_eq!(String::from("0"), fix("0"));
    assert_eq!(String::from("1"), fix("1"));
    assert_eq!(String::from("12"), fix("12"));
    assert_eq!(String::from("1230000"), fix("1230000"));

    // Leading zeros
    assert_eq!(String::from("0"), fix("00"));
    assert_eq!(String::from("123"), fix("000123"));

    // Negatives
    assert_eq!(String::from("-1"), fix("-1"));
    assert_eq!(String::from("-123"), fix("-123"));

    // Floats
    assert_eq!(String::from("123.01"), fix("123.01"));
    assert_eq!(String::from("123.0001"), fix("123.0001"));
    assert_eq!(String::from("0.5"), fix(".5"));
    assert_eq!(String::from("123.0"), fix("123."));
    assert_eq!(String::from("-123.0"), fix("-123."));
    assert_eq!(String::from("123.0001"), fix("123.00.01"));

    // Exponents
    assert_eq!(String::from("123e1"), fix("123e1"));
    assert_eq!(String::from("123e2"), fix("123E2"));
    assert_eq!(String::from("1e+2"), fix("1e+2"));
    assert_eq!(String::from("1e-2"), fix("1e-2"));
    assert_eq!(String::from("1e-10"), fix("1E-10"));
    assert_eq!(String::from("1"), fix("1e"));
    assert_eq!(String::from("100"), fix("100e+"));
}

#[test]
fn test_strings() {
    assert_eq!(String::from("\"test\""), fix("\"test\""));

    // missing closing quote
    assert_eq!(String::from("\"test\""), fix("\"test"));

    // escaped quotes
    assert_eq!(
        String::from("\"a \\\"quote\\\" inside\""),
        fix("\"a \\\"quote\\\" inside\"")
    );

    // backslash escaping
    assert_eq!(
        String::from("\"path \\\\ to \\\\ file\""),
        fix("\"path \\\\ to \\\\ file\"")
    );

    // newline escape
    assert_eq!(String::from("\"line1\nline2\""), fix("\"line1\nline2\""));
}

#[test]
fn test_arrays() {
    assert_eq!(String::from("[]"), fix("[]"));
    assert_eq!(String::from("[1]"), fix("[1]"));
    assert_eq!(String::from("[1,2]"), fix("[1,2]"));
    assert_eq!(String::from("[1,2,3]"), fix("[1,2, 3]"));
    assert_eq!(
        String::from("[1,\"test\",1.5,null,true]"),
        fix("[1,\"test\", 1.5, null, true]")
    );

    // unclosed
    assert_eq!(String::from("[]"), fix("["));
    assert_eq!(String::from("[1]"), fix("[1"));
    assert_eq!(String::from("[1,2]"), fix("[1, 2"));

    // trailing comma
    assert_eq!(String::from("[]"), fix("[,"));
    assert_eq!(String::from("[1]"), fix("[1,"));
    assert_eq!(String::from("[1,2]"), fix("[1, 2,"));

    // nested
    assert_eq!(String::from("[[[]]]"), fix("[[[]]]"));
    assert_eq!(String::from("[[[]]]"), fix("[[[],]"));
}

#[test]
fn test_objects() {
    assert_eq!(String::from("{}"), fix("{}"));
    assert_eq!(String::from("{\"k\":1}"), fix("{\"k\":1}"));
    assert_eq!(
        String::from("{\"nums\":[1,2,3]}"),
        fix("{\"nums\":[1,2,3]}")
    );
    assert_eq!(
        String::from("{\"list\":[{\"id\":1},{\"id\":2}]}"),
        fix("{\"list\":[{\"id\":1},{\"id\":2}]}")
    );
    assert_eq!(
        String::from("{\"grid\":[[1,2],[3,4]]}"),
        fix("{\"grid\":[[1,2],[3,4]]}")
    );
    assert_eq!(String::from("{\"a\":{\"b\":2}}"), fix("{\"a\": {\"b\":2}"));
    assert_eq!(
        String::from("{\"a\":{\"b\":{\"c\":{\"d\":42}}}}"),
        fix("{\"a\":{\"b\":{\"c\":{\"d\":42}}}}")
    );

    // unclosed
    assert_eq!(String::from("{}"), fix("{"));
    assert_eq!(String::from("{\"k\":1}"), fix("{\"k\":1"));

    // trailing comma
    assert_eq!(String::from("{\"a\":1}"), fix("{\"a\":1,"));
    assert_eq!(String::from("{\"a\":1}"), fix("{\"a\":1,},,"));

    // duplicated keys
    assert_eq!(String::from("{\"a\":2}"), fix("{\"a\":1,\"a\":2}"));
}
