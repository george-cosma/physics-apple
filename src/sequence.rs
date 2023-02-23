/// Represents a sequence of numbers that also holds additional information, to generate a sequence of strings.
/// TODO: update examples. They are now wrong.
pub struct Sequence {
    /// String prefix
    prefix: String,
    /// String suffix
    suffix: String,
    /// The start of the sequence
    start: usize,
    /// The (inclusive) end of the sequence.
    end: usize,
    /// Where currently in the sequence we are.
    current: usize,
    /// If set to true, values from Sequence::next() will be padded with 0's so that all elements will be the same size as the biggest value in the sequence.
    pad: bool,
    pad_size: usize,
}

impl Sequence {
    /// Create a new Sequence.
    /// prefix: Optional, currently unused, but can be used manually
    /// suffix: Optional, currently unused, but can be used manually
    /// start: The first value at which the sequence will start counting to. Must be less than "end"
    /// end: The last value (inclusive) at which the sequence will stop. Must be greater than "start"
    /// # Example
    /// ```
    /// let seq = Sequence::new("frame_", ".png", 1, 99);
    /// let format = format!("{}{:0>2}{}", seq.prefix, seq.current, seq.suffix);
    /// assert_eq!(format, "frame_01.png");
    /// ```
    pub fn new(prefix: &String, suffix: &String, start: usize, end: usize, pad: bool) -> Sequence {
        Sequence {
            prefix: prefix.clone(),
            suffix: suffix.clone(),
            start: start,
            end: end,
            current: start,
            pad: pad,
            pad_size: if pad { format!("{}", end).len() } else { 0 },
        }
    }
    /// Returns the current value of the sequence, and advances it.
    ///
    /// # Return values
    /// Returns `Some(usize)` if the sequence hasn't ended (self.current <= self.end).
    /// Otherwise, it returns `None`
    ///
    /// # Example
    /// ```
    /// let seq = Sequence::new("frame_", ".png", 1, 99);
    /// asser_eq!(seq.next(), Some(1));
    /// if let Some(i) = seq.next() {
    ///     let format = format!("{}{:0>2}{}", seq.prefix, i, seq.suffix);
    ///     assert_eq!(format, "frame_02.png");
    ///     assert_eq!(seq.current, 3);
    /// }
    /// ```
    pub fn next(&mut self) -> Option<String> {
        if self.current == self.end + 1 {
            return None;
        }

        let output = format!(
            "{}{:0>width$}{}",
            self.prefix,
            self.current,
            self.suffix,
            width = if self.pad { self.pad_size } else { 0 }
        );
        self.current += 1;
        return Some(output);
    }

    /// Restart the sequence so that the current value is the "start" value.
    pub fn restart(&mut self) {
        self.current = self.start;
    }
}
