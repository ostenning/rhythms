//! A rhythmic pattern generation library with `no_std` support.
//!
//! This project is under development and the current API is subjective to change.
//! Please use at your own risk.
//!
//! ## Example
//!
//! ```
//! use rhythms::Pattern;
//!
//! let pattern = Pattern::<64>::new(4, 2, 0);
//! assert_eq!([true, false, true, false], pattern.as_slice());
//!
//! // or
//! let mut pattern = Pattern::<64>::with_length(4);
//! pattern.pulses(2);
//! pattern.rotate(-1);
//! assert_eq!([false, true, false, true], pattern.as_slice());
//! ```

#![no_std]

use heapless::Vec;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
}

/// The main pattern building block
#[derive(Debug, Clone)]
pub struct Pattern<const MAX_STEPS: usize = 64> {
    pub steps: Vec<bool, MAX_STEPS>,
    length: usize,
    pulses: usize,
    padding: usize,
    rotation: isize,
    cursor: usize,
}

impl<const MAX_STEPS: usize> Pattern<MAX_STEPS> {
    /// Returns a pattern with given length, number of pulses and rotation
    ///
    /// # Arguments
    ///
    /// * `length` - Total number of steps
    /// * `pulses` - The number of pulses
    /// * `rotation` - Number of rotation steps. Polarity represents direction
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::<64>::new(4, 2, 0);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// ```
    pub fn new(length: usize, pulses: usize, padding: usize, rotation: isize) -> Self {
        let mut pattern = Pattern::<MAX_STEPS>::with_length(length);
        pattern.pulses(pulses);
        pattern.rotate(rotation);
        pattern.set_padding(padding);
        pattern
    }

    /// Returns a pattern with given length
    ///
    /// # Arguments
    ///
    /// * `length` - Total number of steps
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::<64>::with_length(8);
    /// assert_eq!(8, pattern.len());
    /// ```
    pub fn with_length(length: usize) -> Self {
        let mut steps = heapless::Vec::<bool, MAX_STEPS>::new();
        for _ in 0..length {
            steps.push(false).unwrap();
        }
        Self {
            steps,
            length,
            padding: 0,
            pulses: 0,
            rotation: 0,
            cursor: 0,
        }
    }

    /// Returns a pattern based on a boolean slice
    ///
    /// # Arguments
    ///
    /// * `slice` - A boolean slice holding the initial pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::from_slice(&[false, false, false, true]);
    /// assert_eq!([false, false, false, true], pattern.as_slice());
    /// ```
    pub fn from_slice(slice: &[bool; MAX_STEPS]) -> Self {
        Self {
            steps: heapless::Vec::from_slice(slice).unwrap(),
            length: slice.len(),
            cursor: 0,
            pulses: 0,
            padding: 0,
            rotation: 0,
        }
    }

    /// Updates the current pattern with a evenly distributed number of pulses, using an
    /// abstraction based on Bjorklund's Euclidean algorithm.
    ///
    /// # Arguments
    ///
    /// * `pulses` - Total number of pulses, from `0` to the pattern length. If `pulses` exceeds
    /// the pattern length, the max value will be used
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::with_length(4);
    /// pattern.pulses(2);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// // or
    /// let mut pattern = Pattern::<64>::new(4, 4, 0);
    /// assert_eq!([true, true, true, true], pattern.as_slice());
    /// pattern.pulses(2);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// ```
    pub fn pulses(&mut self, pulses: usize) {
        self.pulses = if pulses > self.length {
            self.length
        } else {
            pulses
        };

        self.steps.clear();
        let mut bucket: usize = 0;
        for _ in 0..self.length {
            bucket += self.pulses;
            if bucket >= self.length {
                bucket -= self.length;
                self.steps.push(true).unwrap();
            } else {
                self.steps.push(false).unwrap();
            }
        }
        for _ in 0..self.padding {
            if self.steps.push(false).is_err() {
                break;
            }
        }

        // if self.length > 0 && self.pulses > 0 {
        //     let offset = self.length / self.pulses - 1;
        //     self.steps.rotate_right(offset);
        // }
    }

    pub fn num_pulses(&self) -> usize {
        self.pulses
    }

    /// Rotates the current pattern
    ///
    /// # Arguments
    ///
    /// * `rotation` - Number of rotation steps. Polarity represents direction
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::with_length(3);
    /// pattern.pulses(1);
    /// assert_eq!([false, true, false], pattern.as_slice());
    /// pattern.rotate(-1);
    /// assert_eq!([true, false, false], pattern.as_slice());
    /// // or
    /// let pattern = Pattern::<64>::new(3, 1, -1);
    /// assert_eq!([true, false, false], pattern.as_slice());
    /// ```
    pub fn rotate(&mut self, rotation: isize) {
        if rotation.unsigned_abs() >= self.steps.len() {
            return;
        }

        self.rotation = rotation;
        if rotation.is_positive() {
            self.steps.rotate_right(rotation as usize);
        } else if rotation.is_negative() {
            self.steps.rotate_left(rotation.unsigned_abs());
        }
    }

    pub fn rotation(&self) -> isize {
        self.rotation
    }

    /// Clears all pulses from a pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::new(4, 2, 0);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// pattern.clear();
    /// assert_eq!([false, false, false, false], pattern.as_slice());
    /// ```
    pub fn clear(&mut self) {
        self.steps.clear();
        for _ in 0..self.length {
            self.steps.push(false).unwrap();
        }
    }

    /// Resize the current pattern. If length is
    ///
    /// # Arguments
    ///
    /// * `length` - Total number of steps
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::with_length(1);
    /// assert_eq!([false], pattern.as_slice());
    /// pattern.resize(4);
    /// assert_eq!(4, pattern.len());
    /// assert_eq!([false, false, false, false], pattern.as_slice());
    /// ```
    pub fn resize(&mut self, mut length: usize) {
        if length + self.padding > 64 {
            length = 64 - self.padding;
        }

        self.steps.resize(length + self.padding, false).unwrap();
        self.length = length;

        if self.cursor >= self.total_length() - 1 {
            self.cursor = 0;
        }
    }

    pub fn set_padding(&mut self, padding: usize) {
        if padding + self.length > 64 {
            self.padding = 64 - self.length;
        } else {
            self.padding = padding.max(0);
        }

        self.pulses(self.pulses);
    }

    pub fn padding(&self) -> usize {
        self.padding
    }

    /// Moves the pattern cursor to the first step
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::new(4, 2, 0);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// assert_eq!(Some(true), pattern.next());
    /// pattern.reset();
    /// assert_eq!(Some(true), pattern.next());
    /// ```
    pub fn reset(&mut self) {
        self.move_cursor(0);
    }

    /// Moves the pattern cursor to a given step. If step overflows, it will move to the last step
    ///
    /// # Arguments
    ///
    /// * `step` - Step identifiyer. Range starts at 0
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::new(4, 2, 0);
    /// assert_eq!([true, false, true, false], pattern.as_slice());
    /// assert_eq!(Some(true), pattern.next());
    /// assert_eq!(Some(false), pattern.next());
    /// pattern.move_cursor(1);
    /// assert_eq!(Some(false), pattern.next());
    /// ```
    pub fn move_cursor(&mut self, step: usize) {
        self.cursor = if self.is_in_range(step) {
            step
        } else {
            self.last_index()
        };
    }

    pub fn increment_cursor(&mut self) {
        self.cursor += 1;

        if self.cursor >= self.steps.len() {
            self.cursor = 0;
        }
    }

    pub fn current_step(&mut self) -> bool {
        self.steps[self.cursor]
    }

    /// Returns the state of a step
    ///
    /// # Arguments
    ///
    /// * `step` - Step identifiyer. Range starts at 0
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::<64>::new(4, 2, 0);
    /// assert_eq!(Some(true), pattern.step(0));
    /// assert_eq!(Some(false), pattern.step(1));
    /// assert_eq!(None, pattern.step(4));
    /// ```
    pub fn step(&self, step: usize) -> Option<bool> {
        if step < self.len() {
            Some(self.steps[step])
        } else {
            None
        }
    }

    /// Returns length of current pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::<64>::new(8, 2, 0);
    /// assert_eq!(8, pattern.len());
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn total_length(&self) -> usize {
        self.steps.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns a boolean slice reprensenting the pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let pattern = Pattern::<64>::new(4, 2, 1);
    /// assert_eq!([false, true, false, true], pattern.as_slice());
    /// ```
    pub fn as_slice(&self) -> &[bool] {
        self.steps.as_slice()
    }

    /// Returns the next step in a pattern. If the end of the pattern is reached, it resets
    /// the cursor and will return the first step
    ///
    /// # Examples
    ///
    /// ```
    /// use rhythms::Pattern;
    /// let mut pattern = Pattern::<64>::new(2, 1, 0);
    /// assert_eq!(true, pattern.next_looped());
    /// assert_eq!(false, pattern.next_looped());
    /// assert_eq!(true, pattern.next_looped());
    /// assert_eq!(false, pattern.next_looped());
    /// // ...
    /// ```
    pub fn next_looped(&mut self) -> bool {
        let step = self.steps[self.cursor];
        if self.cursor == self.last_index() {
            self.reset();
        } else {
            self.move_cursor(self.cursor + 1);
        }
        step
    }

    fn is_in_range(&self, step: usize) -> bool {
        step < self.steps.len()
    }

    fn last_index(&self) -> usize {
        self.steps.len() - 1
    }
}

/// Iterate over a pattern
///
/// ```
/// use rhythms::Pattern;
/// let pattern = Pattern::<64>::new(8, 2, 0);
/// for step in pattern {
///     println!("{}", step);
/// }
/// ```
impl<const MAX_STEPS: usize> Iterator for Pattern<MAX_STEPS> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.is_in_range(self.cursor) {
            let current = self.cursor;
            self.cursor += 1;
            Some(self.steps[current])
        } else {
            self.reset();
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bjorklund_example() {
        let pattern = Pattern::<64>::new(13, 5, -3, 0);
        assert_eq!(
            [
                true, false, false, true, false, true, false, false, true, false, true, false,
                false
            ],
            pattern.as_slice()
        );
    }

    #[test]
    fn ruchenitza() {
        let pattern = Pattern::<64>::new(7, 3, -3, 0);
        assert_eq!(
            [true, false, true, false, true, false, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn york_samai() {
        let pattern = Pattern::<64>::new(6, 5, 1, 0);
        assert_eq!([true, false, true, true, true, true], pattern.as_slice());
    }

    #[test]
    fn cumbia() {
        let pattern = Pattern::<64>::new(4, 3, 1, 0);
        assert_eq!([true, false, true, true], pattern.as_slice());
    }

    #[test]
    fn khafif_e_ramal() {
        let pattern = Pattern::<64>::new(5, 2, -3, 0);
        assert_eq!([true, false, true, false, false], pattern.as_slice());
    }

    #[test]
    fn agsag_samai() {
        let pattern = Pattern::<64>::new(9, 5, 1, 0);
        assert_eq!(
            [true, false, true, false, true, false, true, false, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn venda() {
        let pattern = Pattern::<64>::new(12, 5, 0, 0);
        assert_eq!(
            [true, false, false, true, false, true, false, false, true, false, true, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn bendir() {
        let pattern = Pattern::<64>::new(8, 7, 1, 0);
        assert_eq!(
            [true, false, true, true, true, true, true, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn overflow() {
        let pattern = Pattern::<64>::new(8, 9, 0, 0);
        assert_eq!(
            [true, true, true, true, true, true, true, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn zero_length() {
        let pattern = Pattern::<64>::with_length(0);
        assert_eq!(0, pattern.len());
    }

    #[test]
    fn zero_pulses() {
        let mut pattern = Pattern::<64>::with_length(2);
        pattern.pulses(0);
        assert_eq!([false, false], pattern.as_slice());
    }

    #[test]
    fn padding() {
        let pattern = Pattern::<64>::new(12, 5, 5, 0);
        assert_eq!(
            [
                true, false, false, true, false, true, false, false, true, false, true, false,
                false, false, false, false, false
            ],
            pattern.as_slice()
        );
    }
}
