//! Text held the way the Python reading holds it.
//!
//! A Python string is a sequence of Unicode code points, and the
//! JSON reading can produce a string holding an unpaired surrogate,
//! which no Rust String can hold. The port therefore carries text
//! as a sequence of UTF-16 code units, the PText type. Equality of
//! two PText values is equality of their code unit sequences, which
//! is exactly Python string equality, and an unpaired surrogate is
//! representable, detected, and refused by name, as PORT-3 in
//! CONTRACT.md states.

use std::cmp::Ordering;

/// A string as a sequence of UTF-16 code units.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PText(Vec<u16>);

impl PText {
    pub fn from_units(units: Vec<u16>) -> PText {
        PText(units)
    }

    pub fn from_text(text: &str) -> PText {
        PText(text.encode_utf16().collect())
    }

    pub fn units(&self) -> &[u16] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return whether the text holds a surrogate code unit that
    /// stands without its pair, the shape the Python reading names
    /// with the unpaired surrogate defect. The Python test encodes
    /// the string as UTF-8 and fails exactly on the unpaired
    /// surrogates, so the two readings agree code unit for code
    /// unit.
    pub fn holds_unpaired_surrogate(&self) -> bool {
        let units = &self.0;
        let mut index = 0;
        while index < units.len() {
            let unit = units[index];
            if (0xD800..0xDC00).contains(&unit) {
                if index + 1 < units.len() && (0xDC00..0xE000).contains(&units[index + 1]) {
                    index += 2;
                    continue;
                }
                return true;
            }
            if (0xDC00..0xE000).contains(&unit) {
                return true;
            }
            index += 1;
        }
        false
    }

    /// Return whether the text holds a newline or a carriage
    /// return, the line break the one line rule refuses in every
    /// printed field.
    pub fn holds_line_break(&self) -> bool {
        self.0.iter().any(|unit| *unit == 0x0A || *unit == 0x0D)
    }

    /// Decode the code points of the text in order. A unit that
    /// cannot stand in a code point yields the replacement
    /// character, which no validated field can hold, because the
    /// loaders refuse unpaired surrogates before any reading.
    pub fn code_points(&self) -> Vec<char> {
        let mut points = Vec::new();
        let units = &self.0;
        let mut index = 0;
        while index < units.len() {
            let unit = units[index];
            if (0xD800..0xDC00).contains(&unit)
                && index + 1 < units.len()
                && (0xDC00..0xE000).contains(&units[index + 1])
            {
                let high = (unit as u32) - 0xD800;
                let low = (units[index + 1] as u32) - 0xDC00;
                let point = 0x10000 + ((high << 10) | low);
                points.push(char::from_u32(point).unwrap_or('\u{FFFD}'));
                index += 2;
                continue;
            }
            if (0xD800..0xE000).contains(&unit) {
                points.push('\u{FFFD}');
                index += 1;
                continue;
            }
            points.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            index += 1;
        }
        points
    }

    /// Render the text as a Rust String. Every field a rendering
    /// prints has passed the loader, so it holds no unpaired
    /// surrogate and the rendering is exact.
    pub fn display(&self) -> String {
        self.code_points().into_iter().collect()
    }

    /// Compare two texts by code point, the order the Python
    /// reading sorts labels in.
    pub fn cmp_code_points(&self, other: &PText) -> Ordering {
        let mine = self.code_points();
        let theirs = other.code_points();
        for (a, b) in mine.iter().zip(theirs.iter()) {
            match (*a as u32).cmp(&(*b as u32)) {
                Ordering::Equal => continue,
                order => return order,
            }
        }
        mine.len().cmp(&theirs.len())
    }
}
