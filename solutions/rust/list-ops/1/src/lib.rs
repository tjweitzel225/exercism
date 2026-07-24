struct Append<A, B>(A, B);
impl<A, B> Iterator for Append<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().or_else(|| self.1.next())
    }
}
pub fn append<I, J>(_a: I, _b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Append(_a, _b)
}

struct Concat<O>
where
    O: Iterator,
    O::Item: Iterator,
{
    outer: O,
    inner: Option<O::Item>,
}
impl<O> Iterator for Concat<O>
where
    O: Iterator,
    O::Item: Iterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.inner.as_mut().and_then(|inner| inner.next()) {
            return Some(item);
        }
        self.inner = Some(self.outer.next()?);
        self.next()
    }
}
pub fn concat<I>(_nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concat {
        outer: _nested_iter,
        inner: None,
    }
}

struct Filter<I, F>(I, F);
impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;
        if self.1(&next) {
            Some(next)
        } else {
            self.next()
        }
    }
}
pub fn filter<I, F>(_iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter(_iter, _predicate)
}

pub fn length<I: Iterator>(_iter: I) -> usize {
    let mut c = 0;
    for _ in _iter {
        c += 1
    }
    c
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
struct Map<I, F> {
    iter: I,
    func: F,
}
impl<I, F, U> Iterator for Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.func)(self.iter.next()?))
    }
}
pub fn map<I, F, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map::<I, F> {
        iter: _iter,
        func: _function,
    }
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut curr = _initial;
    for i in _iter {
        curr = _function(curr, i)
    }
    curr
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut curr = _initial;
    while let Some(i) = _iter.next_back() {
        curr = _function(curr, i)
    }
    curr
}

struct Reverse<I>(I);
impl<I: DoubleEndedIterator> Iterator for Reverse<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}
pub fn reverse<I: DoubleEndedIterator>(_iter: I) -> impl Iterator<Item = I::Item> {
    Reverse(_iter)
}
