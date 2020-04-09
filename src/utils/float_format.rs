pub trait FloatFormat {
  fn fmt_with_decimal(self, force: bool) -> String;
}

fn trailing_zeros(n: i16) -> String {
  if n <= 0 {
    String::new()
  } else {
    (1..n).fold(String::from("0"), |acc, _| acc + "0")
  }
}

impl FloatFormat for (bool, u64, i16) {
  fn fmt_with_decimal(self, force: bool) -> String {
    let (positive, mantissa, exponent) = self;
    let natural: u64 = ((mantissa as f64) * 10_f64.powi(exponent as i32)) as u64;
    let decimal = if exponent < 0 {
      let decimal = format!("{}", (mantissa - (natural * 10_u64.pow(-exponent as u32))));
      let zeros = trailing_zeros(exponent.abs() - (decimal.len() as i16));
      format!(".{}{}", zeros, decimal)
    } else if force {
      String::from(".0")
    } else {
      String::new()
    };
    let sign = if positive { "" } else { "-" };

    format!("{}{}{}", sign, natural, decimal)
  }
}

impl FloatFormat for f64 {
  fn fmt_with_decimal(self, force: bool) -> String {
    json::number::Number::from(self)
      .as_parts()
      .fmt_with_decimal(force)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  pub fn num_parts_force() {
    assert_eq!((true, 0, 0).fmt_with_decimal(true), "0.0");
    assert_eq!((true, 123456, -3).fmt_with_decimal(true), "123.456");
    assert_eq!((false, 987654, -3).fmt_with_decimal(true), "-987.654");
    assert_eq!((true, 1, -8).fmt_with_decimal(true), "0.00000001");
    assert_eq!((false, 1, -8).fmt_with_decimal(true), "-0.00000001");
    assert_eq!((true, 1, 2).fmt_with_decimal(true), "100.0");
  }

  #[test]
  pub fn num_parts() {
    assert_eq!((true, 0, 0).fmt_with_decimal(false), "0");
    assert_eq!((true, 123456, -3).fmt_with_decimal(false), "123.456");
    assert_eq!((false, 987654, -3).fmt_with_decimal(false), "-987.654");
    assert_eq!((true, 1, -8).fmt_with_decimal(false), "0.00000001");
    assert_eq!((false, 1, -8).fmt_with_decimal(false), "-0.00000001");
    assert_eq!((true, 1, 2).fmt_with_decimal(false), "100");
  }

  #[test]
  pub fn f64_force() {
    assert_eq!((0.0).fmt_with_decimal(true), "0.0");
    assert_eq!((123.456).fmt_with_decimal(true), "123.456");
    assert_eq!((-987.654).fmt_with_decimal(true), "-987.654");
    assert_eq!((0.00000001).fmt_with_decimal(true), "0.00000001");
    assert_eq!((-0.00000001).fmt_with_decimal(true), "-0.00000001");
    assert_eq!((100.0).fmt_with_decimal(true), "100.0");
  }

  #[test]
  pub fn f64() {
    assert_eq!((0.0).fmt_with_decimal(false), "0");
    assert_eq!((123.456).fmt_with_decimal(false), "123.456");
    assert_eq!((-987.654).fmt_with_decimal(false), "-987.654");
    assert_eq!((0.00000001).fmt_with_decimal(false), "0.00000001");
    assert_eq!((-0.00000001).fmt_with_decimal(false), "-0.00000001");
    assert_eq!((100.0).fmt_with_decimal(false), "100");
  }

  #[test]
  pub fn trailing_zeros() {
    assert_eq!(super::trailing_zeros(0), String::from(""));
    assert_eq!(super::trailing_zeros(1), String::from("0"));
    assert_eq!(super::trailing_zeros(10), String::from("0000000000"));
    assert_eq!(super::trailing_zeros(-10), String::from(""));
  }
}
