use super::*;

#[test]
fn delay_add() {
    use delay::Delay;
    let left = 420;
    let right = 42;

    let lhs = Delay::with_delay(left);
    let rhs = Delay::with_delay(right);
    let result = lhs + rhs;
    assert_eq!(result, Delay::with_delay(left + right));
}

#[test]
fn delay_sub() {
    use delay::Delay;
    let left = 42;
    let right = 420;

    let lhs = Delay::with_delay(left);
    let rhs = Delay::with_delay(right);
    let result = lhs - rhs;
    assert_eq!(result, Delay::with_delay(left - right));
}

#[test]
fn delay_eq() {
    use delay::Delay;
    let left = 420;
    let right = 420;

    let lhs = Delay::with_delay(left);
    let rhs = Delay::with_delay(right);
    let result = lhs == rhs;
    assert_eq!(result, true);

    let lhs = Delay::with_delay(left - 1);
    let result = lhs == rhs;
    assert_eq!(result, false)
}

#[test]
fn delay_pair_add() {
    use delay::DelayPair;

    let left_min = 2;
    let left_max = 7;

    let right_min = 1;
    let right_max = 9;

    let lhs = DelayPair::with_min_max(left_min.into(), left_max.into());
    let rhs = DelayPair::with_min_max(right_min.into(), right_max.into());
    let result = lhs + rhs;
    assert_eq!(
        result,
        DelayPair::with_min_max((left_min + right_min).into(), (left_max + right_max).into())
    );
}

#[test]
fn delay_pair_sub() {
    use delay::DelayPair;

    let lhs = DelayPair::with_min_max(9.into(), 21.into());
    let rhs = DelayPair::with_min_max(6.into(), 9.into());
    let result = lhs - rhs;
    assert_eq!(result, DelayPair::with_min_max(3.into(), 12.into()));
}
