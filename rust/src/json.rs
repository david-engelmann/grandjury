//! The JSON reading, mirroring the Python json module.
//!
//! The loaders of this repo rest on the semantics of the Python
//! reading, so the port carries its own reading here, as PORT-3
//! in CONTRACT.md states. A repeated object key keeps its last
//! value at its first position, as a Python dict holds it. A
//! number is classified from its raw literal, an integer literal
//! becoming an exact integer and a literal with a fraction or an
//! exponent becoming a double, with the integer digit limit the
//! Python reading enforces. The NaN and Infinity literals the
//! Python reading accepts parse to the doubles they name. A
//! string is a sequence of code units, so an unpaired surrogate
//! is representable. Input nested past the depth the Python
//! command line reading accepts is refused. A second reading,
//! the exact reading, stands for the makers, which read a float
//! literal as the exact decimal it names, as the Python makers
//! read with the decimal type standing for every float.

use num_bigint::BigInt;

use crate::pydecimal::PyDecimal;
use crate::pynum::{py_float_repr, Num};
use crate::pytext::PText;

/// The most open containers the reading accepts at once.
///
/// The Python reading raises on nesting past a depth its own
/// call stack sets, and the depth moves with the caller, so the
/// figure is the one measured through the actual command line
/// entry point the binary mirrors, as PORT-3 states. Probed on
/// this machine by driving the installed entry point in a fresh
/// process over a poll file whose ignored field nests arrays
/// inside the root object, 9996 nested arrays load and 9997
/// draw the not valid JSON defect, so the total of open
/// containers the reading accepts is 9997.
pub const MAX_OPEN_CONTAINERS: usize = 9997;

/// The most digits an integer literal may carry, probed against
/// the Python reading, whose integer string conversion limit
/// stands at 4300 digits. The same limit binds the rendering of
/// a computed integer, a sum whose text would pass it, as
/// has_int_past_digit_limit checks.
pub const MAX_INTEGER_DIGITS: usize = 4300;

/// A parsed JSON value.
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(Num),
    Str(PText),
    Arr(Vec<JsonValue>),
    Obj(Vec<(PText, JsonValue)>),
}

/// Dropping a deeply nested value must not recurse, because the
/// reading accepts nesting thousands of containers deep and a
/// recursive drop would overflow the stack on a value the
/// reading accepted.
impl Drop for JsonValue {
    fn drop(&mut self) {
        let mut stack: Vec<JsonValue> = Vec::new();
        self.drain_children(&mut stack);
        while let Some(mut value) = stack.pop() {
            value.drain_children(&mut stack);
        }
    }
}

impl JsonValue {
    fn drain_children(&mut self, stack: &mut Vec<JsonValue>) {
        match self {
            JsonValue::Arr(items) => stack.append(items),
            JsonValue::Obj(pairs) => {
                for (_, value) in pairs.drain(..) {
                    stack.push(value);
                }
            }
            _ => {}
        }
    }

    /// Build an object from key and value pairs, holding the
    /// pairs the way a Python dict holds them, a repeated key
    /// keeping its first position and its last value.
    pub fn object(pairs: Vec<(PText, JsonValue)>) -> JsonValue {
        let mut held: Vec<(PText, JsonValue)> = Vec::new();
        for (key, value) in pairs {
            object_insert(&mut held, key, value);
        }
        JsonValue::Obj(held)
    }

    /// Read one field of an object by its key text, or None when
    /// the value is not an object or the field is absent.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        if let JsonValue::Obj(pairs) = self {
            let wanted = PText::from_text(key);
            for (held, value) in pairs {
                if *held == wanted {
                    return Some(value);
                }
            }
        }
        None
    }

    /// Return whether this value is an object holding the field.
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn as_object(&self) -> Option<&Vec<(PText, JsonValue)>> {
        match self {
            JsonValue::Obj(pairs) => Some(pairs),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Arr(items) => Some(items),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&PText> {
        match self {
            JsonValue::Str(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_num(&self) -> Option<&Num> {
        match self {
            JsonValue::Num(num) => Some(num),
            _ => None,
        }
    }
}

fn object_insert(pairs: &mut Vec<(PText, JsonValue)>, key: PText, value: JsonValue) {
    for (held, slot) in pairs.iter_mut() {
        if *held == key {
            *slot = value;
            return;
        }
    }
    pairs.push((key, value));
}

/// A parse failure. The loaders name every failure of the
/// reading with the one defect phrase, input is not valid JSON,
/// so the failure carries no detail of its own that any surface
/// prints. The makers read a failure of either reading as a
/// refusal, under the rule PORT-6 states.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    Malformed,
    TooDeep,
}

type ParseResult<T> = Result<T, ParseError>;

/// Parse JSON text the way the Python reading parses it. The
/// text arrives as a Rust string and is read as the code unit
/// sequence the Python string holds.
pub fn parse(text: &str) -> ParseResult<JsonValue> {
    let units: Vec<u16> = text.encode_utf16().collect();
    parse_units_mode(&units, false)
}

/// Parse JSON text the way the Python reading parses it when
/// the decimal type stands for every float, the exact reading
/// the makers sum an envelope cost through. A float literal
/// becomes the exact decimal its text names, under the
/// construction bounds PORT-4 states, and a literal past those
/// bounds fails the reading, as the Python reading raises out
/// of its parse on the same literal.
pub fn parse_exact(text: &str) -> ParseResult<JsonValue> {
    let units: Vec<u16> = text.encode_utf16().collect();
    parse_units_mode(&units, true)
}

/// Parse a JSON document held as code units. The chat envelope
/// reader reads the message content through here, because that
/// content reaches the reader as a parsed string whose units
/// are exactly what the Python reading would hand its own
/// reading of the content.
pub fn parse_units(units: &[u16]) -> ParseResult<JsonValue> {
    parse_units_mode(units, false)
}

fn parse_units_mode(units: &[u16], exact: bool) -> ParseResult<JsonValue> {
    let mut parser = Parser {
        units,
        pos: 0,
        exact,
    };
    parser.skip_whitespace();
    let value = parser.parse_document()?;
    parser.skip_whitespace();
    if parser.pos != units.len() {
        return Err(ParseError::Malformed);
    }
    Ok(value)
}

struct Parser<'a> {
    units: &'a [u16],
    pos: usize,
    exact: bool,
}

enum Frame {
    Arr(Vec<JsonValue>),
    Obj(Vec<(PText, JsonValue)>, Option<PText>),
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u16> {
        self.units.get(self.pos).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(unit) = self.peek() {
            if unit == 0x20 || unit == 0x09 || unit == 0x0A || unit == 0x0D {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Parse one document value with an explicit container
    /// stack, so a deeply nested input meets the container
    /// limit rather than the call stack.
    fn parse_document(&mut self) -> ParseResult<JsonValue> {
        let mut stack: Vec<Frame> = Vec::new();
        'values: loop {
            // Read one value. A container opening either closes
            // at once, holding nothing, or pushes a frame and
            // reads on into its first element or its first key.
            self.skip_whitespace();
            let mut completed = match self.peek() {
                Some(0x5B) => {
                    // [
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return Err(ParseError::TooDeep);
                    }
                    self.pos += 1;
                    self.skip_whitespace();
                    if self.peek() == Some(0x5D) {
                        self.pos += 1;
                        JsonValue::Arr(Vec::new())
                    } else {
                        stack.push(Frame::Arr(Vec::new()));
                        continue 'values;
                    }
                }
                Some(0x7B) => {
                    // {
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return Err(ParseError::TooDeep);
                    }
                    self.pos += 1;
                    self.skip_whitespace();
                    if self.peek() == Some(0x7D) {
                        self.pos += 1;
                        JsonValue::Obj(Vec::new())
                    } else {
                        stack.push(Frame::Obj(Vec::new(), None));
                        self.read_key(&mut stack)?;
                        continue 'values;
                    }
                }
                _ => self.parse_atom()?,
            };
            // Attach the completed value to its container, and
            // keep attaching as containers close behind it.
            'attach: loop {
                match stack.last_mut() {
                    None => return Ok(completed),
                    Some(Frame::Arr(items)) => {
                        items.push(completed);
                        self.skip_whitespace();
                        match self.peek() {
                            Some(0x2C) => {
                                self.pos += 1;
                                continue 'values;
                            }
                            Some(0x5D) => {
                                self.pos += 1;
                                if let Some(Frame::Arr(done)) = stack.pop() {
                                    completed = JsonValue::Arr(done);
                                    continue 'attach;
                                }
                                return Err(ParseError::Malformed);
                            }
                            _ => return Err(ParseError::Malformed),
                        }
                    }
                    Some(Frame::Obj(pairs, pending)) => {
                        let key = pending.take().ok_or(ParseError::Malformed)?;
                        object_insert(pairs, key, completed);
                        self.skip_whitespace();
                        match self.peek() {
                            Some(0x2C) => {
                                self.pos += 1;
                                self.read_key(&mut stack)?;
                                continue 'values;
                            }
                            Some(0x7D) => {
                                self.pos += 1;
                                if let Some(Frame::Obj(done, _)) = stack.pop() {
                                    completed = JsonValue::Obj(done);
                                    continue 'attach;
                                }
                                return Err(ParseError::Malformed);
                            }
                            _ => return Err(ParseError::Malformed),
                        }
                    }
                }
            }
        }
    }

    /// Read an object key and its colon into the top frame.
    fn read_key(&mut self, stack: &mut [Frame]) -> ParseResult<()> {
        self.skip_whitespace();
        if self.peek() != Some(0x22) {
            return Err(ParseError::Malformed);
        }
        let key = self.parse_string()?;
        self.skip_whitespace();
        if self.peek() != Some(0x3A) {
            return Err(ParseError::Malformed);
        }
        self.pos += 1;
        match stack.last_mut() {
            Some(Frame::Obj(_, pending)) => {
                *pending = Some(key);
                Ok(())
            }
            _ => Err(ParseError::Malformed),
        }
    }

    /// Parse a value that is not a container opening, a string,
    /// a number, or one of the literal words.
    fn parse_atom(&mut self) -> ParseResult<JsonValue> {
        match self.peek() {
            Some(0x22) => Ok(JsonValue::Str(self.parse_string()?)),
            Some(0x74) => {
                self.take_word("true")?;
                Ok(JsonValue::Bool(true))
            }
            Some(0x66) => {
                self.take_word("false")?;
                Ok(JsonValue::Bool(false))
            }
            Some(0x6E) => {
                self.take_word("null")?;
                Ok(JsonValue::Null)
            }
            Some(0x4E) => {
                self.take_word("NaN")?;
                Ok(JsonValue::Num(Num::Float(f64::NAN)))
            }
            Some(0x49) => {
                self.take_word("Infinity")?;
                Ok(JsonValue::Num(Num::Float(f64::INFINITY)))
            }
            Some(0x2D) | Some(0x30..=0x39) => self.parse_number(),
            _ => Err(ParseError::Malformed),
        }
    }

    fn take_word(&mut self, word: &str) -> ParseResult<()> {
        for expected in word.encode_utf16() {
            if self.peek() != Some(expected) {
                return Err(ParseError::Malformed);
            }
            self.pos += 1;
        }
        Ok(())
    }

    fn parse_string(&mut self) -> ParseResult<PText> {
        // The caller has seen the opening quote.
        self.pos += 1;
        let mut units: Vec<u16> = Vec::new();
        loop {
            let unit = self.peek().ok_or(ParseError::Malformed)?;
            self.pos += 1;
            match unit {
                0x22 => return Ok(PText::from_units(units)),
                0x5C => {
                    let escape = self.peek().ok_or(ParseError::Malformed)?;
                    self.pos += 1;
                    match escape {
                        0x22 => units.push(0x22),
                        0x5C => units.push(0x5C),
                        0x2F => units.push(0x2F),
                        0x62 => units.push(0x08),
                        0x66 => units.push(0x0C),
                        0x6E => units.push(0x0A),
                        0x72 => units.push(0x0D),
                        0x74 => units.push(0x09),
                        0x75 => {
                            let mut value: u16 = 0;
                            for _ in 0..4 {
                                let digit = self.peek().ok_or(ParseError::Malformed)?;
                                self.pos += 1;
                                let nibble = match digit {
                                    0x30..=0x39 => digit - 0x30,
                                    0x61..=0x66 => digit - 0x61 + 10,
                                    0x41..=0x46 => digit - 0x41 + 10,
                                    _ => return Err(ParseError::Malformed),
                                };
                                value = (value << 4) | nibble;
                            }
                            units.push(value);
                        }
                        _ => return Err(ParseError::Malformed),
                    }
                }
                0x00..=0x1F => return Err(ParseError::Malformed),
                _ => units.push(unit),
            }
        }
    }

    fn parse_number(&mut self) -> ParseResult<JsonValue> {
        let start = self.pos;
        if self.peek() == Some(0x2D) {
            self.pos += 1;
            if self.peek() == Some(0x49) {
                self.take_word("Infinity")?;
                return Ok(JsonValue::Num(Num::Float(f64::NEG_INFINITY)));
            }
        }
        // The integer part, a lone zero or a run not starting
        // with zero.
        match self.peek() {
            Some(0x30) => {
                self.pos += 1;
            }
            Some(0x31..=0x39) => {
                while matches!(self.peek(), Some(0x30..=0x39)) {
                    self.pos += 1;
                }
            }
            _ => return Err(ParseError::Malformed),
        }
        let mut is_float = false;
        if self.peek() == Some(0x2E) {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some(0x30..=0x39)) {
                return Err(ParseError::Malformed);
            }
            while matches!(self.peek(), Some(0x30..=0x39)) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(0x65) | Some(0x45)) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(0x2B) | Some(0x2D)) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(0x30..=0x39)) {
                return Err(ParseError::Malformed);
            }
            while matches!(self.peek(), Some(0x30..=0x39)) {
                self.pos += 1;
            }
        }
        let token: String = self.units[start..self.pos]
            .iter()
            .map(|unit| (*unit as u8) as char)
            .collect();
        if is_float {
            if self.exact {
                let value = PyDecimal::parse_literal(&token).ok_or(ParseError::Malformed)?;
                return Ok(JsonValue::Num(Num::Decimal(value)));
            }
            let value: f64 = token.parse().map_err(|_| ParseError::Malformed)?;
            return Ok(JsonValue::Num(Num::Float(value)));
        }
        let digits = self.pos - start - usize::from(self.units[start] == 0x2D);
        if digits > MAX_INTEGER_DIGITS {
            return Err(ParseError::Malformed);
        }
        let value = BigInt::parse_bytes(token.as_bytes(), 10).ok_or(ParseError::Malformed)?;
        Ok(JsonValue::Num(Num::Int(value)))
    }
}

/// Return whether rendering this value as JSON text would meet
/// an integer past the digit limit. The Python reading refuses
/// to convert an integer of more than MAX_INTEGER_DIGITS digits
/// to text, and a computed sum reaches that conversion when the
/// run record text is built, so the writers of that text check
/// the value here first, as PORT-6 states. The walk is
/// iterative, as the reading accepts values nested thousands of
/// containers deep.
pub fn has_int_past_digit_limit(value: &JsonValue) -> bool {
    use num_traits::Signed;
    let mut stack: Vec<&JsonValue> = vec![value];
    while let Some(item) = stack.pop() {
        match item {
            JsonValue::Num(Num::Int(digits)) => {
                if digits.abs().to_string().len() > MAX_INTEGER_DIGITS {
                    return true;
                }
            }
            JsonValue::Arr(items) => stack.extend(items.iter()),
            JsonValue::Obj(pairs) => stack.extend(pairs.iter().map(|(_, held)| held)),
            _ => {}
        }
    }
    false
}

/// Return whether any string in this value holds an unpaired
/// surrogate. A maker writing such a value as UTF-8 text fails
/// in the Python surface, so the makers refuse the value here
/// before any file is written, as PORT-6 states. The walk is
/// iterative, as the reading accepts values nested thousands of
/// containers deep.
pub fn has_unpaired_surrogate(value: &JsonValue) -> bool {
    let mut stack: Vec<&JsonValue> = vec![value];
    while let Some(item) = stack.pop() {
        match item {
            JsonValue::Str(text) => {
                if text.holds_unpaired_surrogate() {
                    return true;
                }
            }
            JsonValue::Arr(items) => stack.extend(items.iter()),
            JsonValue::Obj(pairs) => {
                for (key, held) in pairs {
                    if key.holds_unpaired_surrogate() {
                        return true;
                    }
                    stack.push(held);
                }
            }
            _ => {}
        }
    }
    false
}

/// Write a value as JSON text the way the Python dumps writes
/// it, with the given indent in spaces, or the plain separators
/// when no indent is given, and with keys sorted when asked.
/// Under the ascii reading every character outside the
/// printable ASCII range is escaped, as the Python dumps
/// escapes it. Under the raw reading only the quote, the
/// backslash, and the control characters are escaped, and every
/// other character stands as itself, as the Python dumps with
/// its ascii reading off writes it.
pub fn dumps(
    value: &JsonValue,
    indent: Option<usize>,
    sort_keys: bool,
    ensure_ascii: bool,
) -> String {
    let mut out = String::new();
    write_value(&mut out, value, indent, sort_keys, ensure_ascii, 0);
    out
}

fn write_value(
    out: &mut String,
    value: &JsonValue,
    indent: Option<usize>,
    sort_keys: bool,
    ensure_ascii: bool,
    level: usize,
) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(true) => out.push_str("true"),
        JsonValue::Bool(false) => out.push_str("false"),
        JsonValue::Num(Num::Int(digits)) => out.push_str(&digits.to_string()),
        JsonValue::Num(Num::Float(figure)) => out.push_str(&dumps_float(*figure)),
        JsonValue::Num(Num::Decimal(figure)) => out.push_str(&figure.render()),
        JsonValue::Str(text) => write_string(out, text, ensure_ascii),
        JsonValue::Arr(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            match indent {
                None => {
                    out.push('[');
                    for (index, item) in items.iter().enumerate() {
                        if index > 0 {
                            out.push_str(", ");
                        }
                        write_value(out, item, indent, sort_keys, ensure_ascii, level);
                    }
                    out.push(']');
                }
                Some(width) => {
                    out.push_str("[\n");
                    for (index, item) in items.iter().enumerate() {
                        if index > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&" ".repeat(width * (level + 1)));
                        write_value(out, item, indent, sort_keys, ensure_ascii, level + 1);
                    }
                    out.push('\n');
                    out.push_str(&" ".repeat(width * level));
                    out.push(']');
                }
            }
        }
        JsonValue::Obj(pairs) => {
            if pairs.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut ordered: Vec<&(PText, JsonValue)> = pairs.iter().collect();
            if sort_keys {
                ordered.sort_by(|a, b| a.0.cmp_code_points(&b.0));
            }
            match indent {
                None => {
                    out.push('{');
                    for (index, (key, item)) in ordered.iter().enumerate() {
                        if index > 0 {
                            out.push_str(", ");
                        }
                        write_string(out, key, ensure_ascii);
                        out.push_str(": ");
                        write_value(out, item, indent, sort_keys, ensure_ascii, level);
                    }
                    out.push('}');
                }
                Some(width) => {
                    out.push_str("{\n");
                    for (index, (key, item)) in ordered.iter().enumerate() {
                        if index > 0 {
                            out.push_str(",\n");
                        }
                        out.push_str(&" ".repeat(width * (level + 1)));
                        write_string(out, key, ensure_ascii);
                        out.push_str(": ");
                        write_value(out, item, indent, sort_keys, ensure_ascii, level + 1);
                    }
                    out.push('\n');
                    out.push_str(&" ".repeat(width * level));
                    out.push('}');
                }
            }
        }
    }
}

fn dumps_float(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        };
    }
    py_float_repr(value)
}

/// Write one string with the escapes of the stated reading.
/// The quote and the backslash are escaped, the short escapes
/// stand for the named control characters, and every other
/// character the reading refuses stands as a unicode escape
/// under the ascii reading, or as itself under the raw reading.
fn write_string(out: &mut String, text: &PText, ensure_ascii: bool) {
    out.push('"');
    let units = text.units();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        match unit {
            0x22 => out.push_str("\\\""),
            0x5C => out.push_str("\\\\"),
            0x08 => out.push_str("\\b"),
            0x0C => out.push_str("\\f"),
            0x0A => out.push_str("\\n"),
            0x0D => out.push_str("\\r"),
            0x09 => out.push_str("\\t"),
            0x20..=0x7E => out.push(unit as u8 as char),
            _ => {
                if ensure_ascii {
                    out.push_str(&format!("\\u{:04x}", unit));
                } else if (0xD800..0xDC00).contains(&unit)
                    && index + 1 < units.len()
                    && (0xDC00..0xE000).contains(&units[index + 1])
                {
                    let point = 0x10000
                        + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
                    out.push(char::from_u32(point).unwrap_or('\u{FFFD}'));
                    index += 1;
                } else if (0xD800..0xE000).contains(&unit) {
                    // An unpaired surrogate has no raw form. The
                    // makers refuse such a value before writing,
                    // so this escape stands only where a library
                    // reading renders a value no file receives.
                    out.push_str(&format!("\\u{:04x}", unit));
                } else if unit < 0x20 {
                    out.push_str(&format!("\\u{:04x}", unit));
                } else {
                    out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                }
            }
        }
        index += 1;
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(text: &str) -> JsonValue {
        parse(text).expect("the probe input parses")
    }

    #[test]
    fn a_repeated_key_keeps_its_last_value_at_its_first_position() {
        let value = parsed("{\"a\": 1, \"b\": 2, \"a\": 3}");
        let pairs = value.as_object().expect("an object");
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].1, JsonValue::Num(Num::from_i64(3)));
        assert_eq!(pairs[1].1, JsonValue::Num(Num::from_i64(2)));
    }

    #[test]
    fn an_integer_literal_stays_an_integer() {
        let value = parsed("{\"n\": 5}");
        match value.get("n") {
            Some(JsonValue::Num(num)) => assert!(num.is_int()),
            other => panic!("an integer, found {:?}", other),
        }
    }

    #[test]
    fn a_fraction_literal_is_a_double_even_when_integral() {
        let value = parsed("{\"n\": 5.0}");
        match value.get("n") {
            Some(JsonValue::Num(Num::Float(figure))) => assert_eq!(*figure, 5.0),
            other => panic!("a double, found {:?}", other),
        }
    }

    #[test]
    fn a_huge_integer_is_an_integer_and_not_finite() {
        let text = format!("{{\"n\": 1{}}}", "0".repeat(400));
        let value = parsed(&text);
        match value.get("n") {
            Some(JsonValue::Num(num)) => {
                assert!(num.is_int());
                assert!(!num.is_finite_number());
            }
            other => panic!("an integer, found {:?}", other),
        }
    }

    #[test]
    fn an_integer_past_the_digit_limit_is_refused() {
        let text = format!("1{}", "0".repeat(4300));
        assert!(parse(&text).is_err());
        let text = format!("1{}", "0".repeat(4299));
        assert!(parse(&text).is_ok());
    }

    #[test]
    fn an_overflowing_float_literal_parses_to_infinity() {
        let value = parsed("{\"n\": 1e400}");
        match value.get("n") {
            Some(JsonValue::Num(num)) => assert!(!num.is_finite_number()),
            other => panic!("a double, found {:?}", other),
        }
    }

    #[test]
    fn the_nan_and_infinity_literals_parse() {
        let value = parsed("{\"a\": NaN, \"b\": Infinity, \"c\": -Infinity}");
        match value.get("a") {
            Some(JsonValue::Num(Num::Float(figure))) => assert!(figure.is_nan()),
            other => panic!("a double, found {:?}", other),
        }
        match value.get("b") {
            Some(JsonValue::Num(Num::Float(figure))) => assert!(figure.is_infinite()),
            other => panic!("a double, found {:?}", other),
        }
    }

    #[test]
    fn an_unpaired_surrogate_is_representable() {
        let value = parsed("{\"s\": \"bad\\ud800x\"}");
        match value.get("s") {
            Some(JsonValue::Str(text)) => assert!(text.holds_unpaired_surrogate()),
            other => panic!("a string, found {:?}", other),
        }
        let value = parsed("{\"s\": \"bad\\ud83d\\ude00x\"}");
        match value.get("s") {
            Some(JsonValue::Str(text)) => assert!(!text.holds_unpaired_surrogate()),
            other => panic!("a string, found {:?}", other),
        }
    }

    #[test]
    fn nesting_at_the_command_line_boundary_loads_and_past_it_fails() {
        // Both sides of the boundary PORT-3 states, a document
        // of nothing but arrays, where the open containers are
        // the arrays themselves.
        let loads = "[".repeat(MAX_OPEN_CONTAINERS) + "1" + &"]".repeat(MAX_OPEN_CONTAINERS);
        assert!(parse(&loads).is_ok());
        let fails =
            "[".repeat(MAX_OPEN_CONTAINERS + 1) + "1" + &"]".repeat(MAX_OPEN_CONTAINERS + 1);
        assert!(parse(&fails).is_err());
        // The shape the boundary was measured in, arrays nested
        // in an ignored field inside a root object.
        let in_object = format!(
            "{{\"extra\": {}1{}}}",
            "[".repeat(MAX_OPEN_CONTAINERS - 1),
            "]".repeat(MAX_OPEN_CONTAINERS - 1)
        );
        assert!(parse(&in_object).is_ok());
        let in_object = format!(
            "{{\"extra\": {}1{}}}",
            "[".repeat(MAX_OPEN_CONTAINERS),
            "]".repeat(MAX_OPEN_CONTAINERS)
        );
        assert!(parse(&in_object).is_err());
    }

    #[test]
    fn a_depth_refusal_names_its_kind_and_a_syntax_refusal_names_its_own() {
        let deep = "[".repeat(10000) + &"]".repeat(10000);
        assert_eq!(parse(&deep), Err(ParseError::TooDeep));
        assert_eq!(parse("{not json"), Err(ParseError::Malformed));
        // Text that is both deep and malformed past the depth
        // point still names the depth refusal, because the
        // reading meets the depth first, as the Python reading
        // raises its recursion failure before any later syntax.
        let deep_then_bad = "[".repeat(10000) + "x";
        assert_eq!(parse(&deep_then_bad), Err(ParseError::TooDeep));
    }

    #[test]
    fn malformed_inputs_are_refused() {
        for text in [
            "{not json",
            "",
            "   ",
            "{\"a\": 01}",
            "{\"a\": 1_000}",
            "{\"a\": +1}",
            "{\"a\": .5}",
            "{\"a\": 1.}",
            "{\"a\": tru}",
            "{\"a\": 1} {\"b\": 2}",
            "[1, 2",
            "{\"a\": \"x\\x\"}",
            "[1,]",
            "{\"a\": 1,}",
        ] {
            assert!(parse(text).is_err(), "refused: {}", text);
        }
    }

    #[test]
    fn the_exact_reading_holds_a_float_literal_as_its_decimal() {
        let value = parse_exact("{\"cost\": 0.10000000000000000000000001, \"n\": 5}")
            .expect("the probe input parses");
        match value.get("cost") {
            Some(JsonValue::Num(Num::Decimal(figure))) => {
                assert_eq!(figure.render(), "0.10000000000000000000000001");
            }
            other => panic!("a decimal, found {:?}", other),
        }
        match value.get("n") {
            Some(JsonValue::Num(num)) => assert!(num.is_int()),
            other => panic!("an integer, found {:?}", other),
        }
        // The plain reading of the same literal is the double,
        // whose shortest text is not the literal.
        let plain = parsed("{\"cost\": 0.10000000000000000000000001}");
        match plain.get("cost") {
            Some(JsonValue::Num(Num::Float(figure))) => assert_eq!(*figure, 0.1),
            other => panic!("a double, found {:?}", other),
        }
        // A float literal past the construction bounds fails
        // the exact reading, as the Python reading raises.
        assert!(parse_exact("{\"cost\": 1e1000000000000000000}").is_err());
    }

    #[test]
    fn dumps_writes_the_python_forms() {
        let value = parsed("{\"b\": 1, \"a\": {\"d\": [1, 2], \"c\": {}}}");
        assert_eq!(
            dumps(&value, Some(1), true, true),
            "{\n \"a\": {\n  \"c\": {},\n  \"d\": [\n   1,\n   2\n  ]\n },\n \"b\": 1\n}"
        );
        let value = parsed("{\"a\": 1.0, \"c\": 1e20, \"e\": 0.00001}");
        assert_eq!(
            dumps(&value, None, true, true),
            "{\"a\": 1.0, \"c\": 1e+20, \"e\": 1e-05}"
        );
        let value = JsonValue::object(vec![(
            PText::from_text("s"),
            JsonValue::Str(PText::from_text("caf\u{e9} \u{2014} \u{7f} \u{1}")),
        )]);
        assert_eq!(
            dumps(&value, None, false, true),
            "{\"s\": \"caf\\u00e9 \\u2014 \\u007f \\u0001\"}"
        );
        assert_eq!(
            dumps(&value, None, false, false),
            "{\"s\": \"caf\u{e9} \u{2014} \u{7f} \\u0001\"}"
        );
    }
}
