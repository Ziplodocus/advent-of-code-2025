use crate::{extract_invalid_ids_from_range, handle_even_range};

#[test]
fn test_even_range_11_22() {
    assert_eq!(Vec::from([11, 22]), handle_even_range(("11", "22")))
}

#[test]
fn test_even_range_95_99() {
    assert_eq!(Vec::from([99]), handle_even_range(("95", "99")))
}

#[test]
fn test_range_11_22() {
    let values = extract_invalid_ids_from_range(("11", "22"));

    assert_eq!(Vec::from([11, 22]), values);
}

#[test]
fn test_range_95_115() {
    let values = extract_invalid_ids_from_range(("95", "112"));

    assert_eq!(Vec::from([99]), values);
}
