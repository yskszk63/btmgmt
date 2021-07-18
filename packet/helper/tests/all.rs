#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/unit-struct.rs");
    t.pass("tests/tuple-struct.rs");
    t.pass("tests/standard-struct.rs");
    t.pass("tests/bitflags.rs");
    t.pass("tests/unit-enum.rs");
    t.pass("tests/commands.rs");
    t.pass("tests/events.rs");
    t.pass("tests/iter_newtype.rs");
    t.pass("tests/iter_newtype_hashset.rs");
    t.pass("tests/iter_newtype_generics.rs");
    t.pass("tests/newtype.rs");
}
