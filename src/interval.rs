//! Interval arithmetic for the raytracer.
//!
//! This module provides an `Interval` struct that represents a closed interval
//! on the real number line. It's used for various purposes in the raytracer,
//! such as bounding boxes, ray parameter ranges, and color clamping.

/// A closed interval on the real number line.
///
/// An interval is defined by its minimum and maximum values, representing
/// the range [min, max]. It provides operations for checking containment,
/// clamping values, and working with special intervals like empty and universe.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    /// The minimum value of the interval (inclusive)
    pub min: f64,
    /// The maximum value of the interval (inclusive)
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    /// Creates a new interval with the given bounds.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive)
    /// * `max` - The maximum value (inclusive)
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Returns the size (width) of the interval.
    ///
    /// # Returns
    ///
    /// The difference between max and min
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if a value is contained within the interval (inclusive bounds).
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check
    ///
    /// # Returns
    ///
    /// True if x is in [min, max]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a value is strictly contained within the interval (exclusive bounds).
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check
    ///
    /// # Returns
    ///
    /// True if x is in (min, max)
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps a value to the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to clamp
    ///
    /// # Returns
    ///
    /// The value clamped to [min, max]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }

        if x > self.max {
            return self.max;
        }

        x
    }

    /// Creates an empty interval.
    ///
    /// An empty interval has min = ∞ and max = -∞, representing
    /// the empty set of real numbers.
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    /// Creates a universe interval.
    ///
    /// A universe interval has min = -∞ and max = ∞, representing
    /// the set of all real numbers.
    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Default for Interval {
    /// Creates a default empty interval.
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}
