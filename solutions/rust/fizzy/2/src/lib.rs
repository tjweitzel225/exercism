/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    predicate: fn(T) -> bool,
    word: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Matcher {
            predicate: _matcher,
            word: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<F>(Vec<Matcher<F>>);

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.0.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where
        T: Copy + ToString,
        I: Iterator<Item = T>,
    {
        _iter.map(move |item| {
            let word: String = self
                .0
                .iter()
                .filter(|m| (m.predicate)(item))
                .map(|m| m.word.as_str())
                .collect();
            if word.is_empty() {
                item.to_string()
            } else {
                word
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: PartialEq + std::ops::Rem<Output = T> + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|i| i % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|i| i % T::from(5) == T::from(0), "buzz"))
}
