#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse.rs");
    t.pass("tests/getters.rs");
    t.pass("tests/same_attribute_for_property.rs");
    t.pass("tests/no_duplicated_getter.rs");
    t.pass("tests/getter_expected_results.rs");
}
