use crate::fix;

#[test]
fn test_static_values() {
    assert_eq!(String::from("true"), fix("true"));
    assert_eq!(String::from("false"), fix("false"));
    assert_eq!(String::from("null"), fix("null"));

    assert_eq!(String::from("true"), fix("tru"));
    assert_eq!(String::from("false"), fix("fals"));
    assert_eq!(String::from("null"), fix("nul"));

    assert_eq!(String::from("true"), fix("True"));
    assert_eq!(String::from("false"), fix("FALSE"));
    assert_eq!(String::from("null"), fix("NuLl"));

    assert_eq!(String::from("true"), fix("truefalse01"));

    assert_eq!(String::from("true"), fix("  true  "));
    assert_eq!(String::from("false"), fix("\nfalse\t"));
    assert_eq!(String::from("null"), fix("null "));
}

#[test]
fn test_numbers() {
    assert_eq!(String::from("0"), fix("0"));
    assert_eq!(String::from("1"), fix("1"));
    assert_eq!(String::from("12"), fix("12"));
    assert_eq!(String::from("1230000"), fix("1230000"));

    assert_eq!(String::from("0"), fix("00"));
    assert_eq!(String::from("123"), fix("000123"));

    assert_eq!(String::from("-1"), fix("-1"));
    assert_eq!(String::from("-123"), fix("-123"));

    assert_eq!(String::from("123.01"), fix("123.01"));
    assert_eq!(String::from("123.0001"), fix("123.0001"));
    assert_eq!(String::from("0.5"), fix(".5"));
    assert_eq!(String::from("123.0"), fix("123."));
    assert_eq!(String::from("-123.0"), fix("-123."));

    assert_eq!(String::from("123e1"), fix("123e1"));
    assert_eq!(String::from("123E2"), fix("123E2"));
    assert_eq!(String::from("1e+2"), fix("1e+2"));
    assert_eq!(String::from("1e-2"), fix("1e-2"));
    assert_eq!(String::from("1E-10"), fix("1E-10"));
    assert_eq!(String::from("1"), fix("1e"));
    assert_eq!(String::from("100"), fix("100e+"));

    assert_eq!(String::from("-1"), fix("--1"));
    assert_eq!(String::from("-5"), fix("---5"));

    assert_eq!(String::from("1.2"), fix("1.2.3"));
    assert_eq!(String::from("-0.5"), fix("-.5"));

    assert_eq!(String::from("-0"), fix("-"));

    assert_eq!(String::from("1"), fix("1e+"));
    assert_eq!(String::from("1e2"), fix("1e2e3"));
}

#[test]
fn test_strings() {
    assert_eq!(String::from("\"test\""), fix("\"test\""));

    assert_eq!(String::from("\"test\""), fix("\"test"));

    assert_eq!(String::from("\"line1\\nline2\""), fix("\"line1\\nline2\""));

    assert_eq!(
        String::from("\"hello\\\"world\""),
        fix("\"hello\\\"world\"")
    );

    assert_eq!(
        String::from("\"path\\\\to\\\\file\""),
        fix("\"path\\\\to\\\\file\"")
    );

    assert_eq!(String::from("\"col1\\tcol2\""), fix("\"col1\\tcol2\""));

    assert_eq!(String::from("\"line1\\rline2\""), fix("\"line1\\rline2\""));

    assert_eq!(String::from("\"a\\bb\""), fix("\"a\\bb\""));

    assert_eq!(String::from("\"page1\\fpage2\""), fix("\"page1\\fpage2\""));

    assert_eq!(String::from("\"\""), fix("\"\""));
    assert_eq!(String::from("\"\""), fix("\""));

    assert_eq!(String::from("\"A\""), fix("\"\\u0041\""));
    assert_eq!(String::from("\"☃\""), fix("\"\\u2603\""));

    assert_eq!(String::from("\"\\u0000\""), fix("\"\\u0000\""));
}

#[test]
fn test_arrays() {
    assert_eq!(String::from("[]"), fix("[]"));
    assert_eq!(String::from("[\n   1\n]"), fix("[1]"));
    assert_eq!(String::from("[\n   1,\n   2\n]"), fix("[1,2]"));
    assert_eq!(String::from("[\n   1,\n   2,\n   3\n]"), fix("[1,2, 3]"));
    assert_eq!(
        String::from("[\n   1,\n   \"test\",\n   1.5,\n   null,\n   true\n]"),
        fix("[1,\"test\", 1.5, null, true]")
    );

    assert_eq!(String::from("[]"), fix("["));
    assert_eq!(String::from("[\n   1\n]"), fix("[1"));
    assert_eq!(String::from("[\n   1,\n   2\n]"), fix("[1, 2"));

    assert_eq!(String::from("[]"), fix("[,"));
    assert_eq!(String::from("[\n   1\n]"), fix("[1,"));
    assert_eq!(String::from("[\n   1,\n   2\n]"), fix("[1, 2,"));

    assert_eq!(String::from("[\n   1,\n   2\n]"), fix("[1,,,2]"));
    assert_eq!(String::from("[]"), fix("[,,,]"));

    assert_eq!(String::from("[\n   1\n]"), fix("[1 abc 2]"));
    assert_eq!(String::from("[\n   1,\n   2\n]"), fix("[   1   ,   2   ]"));

    assert_eq!(String::from("[\n   [\n      []\n   ]\n]"), fix("[[[]]]"));
    assert_eq!(
        String::from(
            "[\n   [\n      [\n         [\n            [\n               [\n                  [\n                     [\n                        [\n                           []\n                        ]\n                     ]\n                  ]\n               ]\n            ]\n         ]\n      ]\n   ]\n]"
        ),
        fix("[[[[[[[[[[]")
    );
    assert_eq!(
        String::from(
            "[\n   [\n      [\n         [\n            [\n               []\n            ]\n         ]\n      ]\n   ]\n]"
        ),
        fix("[[[[[[]")
    );
}

#[test]
fn test_objects() {
    assert_eq!(String::from("{}"), fix("{}"));
    assert_eq!(String::from("{\n   \"k\": 1\n}"), fix("{\"k\":1}"));
    assert_eq!(
        String::from("{\n   \"nums\": [\n      1,\n      2,\n      3\n   ]\n}"),
        fix("{\"nums\":[1,2,3]}")
    );
    assert_eq!(
        String::from(
            "{\n   \"list\": [\n      {\n         \"id\": 1\n      },\n      {\n         \"id\": 2\n      }\n   ]\n}"
        ),
        fix("{\"list\":[{\"id\":1},{\"id\":2}]}")
    );
    assert_eq!(
        String::from(
            "{\n   \"grid\": [\n      [\n         1,\n         2\n      ],\n      [\n         3,\n         4\n      ]\n   ]\n}"
        ),
        fix("{\"grid\":[[1,2],[3,4]]}")
    );
    assert_eq!(
        String::from("{\n   \"a\": {\n      \"b\": 2\n   }\n}"),
        fix("{\"a\": {\"b\":2}")
    );
    assert_eq!(
        String::from(
            "{\n   \"a\": {\n      \"b\": {\n         \"c\": {\n            \"d\": 42\n         }\n      }\n   }\n}"
        ),
        fix("{\"a\":{\"b\":{\"c\":{\"d\":42}}}}")
    );

    assert_eq!(String::from("{}"), fix("{"));
    assert_eq!(String::from("{\n   \"k\": 1\n}"), fix("{\"k\":1"));

    assert_eq!(String::from("{\n   \"a\": 1\n}"), fix("{\"a\":1,"));
    assert_eq!(String::from("{\n   \"a\": 1\n}"), fix("{\"a\":1,},,"));

    assert_eq!(String::from("{\n   \"a\": 2\n}"), fix("{\"a\":1,\"a\":2}"));

    assert_eq!(
        String::from("{\n   \"key\\\"with\\\"quotes\": 1\n}"),
        fix("{\"key\\\"with\\\"quotes\":1}")
    );

    assert_eq!(
        String::from("{\n   \"path\\\\key\": 2\n}"),
        fix("{\"path\\\\key\":2}")
    );

    assert_eq!(
        String::from("{\n   \"key\": null\n}"),
        fix("{\"key\" value}")
    );
    assert_eq!(String::from("{}"), fix("{123: 1}"));
    assert_eq!(String::from("{}"), fix("{true: 1}"));
    assert_eq!(String::from("{}"), fix("{null: 2}"));

    assert_eq!(String::from("{\n   \"\": 1\n}"), fix("{\"\": 1}"));
    assert_eq!(String::from("{\n   \"\": 1\n}"), fix("{\"\": 1,"));

    assert_eq!(
        String::from("{\n   \"a\": 3\n}"),
        fix("{\"a\":1,\"a\":2,\"a\":3}")
    );
}

#[test]
fn test_string_edge_cases() {
    assert_eq!(String::from("\"\""), fix(r#""\u1""#));
    assert_eq!(String::from("\"\""), fix(r#""\u12""#));
    assert_eq!(String::from("\"\""), fix(r#""\u123""#));
    assert_eq!(String::from("\"\""), fix(r#""\uXXXX""#));
    assert_eq!(String::from("\"\""), fix(r#""\uGHIJ""#));
    assert_eq!(String::from("\"\""), fix(r#""\u12XY""#));
    assert_eq!(String::from("\"\""), fix(r#""\u---1""#));

    assert_eq!(String::from("\"\""), fix(r#""\uD800""#));
    assert_eq!(String::from("\"\""), fix(r#""\uDC00""#));
    assert_eq!(String::from("\"\""), fix(r#""\uDBFF""#));
    assert_eq!(String::from("\"\""), fix(r#""\uDFFF""#));

    assert_eq!(String::from("\"\""), fix(r#""\uD800\u0041""#));
    assert_eq!(String::from("\"\""), fix(r#""\uD800\u0030""#));

    assert_eq!(String::from(r#""\u0000""#), fix(r#""\u0000""#));
    assert_eq!(String::from("\"\u{ffff}\""), fix(r#""\uFFFF""#));
    assert_eq!(String::from("\"퟿\""), fix(r#""\uD7FF""#));
    assert_eq!(String::from("\"\""), fix(r#""\uE000""#));

    assert_eq!(String::from(r#""testAB""#), fix(r#""test\u0041\u0042""#));
    assert_eq!(String::from(r#""abAc""#), fix(r#""a\uXXXXb\u0041c""#));
}

#[test]
fn test_deserialize() {
    assert_eq!(
        fix(r#"{"data": "{\"nested\": true}"}"#),
        r#"{
   "data": {
      "nested": true
   }
}"#
    );

    assert_eq!(
        fix(r#"{"arr": "[1,2,3]"}"#),
        r#"{
   "arr": [
      1,
      2,
      3
   ]
}"#
    );

    assert_eq!(
        fix(r#"{"val": "   {\"key\": 1}"}"#),
        r#"{
   "val": {
      "key": 1
   }
}"#
    );

    assert_eq!(
        fix(r#"{"a": "{\"b\": "{\"c\": 1}"}"}"#),
        r#"{
   "a": {
      "b": null
   },
   "c\": 1}": null,
   "}": null
}"#
    );

    assert_eq!(
        fix(r#"{"valid": "{1}", "invalid": "not json"}"#),
        r#"{
   "valid": {},
   "invalid": "not json"
}"#
    );

    assert_eq!(
        fix(r#"["{1}", "{\"key\": 2}"]"#),
        r#"[
   {},
   {
      "key": 2
   }
]"#
    );
}

#[test]
fn test_empty() {
    assert_eq!(fix(""), "null");
    assert_eq!(fix("   "), "null");
    assert_eq!(fix("\n\t"), "null");
    assert_eq!(fix(r#""""#), r#""""#);
    assert_eq!(String::from("[\n   \"]\"\n]"), fix(r#"["]"#));
    assert_eq!(String::from("{\n   \"}\": null\n}"), fix(r#"{"}"#));

    assert_eq!(
        String::from(
            "[\n   [\n      [\n         [\n            [\n               [\n                  [\n                     [\n                        [\n                           [\n                              []\n                           ]\n                        ]\n                     ]\n                  ]\n               ]\n            ]\n         ]\n      ]\n   ]\n]"
        ),
        fix("[[[[[[[[[[[]")
    );

    let many_elements = (0..30).map(|i| i.to_string()).collect::<Vec<_>>();
    assert_eq!(
        String::from(
            "[\n   0,\n   1,\n   2,\n   3,\n   4,\n   5,\n   6,\n   7,\n   8,\n   9,\n   10,\n   11,\n   12,\n   13,\n   14,\n   15,\n   16,\n   17,\n   18,\n   19,\n   20,\n   21,\n   22,\n   23,\n   24,\n   25,\n   26,\n   27,\n   28,\n   29\n]"
        ),
        fix(format!(
            "[{}]",
            many_elements
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ))
    );

    assert_eq!(
        String::from("9999999999999999999999"),
        fix("9999999999999999999999")
    );

    assert_eq!(String::from("1e999"), fix("1e999"));
}

