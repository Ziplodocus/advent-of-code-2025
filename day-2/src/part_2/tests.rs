use crate::part_2::find_invalid_ids;

#[test]
fn check_test_ranges() {
    assert_eq!(Vec::from([11, 22]), find_invalid_ids((11, 22)));
    assert_eq!(Vec::from([99, 111]), find_invalid_ids((95, 115)));
    assert_eq!(Vec::from([999, 1010]), find_invalid_ids((998, 1012)));
    assert_eq!(Vec::from([565656]), find_invalid_ids((565653, 565659)));
    assert_eq!(
        Vec::from([2121212121]),
        find_invalid_ids((2121212118, 2121212124))
    );
}
