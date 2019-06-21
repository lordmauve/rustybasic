use std::collections::HashMap;

use basic::parse_script;


#[test]
fn test_parse() {
    let mut expected = HashMap::new();
    expected.insert(10, "PRINT \"hello\"".to_string());

    assert_eq!(
        parse_script("10 PRINT \"hello\"").unwrap(),
        expected
    )
}