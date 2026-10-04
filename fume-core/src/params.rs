/// A value that can be serialised into a URL query parameter.
///
/// `Param` associates a compile-time name with a value, so endpoint
/// implementations can build queries without repeating string literals and
/// without losing type information.
pub trait Param {
    /// The query parameter name.
    const NAME: &'static str;
    /// The query parameter value.
    fn value(&self) -> String;
}

/// An owned, ordered collection of query parameters.
///
/// Values are stored as already-rendered strings. The transport is responsible
/// only for percent-encoding them, which keeps every backend consistent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Query(Vec<(&'static str, String)>);

impl Query {
    /// Create an empty query.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Append a raw `name = value` pair.
    #[must_use]
    pub fn push(mut self, name: &'static str, value: impl ToString) -> Self {
        self.0.push((name, value.to_string()));
        self
    }

    /// Append an optional raw `name = value` pair, skipping `None`.
    #[must_use]
    pub fn push_opt(mut self, name: &'static str, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.0.push((name, value.to_string()));
        }
        self
    }

    /// Append a typed [`Param`].
    #[must_use]
    pub fn param<P: Param + ?Sized>(mut self, value: &P) -> Self {
        self.0.push((P::NAME, value.value()));
        self
    }

    /// Append an optional typed [`Param`], skipping `None`.
    #[must_use]
    pub fn param_opt<P: Param + ?Sized>(mut self, value: Option<&P>) -> Self {
        if let Some(value) = value {
            self.0.push((P::NAME, value.value()));
        }
        self
    }

    /// Iterate over the `(name, value)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
        self.0.iter().map(|(name, value)| (*name, value.as_str()))
    }

    /// Number of parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether there are no parameters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
