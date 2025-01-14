use crate::{
    fp_decimal::FPDecimal,
    scale::{Scaled, DEC_SCALE_FACTOR},
};
use std::fmt;

impl fmt::Display for FPDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.sign == 0 { "-" } else { "" };
        let integer = self.int().abs();
        let fraction = (FPDecimal::_fraction(*self)).abs();

        if fraction == FPDecimal::ZERO {
            write!(f, "{}{}", sign, integer.num / FPDecimal::ONE.num)
        } else {
            let fraction_string = fraction.num.to_string(); //
            let fraction_string = "0".repeat(FPDecimal::DIGITS - fraction_string.len()) + &fraction_string;
            let integer_num = integer.num / FPDecimal::ONE.num;
            f.write_str(sign)?;
            f.write_str(&integer_num.to_string())?;
            f.write_str(".")?;
            f.write_str(fraction_string.trim_end_matches('0'))?;

            Ok(())
        }
    }
}

pub trait ToProto {
    fn to_proto_string(self) -> String;
}

impl ToProto for FPDecimal {
    fn to_proto_string(self) -> String {
        self.scaled(DEC_SCALE_FACTOR).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::FPDecimal;

    #[test]
    fn test_fmt_sub() {
        let input: FPDecimal = FPDecimal::ONE + FPDecimal::from(3u128).div(100i128);
        assert_eq!(&format!("{input}"), "1.03");
    }

    #[test]
    fn test_fmt() {
        assert_eq!(&format!("{}", FPDecimal::LN_1_5), "0.405465108108164382");
    }

    #[test]
    fn test_fmt_neg() {
        assert_eq!(&format!("{}", -FPDecimal::FIVE), "-5");
    }
}
