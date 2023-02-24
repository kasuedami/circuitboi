use crate::logic::{Signal, Binary};

#[test]
fn test_logic_and_binary() {
    assert!(Signal::Binary(Binary::Low) & Signal::Binary(Binary::High) == Signal::Binary(Binary::Low));
    assert!(Signal::Binary(Binary::High) & Signal::Binary(Binary::Low) == Signal::Binary(Binary::Low));
    assert!(Signal::Binary(Binary::High) & Signal::Binary(Binary::High) == Signal::Binary(Binary::High));
    assert!(Signal::Binary(Binary::Low) & Signal::Binary(Binary::Low) == Signal::Binary(Binary::Low));
}

#[test]
fn test_logic_or_binary() {
    assert!(Signal::Binary(Binary::Low) | Signal::Binary(Binary::High) == Signal::Binary(Binary::High));
    assert!(Signal::Binary(Binary::High) | Signal::Binary(Binary::Low) == Signal::Binary(Binary::High));
    assert!(Signal::Binary(Binary::High) | Signal::Binary(Binary::High) == Signal::Binary(Binary::High));
    assert!(Signal::Binary(Binary::Low) | Signal::Binary(Binary::Low) == Signal::Binary(Binary::Low));
}