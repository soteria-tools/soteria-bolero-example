use bolero_test::buggy_add;

#[test]
// #[cfg_attr(kani, kani::proof)]
fn fuzz_add() {
    bolero::check!()
        .with_type()
        .cloned()
        .for_each(|(a, b)| buggy_add(a, b) == a.wrapping_add(b));
}
