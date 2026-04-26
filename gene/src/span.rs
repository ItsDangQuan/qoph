//! Source-location primitives shared by the generators.

#[derive(Debug, Clone, Copy, PartialEq)]
/// A 1-based line and column position in the source input.
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    /// Creates a new location.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Returns the 1-based line number.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Returns the 1-based column number.
    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// An inclusive span over source input.
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    /// Creates a new span.
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    #[allow(unused)]
    /// Returns the start location.
    pub fn start(&self) -> Location {
        self.start
    }

    #[allow(unused)]
    /// Returns the end location.
    pub fn end(&self) -> Location {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Span};

    #[test]
    fn location_accessors_return_constructor_values() {
        let loc = Location::new(4, 12);
        assert_eq!(loc.line(), 4);
        assert_eq!(loc.column(), 12);
    }

    #[test]
    fn span_accessors_return_boundaries() {
        let start = Location::new(2, 3);
        let end = Location::new(2, 8);
        let span = Span::new(start, end);

        assert_eq!(span.start(), start);
        assert_eq!(span.end(), end);
    }
}
