use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Float(u32);

const fn extract_bits(start: u32, len: u32, value: u32) -> u32 {
    value << start >> (32 - len)
}

impl Float {
    pub fn from_bits(value: u32) -> Self {
        Self(value)
    }

    pub fn from_f32(value: f32) -> Self {
        value.into()
    }
}

fn prepend_leading_one(mantissa: u32, exponent: u32) -> u32 {
    if exponent == 0 {
        mantissa
    } else {
        mantissa | (1 << 23)
    }
}

fn adjust_mantissa(left: (u32, u32), right: (u32, u32)) -> (u32, u32, u32) {
    let (mut m1, e1) = left;
    let (mut m2, e2) = right;
    let e1 = if e1 == 0 { 0 } else { e1 };
    let e2 = if e2 == 0 { 0 } else { e2 };


    if e1 != e2 {
        let difference = e1 as i32 - e2 as i32;
        let absolute_difference = difference.abs();

        if difference > 0 {
            if m2 != 0 {
                m1 <<= absolute_difference;
            }
        } else if m1 != 0 {
            m2 <<= absolute_difference;
        }
    }

    if e1 == 0 || e2 == 0 {
        (m1, m2, if e1 == 0 { e2 } else { e1 })
    } else {
        (m1, m2, e1.min(e2))
    }
}

fn normalize_sign(sign: u32) -> i32 {
    1 - 2 * sign as i32
}

const fn get_parts(value: u32) -> (u32, u32, u32) {
    let sign = extract_bits(0, 1, value);
    let exponent = extract_bits(1, 8, value);
    let mantissa = extract_bits(9, 23, value);

    (sign, exponent, mantissa)
}

const fn required_bit_shifts(mantissa: u32) -> i32 {
    if mantissa == 0 {
        0
    } else {
        let leading_zeros = mantissa.leading_zeros();
        8 - leading_zeros as i32
    }
}

fn normalize_mantissa(mantissa: u32) -> u32 {
    let shift_value = required_bit_shifts(mantissa);

    if shift_value > 0 {
        mantissa >> shift_value
    } else {
        mantissa << shift_value.abs()
    }
}

impl Add for Float {
    type Output = Float;

    fn add(self, rhs: Self) -> Self::Output {
        let left = get_parts(self.0);
        let right = get_parts(rhs.0);
        let s1 = normalize_sign(left.0);
        let s2 = normalize_sign(right.0);


        let m1 = prepend_leading_one(left.2, left.1);
        let m2 = prepend_leading_one(right.2, right.1);

        let (m1, m2, common_exponent) = adjust_mantissa((m1, left.1), (m2, right.1));



        let new_mantissa = s1 * m1 as i32 + s2 * m2 as i32;

        let new_sign = (new_mantissa >> 31) as u32;

        let new_mantissa = new_mantissa.unsigned_abs();


        let exponent = if common_exponent == 0 {
            0
        } else {
            (common_exponent as i32 + required_bit_shifts(new_mantissa)) as u32
        };


        let mantissa = if exponent == 0 {
            new_mantissa
        } else {
            normalize_mantissa(new_mantissa)
        };

        let bits = (new_sign << 31) | (exponent << 23) | (mantissa & !(1 << 23));

        Self::from_bits(bits)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::from_bits(value.to_bits())
    }
}

impl From<Float> for f32 {
    fn from(value: Float) -> Self {
        f32::from_bits(value.0)
    }
}