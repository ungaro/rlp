#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-decode-tx.rs");
    t.pass("tests/02-encode-tx.rs");
    t.pass("tests/03-derive-skip.rs");
    t.pass("tests/04-derive-macro.rs");
    t.pass("tests/05-derive-flatten.rs");
    t.pass("tests/06-decode-raw_full.rs");
}