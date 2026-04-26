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

    /// Returns the start location.
    pub fn start(&self) -> Location {
        self.start
    }

    /// Returns the end location.
    pub fn end(&self) -> Location {
        self.end
    }
}
