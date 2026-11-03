//! Numbers held the way the Python reading holds them.
//!
//! The Python JSON reading classifies a number from its raw
//! literal. An integer literal becomes an arbitrary precision
//! integer, and a literal with a fraction or an exponent becomes
//! a double. The readings depend on that classification, so the
//! port holds the same shapes in the Num type and never lets one
//! become the other. A third shape stands for the exact decimal
//! reading of a float literal, which only the makers' exact parse
//! of an envelope produces, as the Python makers read a cost
//! figure with the decimal type standing for every float.

use num_bigint::BigInt;
use num_rational::Ratio;
use std::cmp::Ordering;

use crate::pydecimal::PyDecimal;

/// A recorded number, as the Python reading classified its
/// literal.
#[derive(Clone, Debug)]
pub enum Num {
    Int(BigInt),
    Float(f64),
    Decimal(PyDecimal),
}

/// The bound an integer must sit under in absolute value to
/// convert to a finite double, probed against the Python
/// reading. The largest finite double is 2^1024 - 2^971, the
/// midpoint between it and 2^1024 is 2^1024 - 2^970, and an
/// integer at or past that midpoint rounds to infinity.
fn finite_int_bound() -> BigInt {
    (BigInt::from(1) << 1024) - (BigInt::from(1) << 970)
}

impl Num {
    pub fn from_i64(value: i64) -> Num {
        Num::Int(BigInt::from(value))
    }

    pub fn from_f64(value: f64) -> Num {
        Num::Float(value)
    }

    /// Return whether this number is an integer, the test the
    /// seat readings apply to the confidence and the
    /// probabilities. A boolean never reaches this type, because
    /// the JSON reading holds it apart, as the Python reading
    /// does.
    pub fn is_int(&self) -> bool {
        matches!(self, Num::Int(_))
    }

    /// Return whether this number counts as a number at all for
    /// the seat readings, an integer, a double, or the exact
    /// decimal of the makers' parse, and never a boolean, which
    /// the JSON reading holds apart.
    pub fn is_number(&self) -> bool {
        matches!(self, Num::Int(_) | Num::Float(_) | Num::Decimal(_))
    }

    /// Return whether this number is finite, without ever
    /// failing. An integer too large to convert to a double
    /// counts as not finite. The Python finiteness test raises
    /// an overflow error on such an integer, and PORT-4 in
    /// CONTRACT.md states the boundary. The recorded outcome is
    /// the same on both sides, no response stands.
    pub fn is_finite_number(&self) -> bool {
        match self {
            Num::Int(value) => {
                let bound = finite_int_bound();
                *value < bound && *value > -bound
            }
            Num::Float(value) => value.is_finite(),
            Num::Decimal(_) => true,
        }
    }

    /// Return the exact fraction this number names. A double
    /// converts to the fraction of its exact binary value, a
    /// decimal to the fraction of its exact decimal value, and
    /// an integer to itself, so comparisons built on this never
    /// round. Every caller passes a finite number, as the Python
    /// code does, because the readings refuse non finite figures
    /// before any reading reaches this.
    pub fn to_fraction(&self) -> Ratio<BigInt> {
        match self {
            Num::Int(value) => Ratio::from(value.clone()),
            Num::Float(value) => float_to_fraction(*value),
            Num::Decimal(value) => value.to_fraction(),
        }
    }

    /// Return whether this number is below a small integer,
    /// compared exactly, the shape of the confidence range test.
    pub fn is_below_int(&self, bound: i64) -> bool {
        match self {
            Num::Int(value) => *value < BigInt::from(bound),
            Num::Float(value) => *value < bound as f64,
            Num::Decimal(value) => value.is_below_int(bound),
        }
    }

    /// Return whether this number is above a small integer,
    /// compared exactly, the shape of the confidence range test.
    pub fn is_above_int(&self, bound: i64) -> bool {
        match self {
            Num::Int(value) => *value > BigInt::from(bound),
            Num::Float(value) => *value > bound as f64,
            Num::Decimal(value) => value.is_above_int(bound),
        }
    }

    /// Return the text the Python reading prints for this number
    /// where it renders one plainly, the integer as its digits,
    /// the double as its shortest round trip text, and the
    /// decimal in the decimal text form.
    pub fn plain_text(&self) -> String {
        match self {
            Num::Int(value) => value.to_string(),
            Num::Float(value) => py_float_repr(*value),
            Num::Decimal(value) => value.render(),
        }
    }

    /// Add two recorded numbers the way the Python figures add,
    /// an integer sum staying an integer and a sum touching a
    /// double becoming a double. The run record counting reads
    /// this for the usage figures of the envelopes.
    pub fn added(&self, other: &Num) -> Num {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            _ => Num::Float(self.as_f64() + other.as_f64()),
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Num::Int(value) => {
                use num_traits::ToPrimitive;
                value.to_f64().unwrap_or(f64::INFINITY)
            }
            Num::Float(value) => *value,
            Num::Decimal(value) => value.render().parse().unwrap_or(f64::INFINITY),
        }
    }
}

impl PartialEq for Num {
    /// Python number equality, exact across the shapes. An
    /// integer equals a double when their exact values are
    /// equal, and non finite doubles equal only themselves of
    /// their kind.
    fn eq(&self, other: &Num) -> bool {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => a == b,
            (Num::Float(a), Num::Float(b)) => a == b,
            (Num::Int(a), Num::Float(b)) | (Num::Float(b), Num::Int(a)) => {
                if !b.is_finite() {
                    return false;
                }
                float_to_fraction(*b) == Ratio::from(a.clone())
            }
            (Num::Decimal(a), Num::Decimal(b)) => a == b,
            (Num::Decimal(a), Num::Int(b)) | (Num::Int(b), Num::Decimal(a)) => {
                a.to_fraction() == Ratio::from(b.clone())
            }
            (Num::Decimal(a), Num::Float(b)) | (Num::Float(b), Num::Decimal(a)) => {
                if !b.is_finite() {
                    return false;
                }
                a.to_fraction() == float_to_fraction(*b)
            }
        }
    }
}

/// Convert a finite double to the fraction of its exact binary
/// value.
pub fn float_to_fraction(value: f64) -> Ratio<BigInt> {
    assert!(value.is_finite(), "a non finite figure has no fraction");
    if value == 0.0 {
        return Ratio::from(BigInt::from(0));
    }
    let bits = value.to_bits();
    let negative = bits >> 63 == 1;
    let biased = ((bits >> 52) & 0x7FF) as i64;
    let stored = bits & ((1u64 << 52) - 1);
    let (mantissa, exponent) = if biased == 0 {
        (stored, -1022 - 52)
    } else {
        (stored | (1u64 << 52), biased - 1023 - 52)
    };
    let mut numerator = BigInt::from(mantissa);
    if negative {
        numerator = -numerator;
    }
    if exponent >= 0 {
        Ratio::from(numerator << (exponent as usize))
    } else {
        Ratio::new(numerator, BigInt::from(1) << ((-exponent) as usize))
    }
}

/// Render a double the way the Python repr renders it, the
/// shortest decimal text that parses back to the same number,
/// in plain notation inside the stated exponent range and in
/// scientific notation outside it.
pub fn py_float_repr(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    if value == 0.0 {
        return if value.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    let negative = value < 0.0;
    // The shortest digits and the decimal exponent, taken from
    // the scientific rendering, whose digit sequence is the same
    // shortest round trip sequence the Python repr computes.
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent_text) = scientific
        .split_once('e')
        .expect("the scientific form carries an exponent");
    let exponent: i64 = exponent_text.parse().expect("the exponent is a number");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    // The value is 0.digits times ten to the decpt power.
    let decpt = exponent + 1;
    let body = if decpt <= -4 || decpt > 16 {
        // Scientific notation, one digit before the point and
        // the exponent signed and padded to two digits.
        let mut out = String::new();
        out.push(digits.as_bytes()[0] as char);
        if digits.len() > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        let shown = decpt - 1;
        out.push(if shown < 0 { '-' } else { '+' });
        let magnitude = shown.abs();
        if magnitude < 10 {
            out.push('0');
        }
        out.push_str(&magnitude.to_string());
        out
    } else if decpt <= 0 {
        let mut out = String::from("0.");
        for _ in 0..(-decpt) {
            out.push('0');
        }
        out.push_str(&digits);
        out
    } else if (decpt as usize) >= digits.len() {
        let mut out = digits.clone();
        for _ in 0..(decpt as usize - digits.len()) {
            out.push('0');
        }
        out.push_str(".0");
        out
    } else {
        let split = decpt as usize;
        format!("{}.{}", &digits[..split], &digits[split..])
    };
    if negative {
        format!("-{}", body)
    } else {
        body
    }
}

/// Compare a recorded number against a small integer bound
/// exactly, the shape of the confidence range test.
pub fn cmp_int(num: &Num, bound: i64) -> Ordering {
    match num {
        Num::Int(value) => value.cmp(&BigInt::from(bound)),
        Num::Float(value) => value
            .partial_cmp(&(bound as f64))
            .unwrap_or(Ordering::Equal),
        Num::Decimal(value) => value.cmp_int(bound),
    }
}
