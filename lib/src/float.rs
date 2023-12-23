use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
};

use rug::Float as RugFloat;

#[derive(Clone, PartialEq)]
pub struct Float(RugFloat);

impl Float {
    pub(crate) fn from_parts(
        sign: bool,
        integral: &str,
        fractional: &str,
        exp_sign: bool,
        exp: &str,
    ) -> Self {
        const LOG_2_10: f64 = 3.3219280948873626;
        let sign = if sign { "+" } else { "-" };
        let exp = if exp.is_empty() {
            "".to_owned()
        } else {
            let exp_sign = if exp_sign { "+" } else { "-" };
            format!("e{exp_sign}{exp}")
        };
        let s = format!("{sign}{integral}.{fractional}{exp}");
        let f = RugFloat::parse(s).unwrap();
        let sig_int = integral.trim_start_matches('0');
        let sig_frac = if sig_int.is_empty() {
            fractional.trim_start_matches('0')
        } else {
            fractional
        };
        let mut sig_len = sig_int.len() + sig_frac.len();
        if sig_len == 0 {
            sig_len = 1;
        }
        let precision = (sig_len as f64 * LOG_2_10).ceil() as u32;
        let f = RugFloat::with_val(precision, f);
        Float(f)
    }
}

impl Eq for Float {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ord().hash(state);
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const LOG_10_2: f64 = 0.30102999566398114;
        let sig_len = (self.0.prec() as f64 * LOG_10_2).floor() as usize;
        let (sign, num, exp) = self.0.to_sign_string_exp(10, Some(sig_len));
        let sign = if sign { "-" } else { "+" };
        let exp = exp.map_or("".to_owned(), |i| format!("e{i}"));
        let s = format!("{sign}0.{num}{exp}");
        write!(f, "{s}")
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(self, f)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float(RugFloat::with_val(24, value))
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(RugFloat::with_val(53, value))
    }
}
