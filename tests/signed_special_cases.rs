#[cfg(test)]
mod i8 {
    use decimal_rs::Decimal;

    fn slow_rounded_div(dividend: i8, divisor: i8) -> i8 {
        let dividend: Decimal = dividend.into();
        let divisor: Decimal = divisor.into();
        let (int_val, scale, negative) = (dividend / divisor).round(0).into_parts();
        assert_eq!(scale, 0);
        let int_val = int_val as i128;
        let factor: i128 = match negative {
            true => -1,
            false => 1,
        };
        (int_val * factor) as i8
    }

    #[test]
    fn min_max_combinations() {
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MIN), -1);
    }

    #[test]
    fn div_by_1() {
        assert_eq!(rounded_div::i8(i8::MIN, 1), i8::MIN);
        assert_eq!(rounded_div::i8(-8, 1), -8);
        assert_eq!(rounded_div::i8(-2, 1), -2);
        assert_eq!(rounded_div::i8(-1, 1), -1);
        assert_eq!(rounded_div::i8(i8::MAX, 1), i8::MAX);
        assert_eq!(rounded_div::i8(8, 1), 8);
        assert_eq!(rounded_div::i8(2, 1), 2);
        assert_eq!(rounded_div::i8(1, 1), 1);
    }

    #[test]
    fn div_by_neg_1() {
        assert_eq!(rounded_div::i8(-8, -1), 8);
        assert_eq!(rounded_div::i8(-2, -1), 2);
        assert_eq!(rounded_div::i8(-1, -1), 1);
        assert_eq!(rounded_div::i8(i8::MAX, -1), -i8::MAX);
        assert_eq!(rounded_div::i8(8, -1), -8);
        assert_eq!(rounded_div::i8(2, -1), -2);
        assert_eq!(rounded_div::i8(1, -1), -1);
    }

    #[test]
    fn div_small_by_min_max() {
        assert_eq!(rounded_div::i8(0, i8::MIN), 0);
        assert_eq!(rounded_div::i8(1, i8::MIN), 0);
        assert_eq!(rounded_div::i8(2, i8::MIN), 0);
        assert_eq!(rounded_div::i8(3, i8::MIN), 0);
        assert_eq!(rounded_div::i8(4, i8::MIN), 0);
        assert_eq!(rounded_div::i8(5, i8::MIN), 0);
        assert_eq!(rounded_div::i8(-1, i8::MIN), 0);
        assert_eq!(rounded_div::i8(-2, i8::MIN), 0);
        assert_eq!(rounded_div::i8(-3, i8::MIN), 0);
        assert_eq!(rounded_div::i8(-4, i8::MIN), 0);
        assert_eq!(rounded_div::i8(-5, i8::MIN), 0);
        assert_eq!(rounded_div::i8(0, i8::MAX), 0);
        assert_eq!(rounded_div::i8(1, i8::MAX), 0);
        assert_eq!(rounded_div::i8(2, i8::MAX), 0);
        assert_eq!(rounded_div::i8(3, i8::MAX), 0);
        assert_eq!(rounded_div::i8(4, i8::MAX), 0);
        assert_eq!(rounded_div::i8(5, i8::MAX), 0);
        assert_eq!(rounded_div::i8(-1, i8::MAX), 0);
        assert_eq!(rounded_div::i8(-2, i8::MAX), 0);
        assert_eq!(rounded_div::i8(-3, i8::MAX), 0);
        assert_eq!(rounded_div::i8(-4, i8::MAX), 0);
        assert_eq!(rounded_div::i8(-5, i8::MAX), 0);

        // MIN is even, so x/MIN where x.abs() < MIN/2 should be less than 0.5 which should round to 0
        assert_eq!(rounded_div::i8(i8::MIN / 2 + 1, i8::MIN), 0);
        assert_eq!(rounded_div::i8(i8::MIN / 2 + 2, i8::MIN), 0);

        // MAX is odd, so x/MAX where x.abs() <= MAX/2 should be less than 0.5 which should round to 0
        assert_eq!(rounded_div::i8(i8::MAX / 2, i8::MAX), 0);
        assert_eq!(rounded_div::i8(i8::MAX / 2 - 1, i8::MAX), 0);
        assert_eq!(rounded_div::i8(i8::MAX / 2 - 2, i8::MAX), 0);
    }

    #[test]
    fn div_big_by_min_max() {
        assert_eq!(i8::MIN & 1, 0); // assert that MIN is even
        assert_eq!(i8::MAX & 1, 1); // assert that MAX is odd

        // MIN is even so x/MIN where x <= MIN/2 should be greater than 0.5 which should round to 1
        assert_eq!(rounded_div::i8(i8::MIN / 2, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN / 2 - 1, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN / 2 - 2, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN + 1, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN + 2, i8::MIN), 1);

        // MIN is even so x/MIN where x >= -MIN/2 should be less than -0.5 which should round to -1
        assert_eq!(rounded_div::i8(-(i8::MIN / 2), i8::MIN), -1);
        assert_eq!(rounded_div::i8(-(i8::MIN / 2) + 1, i8::MIN), -1);
        assert_eq!(rounded_div::i8(-(i8::MIN / 2) + 2, i8::MIN), -1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MIN), -1);
        assert_eq!(rounded_div::i8(i8::MAX - 1, i8::MIN), -1);
        assert_eq!(rounded_div::i8(i8::MAX - 2, i8::MIN), -1);

        // MAX is odd so x/MAX where x > MAX/2 should be greater than 0.5 which should round to 1
        assert_eq!(rounded_div::i8(i8::MAX / 2 + 1, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MAX / 2 + 2, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MAX - 1, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MAX - 2, i8::MAX), 1);

        // MAX is odd so x/MAX where x < -MAX/2 should be less than -0.5 which should round to -1
        assert_eq!(rounded_div::i8(-(i8::MAX / 2) - 1, i8::MAX), -1);
        assert_eq!(rounded_div::i8(-(i8::MAX / 2) - 2, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MIN + 1, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MIN + 2, i8::MAX), -1);
    }

    #[test]
    fn div_min_max_by_small() {
        assert_eq!(rounded_div::i8(i8::MIN, 1), i8::MIN);
        assert_eq!(rounded_div::i8(i8::MIN, 2), slow_rounded_div(i8::MIN, 2));
        assert_eq!(rounded_div::i8(i8::MIN, 3), slow_rounded_div(i8::MIN, 3));
        assert_eq!(rounded_div::i8(i8::MIN, 4), slow_rounded_div(i8::MIN, 4));
        assert_eq!(rounded_div::i8(i8::MIN, 5), slow_rounded_div(i8::MIN, 5));
        assert_eq!(rounded_div::i8(i8::MIN, 6), slow_rounded_div(i8::MIN, 6));
        assert_eq!(rounded_div::i8(i8::MIN, 7), slow_rounded_div(i8::MIN, 7));
        assert_eq!(rounded_div::i8(i8::MAX, 1), i8::MAX);
        assert_eq!(rounded_div::i8(i8::MAX, 2), slow_rounded_div(i8::MAX, 2));
        assert_eq!(rounded_div::i8(i8::MAX, 3), slow_rounded_div(i8::MAX, 3));
        assert_eq!(rounded_div::i8(i8::MAX, 4), slow_rounded_div(i8::MAX, 4));
        assert_eq!(rounded_div::i8(i8::MAX, 5), slow_rounded_div(i8::MAX, 5));
        assert_eq!(rounded_div::i8(i8::MAX, 6), slow_rounded_div(i8::MAX, 6));
        assert_eq!(rounded_div::i8(i8::MAX, 7), slow_rounded_div(i8::MAX, 7));
    }

    #[test]
    fn div_min_max_by_big() {
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN / 2), 2);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN / 2 - 1), 2);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN / 2 - 2), 2);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN), 1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN + 1), 1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MIN + 2), 1);

        assert_eq!(rounded_div::i8(i8::MIN, -(i8::MIN / 2)), -2);
        assert_eq!(rounded_div::i8(i8::MIN, -(i8::MIN / 2) + 1), -2);
        assert_eq!(rounded_div::i8(i8::MIN, -(i8::MIN / 2) + 2), -2);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MAX - 1), -1);
        assert_eq!(rounded_div::i8(i8::MIN, i8::MAX - 2), -1);

        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX / 2), 2);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX / 2 + 1), 2);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX / 2 + 2), 2);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX), 1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX - 1), 1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX - 2), 1);

        assert_eq!(rounded_div::i8(i8::MAX, -(i8::MAX / 2)), -2);
        assert_eq!(rounded_div::i8(i8::MAX, -(i8::MAX / 2) - 1), -2);
        assert_eq!(rounded_div::i8(i8::MAX, -(i8::MAX / 2) - 2), -2);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MIN), -1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MIN + 1), -1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MIN + 2), -1);
    }

    #[test]
    fn equal_absolutes() {
        assert_eq!(rounded_div::i8(1, 1), 1);
        assert_eq!(rounded_div::i8(2, 2), 1);
        assert_eq!(rounded_div::i8(3, 3), 1);
        assert_eq!(rounded_div::i8(4, 4), 1);
        assert_eq!(rounded_div::i8(5, 5), 1);
        assert_eq!(rounded_div::i8(1, -1), -1);
        assert_eq!(rounded_div::i8(2, -2), -1);
        assert_eq!(rounded_div::i8(3, -3), -1);
        assert_eq!(rounded_div::i8(4, -4), -1);
        assert_eq!(rounded_div::i8(5, -5), -1);
        assert_eq!(rounded_div::i8(-1, 1), -1);
        assert_eq!(rounded_div::i8(-2, 2), -1);
        assert_eq!(rounded_div::i8(-3, 3), -1);
        assert_eq!(rounded_div::i8(-4, 4), -1);
        assert_eq!(rounded_div::i8(-5, 5), -1);
        assert_eq!(rounded_div::i8(-1, -1), 1);
        assert_eq!(rounded_div::i8(-2, -2), 1);
        assert_eq!(rounded_div::i8(-3, -3), 1);
        assert_eq!(rounded_div::i8(-4, -4), 1);
        assert_eq!(rounded_div::i8(-5, -5), 1);
        assert_eq!(rounded_div::i8(i8::MAX, i8::MAX), 1);
        assert_eq!(rounded_div::i8(-i8::MAX, i8::MAX), -1);
        assert_eq!(rounded_div::i8(i8::MAX, -i8::MAX), -1);
        assert_eq!(rounded_div::i8(-i8::MAX, -i8::MAX), 1);
    }

    #[cfg(test)]
    mod div_by_0 {
        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_0_by_0() {
            let _ = rounded_div::i8(0, 0);
        }

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_min_by_0() {
            let _ = rounded_div::i8(i8::MIN, 0);
        }

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_max_by_0() {
            let _ = rounded_div::i8(i8::MAX, 0);
        }

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_1_by_0() {
            let _ = rounded_div::i8(1, 0);
        }
    }
}
