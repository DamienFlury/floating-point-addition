#[cfg(test)]
mod tests {
    use floating_point_addition::Float;


    fn test_two_floats(lhs: f32, rhs: f32) {

        let f1: Float = lhs.into();
        let f2: Float = rhs.into();

        let expected = lhs + rhs;


        let actual: f32 = (f1 + f2).into();


        assert_eq!(expected, actual, "Testing addition with {:032b} and {:032b}", expected.to_bits(), actual.to_bits());
    }

    #[test]
    fn test_positive_1() {
        let f1 = 5.1f32;
        let f2 = 2.8f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_positive_2() {
        let f1 = 1.5f32;
        let f2 = 3.25f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_negative_1() {
        let f1 = -5.1f32;
        let f2 = 2.8f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_negative_2() {
        let f1 = 1.5f32;
        let f2 = -3.25f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_negative_3() {
        let f1 = -1.5f32;
        let f2 = -3.25f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_negative_4() {
        let f1 = -5.1f32;
        let f2 = -2.8f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_subnormal_numbers_1() {
        let zero = 0f32;
        let value = 419037.1875f32;
        
        let non_zero_mantissa_1 =  5.90381056005e-41f32;
        let non_zero_mantissa_2 = 7.35996705665e-39f32;

        test_two_floats(zero, value);
        test_two_floats(value, zero);
        test_two_floats(zero, zero);
        test_two_floats(non_zero_mantissa_1, non_zero_mantissa_2);
        test_two_floats(non_zero_mantissa_2, zero);
    }

    #[test]
    fn edge_case() {
        let lhs = 4.34234f32;
        let rhs = 1.23214f32;

        let f1: Float = lhs.into();
        let f2: Float = rhs.into();

        let expected = lhs + rhs;


        let actual: f32 = (f1 + f2).into();

        test_two_floats(lhs, rhs)
    }

}
