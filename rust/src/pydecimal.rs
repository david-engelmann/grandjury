//! An exact decimal figure, the shape the price reading and
//! the run record sums run on.
//!
//! The value is a coefficient times a power of ten, held
//! exactly. The operations the port uses are the ones the
//! Python decimal type performs on its surfaces. Construction
//! from a literal is exact and refuses past the probed bounds
//! of PORT-4 in CONTRACT.md. Addition runs under a context
//! precision with a stated rounding. Quantize rounds to a
//! stated exponent under a context precision and refuses when
//! the result would pass it, as the Python quantize refuses.
//! Division runs at the default context precision with halves
//! rounding to even, and an exact quotient takes the preferred
//! exponent of the operands when its digits fit the precision,
//! the shape the Python division shows. Rendering is the text
//! form the Python decimal type renders, in scientific form
//! when the stored exponent is positive or the adjusted
//! exponent falls below minus six, and in plain form otherwise.

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Signed;

/// The rounding rule of a context operation.
#[derive(Clone, Copy, PartialEq)]
pub enum Rounding {
    HalfUp,
    HalfEven,
}

/// The largest adjusted exponent a constructed figure may
/// hold, probed against the Python decimal constructor.
const MAX_ADJUSTED: i64 = 999999999999999999;

/// The smallest stored exponent a constructed figure may
/// hold, probed against the Python decimal constructor.
const MIN_STORED_EXPONENT: i64 = -1999999999999999997;

/// A power of ten as an integer, materialized however wide
/// the exponent stands. PORT-11 in CONTRACT.md states that
/// no materialization bound stands in the port, because a
/// BigInt coefficient grows in proportion to the figure it
/// holds, so a figure the Python surface computes and prints
/// is computed and printed here at its full width. The
/// failures on this surface stand in the cost counter and
/// not in a width here. The counter refuses a working
/// precision past the Python context bound in polls.rs
/// before any sum runs, and inside that bound it attempts
/// the fallible coefficient reservation the PORT-11
/// amendment states before any sum runs, so a width the
/// machine cannot materialize never reaches this path from
/// the counter at all.
fn pow10(exponent: i64) -> BigInt {
    let mut result = BigInt::from(1);
    let mut base = BigInt::from(10);
    let mut rest = exponent.unsigned_abs();
    while rest > 0 {
        if rest & 1 == 1 {
            result *= &base;
        }
        rest >>= 1;
        if rest > 0 {
            base = &base * &base;
        }
    }
    result
}

/// An exact decimal figure.
#[derive(Clone, Debug)]
pub struct PyDecimal {
    coefficient: BigInt,
    exponent: i64,
}

impl PyDecimal {
    pub fn zero() -> PyDecimal {
        PyDecimal {
            coefficient: BigInt::from(0),
            exponent: 0,
        }
    }

    pub fn from_bigint(value: BigInt) -> PyDecimal {
        PyDecimal {
            coefficient: value,
            exponent: 0,
        }
    }

    /// Parse the text of a decimal literal the way the Python
    /// decimal constructor reads it, exactly, or None where
    /// the constructor raises. The text arrives already shaped
    /// by the price grammar of GJ-4 or by the JSON number
    /// reading, an optional minus sign, digits with an optional
    /// point, and an optional exponent. The constructor refuses
    /// a figure whose adjusted exponent stands above the probed
    /// bound or whose stored exponent stands below it, and the
    /// refusal is what the price loader names with the not a
    /// decimal string defect.
    pub fn parse_literal(text: &str) -> Option<PyDecimal> {
        let bytes = text.as_bytes();
        let mut index = 0;
        let mut negative = false;
        if index < bytes.len() && bytes[index] == b'-' {
            negative = true;
            index += 1;
        }
        let mut digits: Vec<u8> = Vec::new();
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            digits.push(bytes[index]);
            index += 1;
        }
        let whole = digits.len();
        let mut fraction = 0usize;
        if index < bytes.len() && bytes[index] == b'.' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                digits.push(bytes[index]);
                fraction += 1;
                index += 1;
            }
        }
        if whole + fraction == 0 {
            return None;
        }
        let mut exponent_value: i64 = 0;
        if index < bytes.len() && (bytes[index] == b'e' || bytes[index] == b'E') {
            index += 1;
            let mut exponent_negative = false;
            if index < bytes.len() && (bytes[index] == b'+' || bytes[index] == b'-') {
                exponent_negative = bytes[index] == b'-';
                index += 1;
            }
            let mut seen = 0usize;
            let mut magnitude: i64 = 0;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                magnitude = magnitude
                    .saturating_mul(10)
                    .saturating_add((bytes[index] - b'0') as i64)
                    .min(4_000_000_000_000_000_000);
                seen += 1;
                index += 1;
            }
            if seen == 0 {
                return None;
            }
            exponent_value = if exponent_negative {
                -magnitude
            } else {
                magnitude
            };
        }
        if index != bytes.len() {
            return None;
        }
        let mut coefficient = BigInt::parse_bytes(&digits, 10)?;
        if negative {
            coefficient = -coefficient;
        }
        let stored = exponent_value - fraction as i64;
        let value = PyDecimal {
            coefficient,
            exponent: stored,
        };
        if value.adjusted() > MAX_ADJUSTED || stored < MIN_STORED_EXPONENT {
            return None;
        }
        Some(value)
    }

    /// The stored exponent, the figure the Python tuple reading
    /// reports.
    pub fn exponent(&self) -> i64 {
        self.exponent
    }

    /// The count of digits in the coefficient, the width the
    /// Python decimal context measures a figure by. A zero
    /// coefficient holds one digit, as the Python tuple of a
    /// zero figure holds one zero digit.
    pub fn coefficient_digits(&self) -> usize {
        coefficient_digits_of(&self.coefficient)
    }

    /// The adjusted exponent, the exponent of the most
    /// significant digit, as the Python reading reports it.
    pub fn adjusted(&self) -> i64 {
        self.exponent + self.coefficient_digits() as i64 - 1
    }

    /// Return whether the figure is below zero. A negative zero
    /// is not below zero, as the Python comparison holds.
    pub fn is_negative(&self) -> bool {
        self.coefficient < BigInt::from(0)
    }

    /// Return the exact fraction this figure names.
    pub fn to_fraction(&self) -> Ratio<BigInt> {
        if self.exponent >= 0 {
            Ratio::from(&self.coefficient * pow10(self.exponent))
        } else {
            Ratio::new(self.coefficient.clone(), pow10(-self.exponent))
        }
    }

    /// The canonical shape of the figure, its coefficient with
    /// trailing zeros folded into the exponent, and a zero
    /// figure at exponent zero, so two figures are equal
    /// exactly when their canonical shapes agree, as the
    /// Python value equality holds across spellings.
    fn canonical(&self) -> (BigInt, i64) {
        if self.coefficient == BigInt::from(0) {
            return (BigInt::from(0), 0);
        }
        let mut coefficient = self.coefficient.clone();
        let mut exponent = self.exponent;
        let ten = BigInt::from(10);
        while &coefficient % &ten == BigInt::from(0) {
            coefficient /= &ten;
            exponent += 1;
        }
        (coefficient, exponent)
    }

    /// Compare against a small integer exactly.
    pub fn cmp_int(&self, bound: i64) -> std::cmp::Ordering {
        let other = PyDecimal::from_bigint(BigInt::from(bound));
        self.cmp_decimal(&other)
    }

    pub fn is_below_int(&self, bound: i64) -> bool {
        self.cmp_int(bound) == std::cmp::Ordering::Less
    }

    pub fn is_above_int(&self, bound: i64) -> bool {
        self.cmp_int(bound) == std::cmp::Ordering::Greater
    }

    /// Compare two figures exactly. When the adjusted exponents
    /// differ the comparison is settled there. When they agree
    /// the exponent gap is the digit gap, which the figures
    /// themselves bound, so the scaling is materializable.
    pub fn cmp_decimal(&self, other: &PyDecimal) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let mine = self.coefficient.cmp(&BigInt::from(0));
        let theirs = other.coefficient.cmp(&BigInt::from(0));
        if mine != theirs {
            return mine.cmp(&theirs);
        }
        if mine == Ordering::Equal {
            return Ordering::Equal;
        }
        match self.adjusted().cmp(&other.adjusted()) {
            Ordering::Equal => {}
            order => {
                return if mine == Ordering::Less {
                    order.reverse()
                } else {
                    order
                };
            }
        }
        let exponent = self.exponent.min(other.exponent);
        let a = &self.coefficient * pow10(self.exponent - exponent);
        let b = &other.coefficient * pow10(other.exponent - exponent);
        a.cmp(&b)
    }

    /// Add exactly, the sum holding the lesser exponent of the
    /// two figures, trailing zeros standing as the Python sum
    /// leaves them.
    pub fn added_exact(&self, other: &PyDecimal) -> PyDecimal {
        let exponent = self.exponent.min(other.exponent);
        let mine = &self.coefficient * pow10(self.exponent - exponent);
        let theirs = &other.coefficient * pow10(other.exponent - exponent);
        PyDecimal {
            coefficient: mine + theirs,
            exponent,
        }
    }

    /// Round to a count of significant digits under the stated
    /// rule, the operation a context performs on a result.
    pub fn rounded_to_prec(&self, prec: usize, mode: Rounding) -> PyDecimal {
        let digits = self.coefficient_digits();
        if digits <= prec {
            return self.clone();
        }
        let drop = (digits - prec) as i64;
        let rounded = round_div_pow10(&self.coefficient, drop, mode);
        let mut coefficient = rounded;
        let mut exponent = self.exponent + drop;
        if coefficient_digits_of(&coefficient) > prec {
            // The rounding carried into a new digit, so the
            // figure shifts once more, exactly as the Python
            // context leaves a carried result one digit wide
            // at the precision.
            coefficient /= BigInt::from(10);
            exponent += 1;
        }
        PyDecimal {
            coefficient,
            exponent,
        }
    }

    /// Add under a context precision with a stated rounding,
    /// the operation the cost counter and the run record sums
    /// perform. Where one operand sits far below the rounding
    /// position of the other, the sum rounds from the dominant
    /// operand alone, because the lesser cannot reach the last
    /// digit the context keeps.
    pub fn added_ctx(&self, other: &PyDecimal, prec: usize, mode: Rounding) -> PyDecimal {
        let gap = (self.adjusted() - other.adjusted()).abs();
        if gap > prec as i64 + 64 {
            let dominant = if self.adjusted() >= other.adjusted() {
                self
            } else {
                other
            };
            return dominant.rounded_to_prec(prec, mode);
        }
        self.added_exact(other).rounded_to_prec(prec, mode)
    }

    /// Round to the stated exponent under a context precision
    /// and the stated rule, or None where the Python quantize
    /// raises because the result would hold more digits than
    /// the precision allows.
    pub fn quantized(&self, target: i64, prec: usize, mode: Rounding) -> Option<PyDecimal> {
        if self.exponent >= target {
            let widen = self.exponent - target;
            if self.coefficient_digits() as i64 + widen > prec as i64 {
                return None;
            }
            return Some(PyDecimal {
                coefficient: &self.coefficient * pow10(widen),
                exponent: target,
            });
        }
        let drop = target - self.exponent;
        let coefficient = round_div_pow10(&self.coefficient, drop, mode);
        if coefficient_digits_of(&coefficient) > prec {
            return None;
        }
        Some(PyDecimal {
            coefficient,
            exponent: target,
        })
    }

    /// Round to six decimal places, an exact half rounding
    /// away from zero, under the stated context precision, the
    /// rendering the cost counter and the price recomputation
    /// share.
    pub fn quantized_six(&self, prec: usize) -> Option<PyDecimal> {
        self.quantized(-6, prec, Rounding::HalfUp)
    }

    /// Divide under the default context, precision 28 with
    /// halves rounding to even, or None where the Python
    /// division raises on a zero divisor. An exact quotient
    /// takes the preferred exponent of the operands, the
    /// difference of their stored exponents, scaled up with
    /// trailing zeros while its digits fit the precision, and
    /// a quotient that does not terminate at the precision
    /// rounds at 28 significant digits.
    pub fn divided_default(&self, other: &PyDecimal) -> Option<PyDecimal> {
        const PREC: usize = 28;
        if other.coefficient == BigInt::from(0) {
            return None;
        }
        let preferred = self.exponent - other.exponent;
        if self.coefficient == BigInt::from(0) {
            return Some(PyDecimal {
                coefficient: BigInt::from(0),
                exponent: preferred,
            });
        }
        let negative =
            (self.coefficient < BigInt::from(0)) != (other.coefficient < BigInt::from(0));
        let a = self.coefficient.abs();
        let b = other.coefficient.abs();
        // The quotient is the ratio of the coefficients scaled
        // by the difference of the stored exponents, and that
        // difference rides on the exponent of the result alone,
        // so no operation here materializes a power of ten at
        // the width of the difference, however wide it stands.
        let diff = self.exponent - other.exponent;
        // The exact quotient, when it terminates, in its
        // shortest coefficient.
        if let Some((mut coefficient, mut exponent)) = exact_quotient(&a, &b) {
            exponent += diff;
            while coefficient != BigInt::from(0)
                && &coefficient % BigInt::from(10) == BigInt::from(0)
            {
                coefficient /= BigInt::from(10);
                exponent += 1;
            }
            if coefficient_digits_of(&coefficient) <= PREC {
                while exponent > preferred && coefficient_digits_of(&coefficient) < PREC {
                    coefficient *= BigInt::from(10);
                    exponent -= 1;
                }
                return Some(PyDecimal {
                    coefficient: if negative { -coefficient } else { coefficient },
                    exponent,
                });
            }
        }
        // Rounded at the precision, halves to even. The
        // adjusted exponent of the coefficient ratio is settled
        // from the digit counts of the coefficients alone, and
        // the scaling that rounds the ratio at the precision is
        // bounded by those counts, with the exponent difference
        // entering only through the adjusted exponent it names.
        let dn = coefficient_digits_of(&a) as i64;
        let dd = coefficient_digits_of(&b) as i64;
        let mut adjusted = dn - dd;
        {
            let order = if adjusted >= 0 {
                a.cmp(&(&b * pow10(adjusted)))
            } else {
                (&a * pow10(-adjusted)).cmp(&b)
            };
            if order == std::cmp::Ordering::Less {
                adjusted -= 1;
            }
        }
        adjusted += diff;
        let scale = PREC as i64 - 1 - (adjusted - diff);
        let (num, den) = if scale >= 0 {
            (&a * pow10(scale), b.clone())
        } else {
            (a.clone(), &b * pow10(-scale))
        };
        let mut quotient = &num / &den;
        let remainder = &num % &den;
        let doubled = &remainder * 2;
        if doubled > den || (doubled == den && &quotient % BigInt::from(2) == BigInt::from(1)) {
            quotient += 1;
        }
        if coefficient_digits_of(&quotient) > PREC {
            quotient /= BigInt::from(10);
            adjusted += 1;
        }
        Some(PyDecimal {
            coefficient: if negative { -quotient } else { quotient },
            exponent: adjusted - PREC as i64 + 1,
        })
    }

    /// Render the figure the way the Python decimal type
    /// renders it, in scientific form when the stored exponent
    /// is positive or the adjusted exponent falls below minus
    /// six, and in plain form otherwise. The exponent of the
    /// scientific form carries its sign and no padding.
    pub fn render(&self) -> String {
        let negative = self.coefficient < BigInt::from(0);
        let digits = self.coefficient.abs().to_string();
        let exponent = self.exponent;
        let adjusted = exponent + digits.len() as i64 - 1;
        let body = if exponent > 0 || adjusted < -6 {
            let mut out = String::new();
            out.push_str(&digits[..1]);
            if digits.len() > 1 {
                out.push('.');
                out.push_str(&digits[1..]);
            }
            out.push('E');
            out.push(if adjusted < 0 { '-' } else { '+' });
            out.push_str(&adjusted.abs().to_string());
            out
        } else {
            let point = digits.len() as i64 + exponent;
            if point <= 0 {
                format!("0.{}{}", "0".repeat((-point) as usize), digits)
            } else if (point as usize) >= digits.len() {
                format!("{}{}", digits, "0".repeat((point as usize) - digits.len()))
            } else {
                format!(
                    "{}.{}",
                    &digits[..point as usize],
                    &digits[point as usize..]
                )
            }
        };
        if negative {
            format!("-{}", body)
        } else {
            body
        }
    }
}

impl PartialEq for PyDecimal {
    /// Python decimal equality, by value across spellings.
    fn eq(&self, other: &PyDecimal) -> bool {
        self.canonical() == other.canonical()
    }
}

/// The count of decimal digits in a coefficient, settled
/// exactly without rendering the figure. The estimate from
/// the bit width lands within a digit of the truth, and the
/// two comparisons against powers of ten settle which side
/// it stands on, so a coefficient of millions of digits costs
/// two power computations and no decimal rendering.
fn coefficient_digits_of(coefficient: &BigInt) -> usize {
    use num_traits::Zero;
    if coefficient.is_zero() {
        return 1;
    }
    let magnitude = coefficient.abs();
    let bits = magnitude.bits();
    let mut digits = ((bits as f64 * std::f64::consts::LOG10_2) as usize) + 1;
    let mut lower = pow10(digits as i64 - 1);
    while magnitude < lower {
        digits -= 1;
        lower = pow10(digits as i64 - 1);
    }
    while magnitude >= &lower * BigInt::from(10) {
        digits += 1;
        lower *= BigInt::from(10);
    }
    digits
}

/// Divide a signed coefficient by a power of ten under the
/// stated rounding, the rounding reading the remainder
/// against the divisor on the magnitude, as both of the
/// Python rounding rules in play read a negative figure by
/// its magnitude.
fn round_div_pow10(coefficient: &BigInt, drop: i64, mode: Rounding) -> BigInt {
    if drop <= 0 {
        return coefficient.clone();
    }
    let magnitude = coefficient.abs();
    if drop > coefficient_digits_of(&magnitude) as i64 {
        // The divisor dwarfs the figure, which rounds to zero
        // under either rule, because twice the figure cannot
        // reach the divisor.
        return BigInt::from(0);
    }
    let divisor = pow10(drop);
    let mut quotient = &magnitude / &divisor;
    let remainder = &magnitude % &divisor;
    let doubled = &remainder * 2;
    let up = match mode {
        Rounding::HalfUp => doubled >= divisor,
        Rounding::HalfEven => {
            doubled > divisor || (doubled == divisor && &quotient % 2 == BigInt::from(1))
        }
    };
    if up {
        quotient += 1;
    }
    if *coefficient < BigInt::from(0) {
        -quotient
    } else {
        quotient
    }
}

/// Return the exact quotient of two positive integers as a
/// decimal coefficient and exponent when the quotient
/// terminates, or None when it does not.
fn exact_quotient(big_n: &BigInt, big_d: &BigInt) -> Option<(BigInt, i64)> {
    let gcd = num_integer_gcd(big_n, big_d);
    let n = big_n / &gcd;
    let mut d = big_d / &gcd;
    let two = BigInt::from(2);
    let five = BigInt::from(5);
    let mut twos = 0i64;
    while &d % &two == BigInt::from(0) {
        d /= &two;
        twos += 1;
    }
    let mut fives = 0i64;
    while &d % &five == BigInt::from(0) {
        d /= &five;
        fives += 1;
    }
    if d != BigInt::from(1) {
        return None;
    }
    let places = twos.max(fives);
    let mut factor = BigInt::from(1);
    for _ in 0..(places - twos) {
        factor *= &two;
    }
    for _ in 0..(places - fives) {
        factor *= &five;
    }
    Some((&n * factor, -places))
}

fn num_integer_gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != BigInt::from(0) {
        let next = &a % &b;
        a = b;
        b = next;
    }
    a
}
