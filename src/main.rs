use floating_point_addition::Float;

fn main() {

    let f1 = 1.5f32;
    let f2 = -4.25f32;

    println!("{f1}: {:032b}", f1.to_bits());
    println!("{f2}: {:032b}", f2.to_bits());

    let f1: Float = f1.into();
    let f2: Float = f2.into();

    let sum = f1 + f2;

    let sum: f32 = sum.into();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use floating_point_addition::Float;

    // #[test]
    // fn extract_bits_works() {
    //     let value = 0b00111000000000000000000000000000;
    //     let extracted = extract_bits(1, 5, value);

    //     assert_eq!(extracted, 0b01110)
    // }

    // #[test]
    // fn get_parts_works() {
    //     let value = 0b10110100101000100101010100111100u32;

    //     let (sign, exponent, mantissa) = get_parts(value);

    //     assert_eq!(sign, 1);
    //     assert_eq!(exponent, 0b01101001);
    //     assert_eq!(mantissa, 0b01000100101010100111100);
    // }

    // #[test]
    // fn adjust_mantissa_works() {
    //     let m1 = 0b1000100000;
    //     let m2 = 0b111000000;
    //     let e1 = 2;
    //     let e2 = 5;

    //     let (m1, m2, common_exponent) = adjust_mantissa((m1, e1), (m2, e2));

    //     assert_eq!(m1, 0b1000100);
    //     assert_eq!(m2, 0b111000000);
    //     assert_eq!(common_exponent, 5);
    // }

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
        let f1 = -1.2f32;
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
        let f1 = 5.1f32;
        let f2 = -2.8f32;

        test_two_floats(f1, f2);
    }

    #[test]
    fn test_subnormal_numbers_1() {
        let zero = 0f32;
        let value = 419037.1875f32;
        
        let non_zero_mantissa_1 =  5.90381056005e-41f32;
        let non_zero_mantissa_2 = 7.35996705665e-39f32;

        // test_two_floats(zero, value);
        test_two_floats(value, zero);
        // test_two_floats(zero, zero);
        //test_two_floats(non_zero_mantissa_1, non_zero_mantissa_2);
        // test_two_floats(non_zero_mantissa_1, zero);
    }




}


// 101000110011001100110011
// 1010001100110011001100110

// 10110011001100110011001
// 101100110011001100110011