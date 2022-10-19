use super::*;

#[test]
fn delay_pair_add() {
    use delay::DelayPair;

    let lhs = DelayPair::with_min_max(2.into(), 2.into());
    let rhs = DelayPair::with_min_max(1.into(), 6.into());
    let result = lhs + rhs;
    assert_eq!(result, DelayPair::with_min_max(3.into(), 8.into()));
}

#[test]
fn delay_pair_sub() {
    use delay::DelayPair;

    let lhs = DelayPair::with_min_max(9.into(), 21.into());
    let rhs = DelayPair::with_min_max(6.into(), 9.into());
    let result = lhs - rhs;
    assert_eq!(result, DelayPair::with_min_max(3.into(), 12.into()));
}