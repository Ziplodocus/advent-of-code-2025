use crate::bound::{self, Range};

use super::{find_invalid_ids, handle_even_range, subdivide_range};

#[test]
fn test_even_range_11_22() {
    assert_eq!(
        Vec::from([11, 22]),
        handle_even_range(Range::new(11.into(), 22.into()))
    )
}

#[test]
fn test_even_range_95_99() {
    assert_eq!(
        Vec::from([99]),
        handle_even_range(Range::new(95.into(), 99.into()))
    )
}

#[test]
fn test_range_11_22() {
    let values = find_invalid_ids(Range::new(11.into(), 22.into()));

    assert_eq!(Vec::from([11, 22]), values);
}

#[test]
fn test_range_95_115() {
    let values = find_invalid_ids(Range::new(95.into(), 112.into()));

    assert_eq!(Vec::from([99]), values);
}

#[test]
fn test_range_2121212118_2121212124() {
    let values = handle_even_range(Range::new(2_121_212_118.into(), 2_121_212_121.into()));
    let expected: Vec<usize> = Vec::new();
    assert_eq!(expected, values);
}

#[test]
fn test_range_subdivision() {
    let mut test_range = Range::new(1.into(), 50.into());

    assert_eq!(
        subdivide_range(test_range),
        Vec::from([Range::new(10.into(), 50.into())])
    );

    test_range = Range::new(78.into(), 52725.into());

    assert_eq!(
        subdivide_range(test_range),
        Vec::from([
            Range::new(78.into(), 99.into()),
            Range::new(1000.into(), 9999.into()),
        ])
    )
}
