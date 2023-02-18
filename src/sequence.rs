/// Represents a sequence of numbers that also holds additional information, to generate a sequence of strings.
pub struct Sequence<'a> {
    /// String prefix
    pub prefix: &'a str,
    /// String suffix
    pub suffix: &'a str,
    /// The start of the sequence
    start: usize,
    /// The (inclusive) end of the sequence.
    end: usize,
    /// Where currently in the sequence we are.
    current: usize,
}

impl Sequence<'_> {
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
    pub fn new<'a>(prefix: &'a str, suffix: &'a str, start: usize, end: usize) -> Sequence<'a> {
        Sequence {
            prefix: prefix,
            suffix: suffix,
            start: start,
            end: end,
            current: start,
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
    pub fn next(&mut self) -> Option<usize> {
        if self.current == self.end + 1 {
            return None;
        }

        let output = self.current;
        self.current += 1;
        return Some(output);
    }

    /// Restart the sequence so that the current value is the "start" value.
    pub fn restart(&mut self) {
        self.current = self.start;
    }
}
