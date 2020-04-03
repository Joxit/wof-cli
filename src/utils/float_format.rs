pub trait FloatFormat {
  fn fmt_with_decimal(self) -> String;
}

fn trailing_zeros(n: i16) -> String {
  if n <= 0 {
    String::new()
  } else {
    (1..n).fold(String::from("0"), |acc, _| acc + "0")
  }
}

impl FloatFormat for (bool, u64, i16) {
  fn fmt_with_decimal(self) -> String {
    let (positive, mantissa, exponent) = self;
    let natural: u64 = ((mantissa as f64) * 10_f64.powi(exponent as i32)) as u64;
    let decimal = if exponent < 0 {
      let decimal = format!("{}", (mantissa - (natural * 10_u64.pow(-exponent as u32))));
      let zeros = trailing_zeros(exponent.abs() - (decimal.len() as i16));
      format!(".{}{}", zeros, decimal)
    } else {
      String::from(".0")
    };
    let sign = if positive { "" } else { "-" };

    format!("{}{}{}", sign, natural, decimal)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  pub fn num_parts() {
    assert_eq!((true, 0, 0).fmt_with_decimal(), "0.0");
    assert_eq!((true, 123456, -3).fmt_with_decimal(), "123.456");
    assert_eq!((false, 987654, -3).fmt_with_decimal(), "-987.654");
    assert_eq!((true, 1, -8).fmt_with_decimal(), "0.00000001");
    assert_eq!((false, 1, -8).fmt_with_decimal(), "-0.00000001");
  }

  #[test]
  pub fn trailing_zeros() {
    assert_eq!(super::trailing_zeros(0), String::from(""));
    assert_eq!(super::trailing_zeros(1), String::from("0"));
    assert_eq!(super::trailing_zeros(10), String::from("0000000000"));
    assert_eq!(super::trailing_zeros(-10), String::from(""));
  }
}
