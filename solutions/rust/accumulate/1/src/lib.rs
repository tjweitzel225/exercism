pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut res = Vec::with_capacity(input.len());
    for v in input {
        res.push(function(v))
    }
    res
}
