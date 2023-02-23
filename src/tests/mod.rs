use crate::logic::{Signal, Binary};

#[test]
fn test_logic() {
    let signal_low = Signal::Binary(Binary::Low);
    let signal_high = Signal::Binary(Binary::Low);
    
    assert!(signal_low & signal_high == Signal::Binary(Binary::Low));
}