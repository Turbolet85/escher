//! The value of an `input` in the range state, and the rules that keep it a number inside its
//! bounds and on its step grid.
//!
//! <https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)>

/// A range `input`'s current value: the number its keys and clicks move, held apart from its
/// `value` attribute, which is its default value.
///
/// The value is always finite, inside the input's `[min, max]` and, where the input has an
/// allowed step, on its step grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeInputData {
    value: f64,
}

impl RangeInputData {
    /// The data of a range input whose attributes read `rules` and whose `value` attribute reads
    /// `value_attribute`, its current value that attribute sanitized.
    pub(crate) fn new(rules: &RangeRules, value_attribute: Option<&str>) -> Self {
        Self {
            value: rules.sanitize_attribute(value_attribute),
        }
    }

    /// The current value as a number.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the current value to `value_attribute` sanitized, as a write of the `value`
    /// attribute does.
    pub(crate) fn reseed(&mut self, rules: &RangeRules, value_attribute: Option<&str>) {
        self.value = rules.sanitize_attribute(value_attribute);
    }

    /// Sanitizes the current value against `rules`, as a write of `min`, `max` or `step` does.
    pub(crate) fn resanitize(&mut self, rules: &RangeRules) {
        self.value = rules.sanitize(self.value);
    }

    /// Applies `to` under `rules` and reads whether the current value changed.
    pub(crate) fn apply(&mut self, rules: &RangeRules, to: RangeMove) -> bool {
        let moved = rules.moved(self.value, to);
        let changed = moved != self.value;
        self.value = moved;
        changed
    }
}

/// A move of a range input's value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RangeMove {
    /// One step up from the current value.
    StepUp,
    /// One step down from the current value.
    StepDown,
    /// To the minimum.
    Minimum,
    /// To the maximum.
    Maximum,
    /// To a fraction in `[0, 1]` of the way from the minimum to the maximum.
    Fraction(f64),
}

/// The range state's rules as one input's `min`, `max`, `step` and `value` attributes read them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RangeRules {
    min: f64,
    max: f64,
    /// The allowed step; `None` for `step="any"`.
    step: Option<f64>,
    base: f64,
}

const DEFAULT_MIN: f64 = 0.0;
const DEFAULT_MAX: f64 = 100.0;
const DEFAULT_STEP: f64 = 1.0;

/// How far a quotient may sit from a whole number and still count as one, so that a grid value
/// computed in binary floating point (`0.3 / 0.1`) lands on its step.
const GRID_TOLERANCE: f64 = 1e-9;

impl RangeRules {
    /// The rules of a range input whose four attributes read these texts (`None` where absent).
    pub(crate) fn new(
        min: Option<&str>,
        max: Option<&str>,
        step: Option<&str>,
        value: Option<&str>,
    ) -> Self {
        let parsed_min = min.and_then(parse_float);
        let min = parsed_min.unwrap_or(DEFAULT_MIN);
        // https://html.spec.whatwg.org/multipage/input.html#concept-input-max-default: a
        // maximum below the minimum reads as the minimum.
        let max = max.and_then(parse_float).unwrap_or(DEFAULT_MAX).max(min);
        // https://html.spec.whatwg.org/multipage/input.html#concept-input-step
        let step = match step {
            Some(step)
                if step
                    .trim_matches(is_ascii_whitespace)
                    .eq_ignore_ascii_case("any") =>
            {
                None
            }
            Some(step) => Some(
                parse_float(step)
                    .filter(|step| *step > 0.0)
                    .unwrap_or(DEFAULT_STEP),
            ),
            None => Some(DEFAULT_STEP),
        };
        // https://html.spec.whatwg.org/multipage/input.html#concept-input-min-zero
        let base = parsed_min
            .or_else(|| value.and_then(parse_float))
            .unwrap_or(0.0);
        Self {
            min,
            max,
            step,
            base,
        }
    }

    /// The default value: the midpoint of `[min, max]`, written so that no width overflows.
    fn default_value(&self) -> f64 {
        self.min / 2.0 + self.max / 2.0
    }

    /// The `value` attribute's text sanitized: the default value where it does not parse.
    fn sanitize_attribute(&self, value: Option<&str>) -> f64 {
        let value = value
            .and_then(parse_float)
            .unwrap_or_else(|| self.default_value());
        self.sanitize(value)
    }

    /// `value` clamped to `[min, max]` and moved to the nearest step-grid value inside them, the
    /// greater on a tie, then rounded to the grid's decimals.
    ///
    /// <https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)>, value
    /// sanitization and step mismatch.
    fn sanitize(&self, value: f64) -> f64 {
        let value = self.clamp(if value.is_finite() {
            value
        } else {
            self.default_value()
        });
        let Some(step) = self.step else {
            return value;
        };
        let lowest = (self.grid_index(self.min, step) - GRID_TOLERANCE).ceil();
        let highest = (self.grid_index(self.max, step) + GRID_TOLERANCE).floor();
        if lowest > highest || !lowest.is_finite() || !highest.is_finite() {
            return value;
        }
        let nearest = (self.grid_index(value, step) + 0.5 + GRID_TOLERANCE).floor();
        let index = nearest.max(lowest).min(highest);
        self.clamp(round_to(self.base + index * step, self.decimals()))
    }

    /// How many steps `value` lies from the step base, each side divided first so that no
    /// difference of two finite bounds overflows.
    fn grid_index(&self, value: f64, step: f64) -> f64 {
        value / step - self.base / step
    }

    fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }

    /// Where `to` takes `value`, sanitized.
    fn moved(&self, value: f64, to: RangeMove) -> f64 {
        // A key steps by 1 where the step is `any`.
        let step = self.step.unwrap_or(DEFAULT_STEP);
        let target = match to {
            RangeMove::StepUp => value + step,
            RangeMove::StepDown => value - step,
            RangeMove::Minimum => self.min,
            RangeMove::Maximum => self.max,
            RangeMove::Fraction(fraction) => {
                let fraction = if fraction.is_finite() {
                    fraction.clamp(0.0, 1.0)
                } else {
                    0.0
                };
                self.min * (1.0 - fraction) + self.max * fraction
            }
        };
        self.sanitize(target)
    }

    /// The decimals a value on the grid is written with: those of the step and the step base,
    /// so that `0.1` stepped three times from `0` writes `0.3`. `None` where there is no grid.
    fn decimals(&self) -> Option<usize> {
        let step = self.step?;
        Some(decimals_of(step).max(decimals_of(self.base)))
    }

    /// `value` as text: the shortest form that reads back as it, `-0` written `0`.
    pub(crate) fn format(&self, value: f64) -> String {
        let value = round_to(value, self.decimals());
        format!("{}", if value == 0.0 { 0.0 } else { value })
    }
}

/// The number of digits after the point in `value`'s shortest decimal form.
fn decimals_of(value: f64) -> usize {
    let text = format!("{value}");
    text.split_once('.')
        .map_or(0, |(_, fraction)| fraction.len())
}

/// `value` rounded to `decimals` places, read back from its decimal text so that no scaling
/// overflows; `value` itself where `decimals` is `None`.
fn round_to(value: f64, decimals: Option<usize>) -> f64 {
    let Some(decimals) = decimals else {
        return value;
    };
    format!("{value:.decimals$}")
        .parse::<f64>()
        .ok()
        .filter(|rounded| rounded.is_finite())
        .unwrap_or(value)
}

fn is_ascii_whitespace(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\x0C' | '\r' | ' ')
}

/// The rules for parsing floating-point number values, an error, NaN or an infinity read as
/// `None`, `-0` read as `0`.
///
/// <https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#rules-for-parsing-floating-point-number-values>
fn parse_float(input: &str) -> Option<f64> {
    let mut chars = input
        .trim_start_matches(is_ascii_whitespace)
        .chars()
        .peekable();
    let mut number = String::new();

    match chars.peek() {
        Some('-') => {
            number.push('-');
            chars.next();
        }
        Some('+') => {
            chars.next();
        }
        _ => {}
    }

    let mut integer = String::new();
    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
        integer.push(digit);
    }

    let mut fraction = String::new();
    if chars.next_if_eq(&'.').is_some() {
        while let Some(digit) = chars.next_if(char::is_ascii_digit) {
            fraction.push(digit);
        }
    }
    // A number opens with a digit, or with a point a digit follows.
    if integer.is_empty() && fraction.is_empty() {
        return None;
    }
    number.push_str(if integer.is_empty() { "0" } else { &integer });
    if !fraction.is_empty() {
        number.push('.');
        number.push_str(&fraction);
    }

    if chars.next_if(|c| matches!(c, 'e' | 'E')).is_some() {
        let mut exponent = String::new();
        if let Some(sign) = chars.next_if(|c| matches!(c, '-' | '+')) {
            exponent.push(sign);
        }
        let mut digits = String::new();
        while let Some(digit) = chars.next_if(char::is_ascii_digit) {
            digits.push(digit);
        }
        if !digits.is_empty() {
            number.push('e');
            number.push_str(&exponent);
            number.push_str(&digits);
        }
    }

    let value = number
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())?;
    Some(if value == 0.0 { 0.0 } else { value })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A value on its rules' grid, within the tolerance the rules round with.
    fn on_grid(rules: &RangeRules, value: f64) -> bool {
        match rules.step {
            None => true,
            Some(step) => {
                let index = rules.grid_index(value, step);
                (index - index.round()).abs() < 1e-6
            }
        }
    }

    type Attributes = (
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
    );

    fn rules(attributes: Attributes) -> RangeRules {
        let (min, max, step, value) = attributes;
        RangeRules::new(min, max, step, value)
    }

    /// Texts and what the float parser reads of them.
    const PARSED: [(&str, Option<f64>); 20] = [
        ("15", Some(15.0)),
        ("-2.5", Some(-2.5)),
        ("+3", Some(3.0)),
        (".5", Some(0.5)),
        ("-.5", Some(-0.5)),
        ("  7", Some(7.0)),
        ("4abc", Some(4.0)),
        ("1.", Some(1.0)),
        ("1e3", Some(1000.0)),
        ("2E-1", Some(0.2)),
        ("5e", Some(5.0)),
        ("1e308", Some(1e308)),
        ("-0", Some(0.0)),
        ("1e309", None),
        ("abc", None),
        ("", None),
        ("-", None),
        (".", None),
        ("NaN", None),
        ("inf", None),
    ];

    #[test]
    fn the_float_parser_reads_the_html_grammar() {
        assert_eq!(PARSED.len(), 20);
        for (row, (text, read)) in PARSED.into_iter().enumerate() {
            let parsed = parse_float(text);
            assert_eq!(parsed, read, "row {row}");
            if let Some(parsed) = parsed {
                assert!(parsed.is_sign_positive() || parsed < 0.0, "row {row}");
            }
        }
    }

    /// Attributes (`min`, `max`, `step`, `value`) and the rules they read as: `min`, `max`,
    /// `step`, step base.
    #[allow(clippy::type_complexity)]
    const RULES: [(Attributes, (f64, f64, Option<f64>, f64)); 9] = [
        ((None, None, None, None), (0.0, 100.0, Some(1.0), 0.0)),
        (
            (Some("0"), Some("30"), Some("0.5"), Some("15")),
            (0.0, 30.0, Some(0.5), 0.0),
        ),
        ((None, None, Some("0"), None), (0.0, 100.0, Some(1.0), 0.0)),
        ((None, None, Some("-1"), None), (0.0, 100.0, Some(1.0), 0.0)),
        ((None, None, Some("ANY"), None), (0.0, 100.0, None, 0.0)),
        (
            (Some("10"), Some("5"), None, None),
            (10.0, 10.0, Some(1.0), 10.0),
        ),
        (
            (Some("NaN"), None, None, Some("0.5")),
            (0.0, 100.0, Some(1.0), 0.5),
        ),
        (
            (None, Some("abc"), Some("x"), Some("abc")),
            (0.0, 100.0, Some(1.0), 0.0),
        ),
        (
            (Some("-1e308"), Some("1e308"), None, None),
            (-1e308, 1e308, Some(1.0), -1e308),
        ),
    ];

    #[test]
    fn attributes_read_as_bounds_step_and_base() {
        assert_eq!(RULES.len(), 9);
        for (row, (attributes, (min, max, step, base))) in RULES.into_iter().enumerate() {
            let read = rules(attributes);
            assert_eq!(
                read,
                RangeRules {
                    min,
                    max,
                    step,
                    base
                },
                "row {row}"
            );
        }
    }

    /// Attributes and the current value a fresh input reads, as text.
    const SEEDED: [(Attributes, &str); 12] = [
        ((None, None, None, None), "50"),
        ((Some("0"), Some("30"), Some("0.5"), Some("15")), "15"),
        ((Some("0"), Some("10"), Some("2"), None), "6"),
        ((Some("0"), Some("10"), Some("2"), Some("7")), "8"),
        ((Some("0"), Some("10"), Some("2"), Some("abc")), "6"),
        ((None, None, Some("0"), Some("3.4")), "3.4"),
        ((Some("0"), None, Some("-1"), Some("3.6")), "4"),
        ((None, None, Some("any"), Some("3.25")), "3.25"),
        ((Some("10"), Some("5"), None, Some("7")), "10"),
        ((Some("0"), None, None, Some("1e308")), "100"),
        ((Some("NaN"), None, None, Some("2.5")), "2.5"),
        ((Some("0"), Some("9"), Some("2"), Some("9")), "8"),
    ];

    #[test]
    fn a_value_attribute_seeds_finite_bounded_and_on_the_grid() {
        assert_eq!(SEEDED.len(), 12);
        for (row, (attributes, read)) in SEEDED.into_iter().enumerate() {
            let rules = rules(attributes);
            let data = RangeInputData::new(&rules, attributes.3);
            let value = data.value();
            assert!(value.is_finite(), "row {row}");
            assert!(rules.min <= value && value <= rules.max, "row {row}");
            assert!(on_grid(&rules, value), "row {row}");
            assert_eq!(rules.format(value), read, "row {row}");
        }
    }

    const STEP_TWO: Attributes = (Some("0"), Some("10"), Some("2"), Some("4"));

    /// Moves on `min=0 max=10 step=2 value=4`, applied in order, the value each reads and
    /// whether it changed.
    const MOVES: [(RangeMove, &str, bool); 9] = [
        (RangeMove::StepUp, "6", true),
        (RangeMove::StepDown, "4", true),
        (RangeMove::Maximum, "10", true),
        (RangeMove::StepUp, "10", false),
        (RangeMove::Minimum, "0", true),
        (RangeMove::StepDown, "0", false),
        (RangeMove::Fraction(0.5), "6", true),
        (RangeMove::Fraction(0.25), "2", true),
        (RangeMove::Fraction(2.0), "10", true),
    ];

    #[test]
    fn moves_step_jump_and_place_on_the_grid() {
        assert_eq!(MOVES.len(), 9);
        let rules = rules(STEP_TWO);
        let mut data = RangeInputData::new(&rules, STEP_TWO.3);
        for (row, (to, read, changed)) in MOVES.into_iter().enumerate() {
            assert_eq!(data.apply(&rules, to), changed, "row {row}");
            assert_eq!(rules.format(data.value()), read, "row {row}");
        }
    }

    /// Degenerate attributes, each moved by every kind of move: every value reads finite,
    /// inside its bounds and on its grid.
    const DEGENERATE: [Attributes; 8] = [
        (None, None, Some("0"), None),
        (None, None, Some("-1"), None),
        (None, None, Some("any"), None),
        (Some("10"), Some("5"), None, None),
        (None, None, None, Some("abc")),
        (None, None, None, Some("1e308")),
        (Some("NaN"), None, None, None),
        (Some("-1e308"), Some("1e308"), Some("1e307"), None),
    ];

    #[test]
    fn degenerate_attributes_keep_the_value_finite_bounded_and_on_the_grid() {
        assert_eq!(DEGENERATE.len(), 8);
        let moves = [
            RangeMove::StepUp,
            RangeMove::StepDown,
            RangeMove::Minimum,
            RangeMove::Maximum,
            RangeMove::Fraction(0.5),
            RangeMove::Fraction(f64::NAN),
            RangeMove::Fraction(-1.0),
        ];
        for (row, attributes) in DEGENERATE.into_iter().enumerate() {
            let rules = rules(attributes);
            let mut data = RangeInputData::new(&rules, attributes.3);
            for to in moves {
                data.apply(&rules, to);
                let value = data.value();
                assert!(value.is_finite(), "row {row}");
                assert!(rules.min <= value && value <= rules.max, "row {row}");
                assert!(on_grid(&rules, value), "row {row}");
            }
        }
    }

    #[test]
    fn a_decimal_step_writes_its_decimals() {
        let rules = rules((Some("0"), Some("1"), Some("0.1"), Some("0")));
        let mut data = RangeInputData::new(&rules, Some("0"));
        for _ in 0..3 {
            data.apply(&rules, RangeMove::StepUp);
        }
        assert_eq!(rules.format(data.value()), "0.3");
        assert_eq!(data.value(), 0.3);
    }

    #[test]
    fn a_reseed_and_a_resanitize_follow_the_attributes() {
        let mut data = RangeInputData::new(&rules(STEP_TWO), STEP_TWO.3);
        data.reseed(&rules(STEP_TWO), Some("7"));
        assert_eq!(data.value(), 8.0);
        let narrowed = rules((Some("0"), Some("5"), Some("2"), Some("4")));
        data.resanitize(&narrowed);
        assert_eq!(data.value(), 4.0);
    }
}
