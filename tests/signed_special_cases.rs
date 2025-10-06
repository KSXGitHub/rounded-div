/// Run test cases for a signed integer type.
macro_rules! cases {
    ($name:ident) => {
        #[cfg(test)]
        mod $name {
            use num::rational::Ratio;

            fn slow(dividend: $name, divisor: $name) -> $name {
                Ratio::new(dividend, divisor).round().to_integer()
            }

            #[test]
            fn min_max_combinations() {
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MIN), -1);
            }

            #[test]
            fn div_by_1() {
                assert_eq!(rounded_div::$name($name::MIN, 1), $name::MIN);
                assert_eq!(rounded_div::$name(-8, 1), -8);
                assert_eq!(rounded_div::$name(-2, 1), -2);
                assert_eq!(rounded_div::$name(-1, 1), -1);
                assert_eq!(rounded_div::$name($name::MAX, 1), $name::MAX);
                assert_eq!(rounded_div::$name(8, 1), 8);
                assert_eq!(rounded_div::$name(2, 1), 2);
                assert_eq!(rounded_div::$name(1, 1), 1);
            }

            #[test]
            fn div_by_neg_1() {
                assert_eq!(rounded_div::$name(-8, -1), 8);
                assert_eq!(rounded_div::$name(-2, -1), 2);
                assert_eq!(rounded_div::$name(-1, -1), 1);
                assert_eq!(rounded_div::$name($name::MAX, -1), -$name::MAX);
                assert_eq!(rounded_div::$name(8, -1), -8);
                assert_eq!(rounded_div::$name(2, -1), -2);
                assert_eq!(rounded_div::$name(1, -1), -1);
            }

            #[test]
            fn div_small_by_min_max() {
                assert_eq!(rounded_div::$name(0, $name::MIN), 0);
                assert_eq!(rounded_div::$name(1, $name::MIN), 0);
                assert_eq!(rounded_div::$name(2, $name::MIN), 0);
                assert_eq!(rounded_div::$name(3, $name::MIN), 0);
                assert_eq!(rounded_div::$name(4, $name::MIN), 0);
                assert_eq!(rounded_div::$name(5, $name::MIN), 0);
                assert_eq!(rounded_div::$name(-1, $name::MIN), 0);
                assert_eq!(rounded_div::$name(-2, $name::MIN), 0);
                assert_eq!(rounded_div::$name(-3, $name::MIN), 0);
                assert_eq!(rounded_div::$name(-4, $name::MIN), 0);
                assert_eq!(rounded_div::$name(-5, $name::MIN), 0);
                assert_eq!(rounded_div::$name(0, $name::MAX), 0);
                assert_eq!(rounded_div::$name(1, $name::MAX), 0);
                assert_eq!(rounded_div::$name(2, $name::MAX), 0);
                assert_eq!(rounded_div::$name(3, $name::MAX), 0);
                assert_eq!(rounded_div::$name(4, $name::MAX), 0);
                assert_eq!(rounded_div::$name(5, $name::MAX), 0);
                assert_eq!(rounded_div::$name(-1, $name::MAX), 0);
                assert_eq!(rounded_div::$name(-2, $name::MAX), 0);
                assert_eq!(rounded_div::$name(-3, $name::MAX), 0);
                assert_eq!(rounded_div::$name(-4, $name::MAX), 0);
                assert_eq!(rounded_div::$name(-5, $name::MAX), 0);

                // MIN is even, so x/MIN where x.abs() < MIN/2 should be less than 0.5 which should round to 0
                assert_eq!(rounded_div::$name($name::MIN / 2 + 1, $name::MIN), 0);
                assert_eq!(rounded_div::$name($name::MIN / 2 + 2, $name::MIN), 0);

                // MAX is odd, so x/MAX where x.abs() <= MAX/2 should be less than 0.5 which should round to 0
                assert_eq!(rounded_div::$name($name::MAX / 2, $name::MAX), 0);
                assert_eq!(rounded_div::$name($name::MAX / 2 - 1, $name::MAX), 0);
                assert_eq!(rounded_div::$name($name::MAX / 2 - 2, $name::MAX), 0);
            }

            #[test]
            fn div_big_by_min_max() {
                assert_eq!($name::MIN & 1, 0); // assert that MIN is even
                assert_eq!($name::MAX & 1, 1); // assert that MAX is odd

                // MIN is even so x/MIN where x <= MIN/2 should be greater than 0.5 which should round to 1
                assert_eq!(rounded_div::$name($name::MIN / 2, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN / 2 - 1, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN / 2 - 2, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN + 1, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN + 2, $name::MIN), 1);

                // MIN is even so x/MIN where x >= -MIN/2 should be less than -0.5 which should round to -1
                assert_eq!(rounded_div::$name(-($name::MIN / 2), $name::MIN), -1);
                assert_eq!(rounded_div::$name(-($name::MIN / 2) + 1, $name::MIN), -1);
                assert_eq!(rounded_div::$name(-($name::MIN / 2) + 2, $name::MIN), -1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MIN), -1);
                assert_eq!(rounded_div::$name($name::MAX - 1, $name::MIN), -1);
                assert_eq!(rounded_div::$name($name::MAX - 2, $name::MIN), -1);

                // MAX is odd so x/MAX where x > MAX/2 should be greater than 0.5 which should round to 1
                assert_eq!(rounded_div::$name($name::MAX / 2 + 1, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MAX / 2 + 2, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MAX - 1, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MAX - 2, $name::MAX), 1);

                // MAX is odd so x/MAX where x < -MAX/2 should be less than -0.5 which should round to -1
                assert_eq!(rounded_div::$name(-($name::MAX / 2) - 1, $name::MAX), -1);
                assert_eq!(rounded_div::$name(-($name::MAX / 2) - 2, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MIN + 1, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MIN + 2, $name::MAX), -1);
            }

            #[test]
            fn div_min_max_by_small() {
                assert_eq!(rounded_div::$name($name::MIN, 1), $name::MIN);
                assert_eq!(rounded_div::$name($name::MIN, 2), slow($name::MIN, 2));
                assert_eq!(rounded_div::$name($name::MIN, 3), slow($name::MIN, 3));
                assert_eq!(rounded_div::$name($name::MIN, 4), slow($name::MIN, 4));
                assert_eq!(rounded_div::$name($name::MIN, 5), slow($name::MIN, 5));
                assert_eq!(rounded_div::$name($name::MIN, 6), slow($name::MIN, 6));
                assert_eq!(rounded_div::$name($name::MIN, 7), slow($name::MIN, 7));
                assert_eq!(rounded_div::$name($name::MAX, 1), $name::MAX);
                assert_eq!(rounded_div::$name($name::MAX, 2), slow($name::MAX, 2));
                assert_eq!(rounded_div::$name($name::MAX, 3), slow($name::MAX, 3));
                assert_eq!(rounded_div::$name($name::MAX, 4), slow($name::MAX, 4));
                assert_eq!(rounded_div::$name($name::MAX, 5), slow($name::MAX, 5));
                assert_eq!(rounded_div::$name($name::MAX, 6), slow($name::MAX, 6));
                assert_eq!(rounded_div::$name($name::MAX, 7), slow($name::MAX, 7));
            }

            #[test]
            fn div_min_max_by_big() {
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN / 2), 2);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN / 2 - 1), 2);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN / 2 - 2), 2);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN), 1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN + 1), 1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MIN + 2), 1);

                assert_eq!(rounded_div::$name($name::MIN, -($name::MIN / 2)), -2);
                assert_eq!(rounded_div::$name($name::MIN, -($name::MIN / 2) + 1), -2);
                assert_eq!(rounded_div::$name($name::MIN, -($name::MIN / 2) + 2), -2);
                assert_eq!(rounded_div::$name($name::MIN, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MAX - 1), -1);
                assert_eq!(rounded_div::$name($name::MIN, $name::MAX - 2), -1);

                assert_eq!(rounded_div::$name($name::MAX, $name::MAX / 2), 2);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX / 2 + 1), 2);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX / 2 + 2), 2);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX), 1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX - 1), 1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX - 2), 1);

                assert_eq!(rounded_div::$name($name::MAX, -($name::MAX / 2)), -2);
                assert_eq!(rounded_div::$name($name::MAX, -($name::MAX / 2) - 1), -2);
                assert_eq!(rounded_div::$name($name::MAX, -($name::MAX / 2) - 2), -2);
                assert_eq!(rounded_div::$name($name::MAX, $name::MIN), -1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MIN + 1), -1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MIN + 2), -1);
            }

            #[test]
            fn equal_absolutes() {
                assert_eq!(rounded_div::$name(1, 1), 1);
                assert_eq!(rounded_div::$name(2, 2), 1);
                assert_eq!(rounded_div::$name(3, 3), 1);
                assert_eq!(rounded_div::$name(4, 4), 1);
                assert_eq!(rounded_div::$name(5, 5), 1);
                assert_eq!(rounded_div::$name(1, -1), -1);
                assert_eq!(rounded_div::$name(2, -2), -1);
                assert_eq!(rounded_div::$name(3, -3), -1);
                assert_eq!(rounded_div::$name(4, -4), -1);
                assert_eq!(rounded_div::$name(5, -5), -1);
                assert_eq!(rounded_div::$name(-1, 1), -1);
                assert_eq!(rounded_div::$name(-2, 2), -1);
                assert_eq!(rounded_div::$name(-3, 3), -1);
                assert_eq!(rounded_div::$name(-4, 4), -1);
                assert_eq!(rounded_div::$name(-5, 5), -1);
                assert_eq!(rounded_div::$name(-1, -1), 1);
                assert_eq!(rounded_div::$name(-2, -2), 1);
                assert_eq!(rounded_div::$name(-3, -3), 1);
                assert_eq!(rounded_div::$name(-4, -4), 1);
                assert_eq!(rounded_div::$name(-5, -5), 1);
                assert_eq!(rounded_div::$name($name::MAX, $name::MAX), 1);
                assert_eq!(rounded_div::$name(-$name::MAX, $name::MAX), -1);
                assert_eq!(rounded_div::$name($name::MAX, -$name::MAX), -1);
                assert_eq!(rounded_div::$name(-$name::MAX, -$name::MAX), 1);
            }

            #[cfg(test)]
            mod div_by_0 {
                #[test]
                #[should_panic(expected = "attempt to divide by zero")]
                fn div_0_by_0() {
                    let _ = rounded_div::$name(0, 0);
                }

                #[test]
                #[should_panic(expected = "attempt to divide by zero")]
                fn div_min_by_0() {
                    let _ = rounded_div::$name($name::MIN, 0);
                }

                #[test]
                #[should_panic(expected = "attempt to divide by zero")]
                fn div_max_by_0() {
                    let _ = rounded_div::$name($name::MAX, 0);
                }

                #[test]
                #[should_panic(expected = "attempt to divide by zero")]
                fn div_1_by_0() {
                    let _ = rounded_div::$name(1, 0);
                }
            }
        }
    };
}

cases!(i8);
cases!(i16);
cases!(i32);
cases!(i64);
cases!(i128);
cases!(isize);
