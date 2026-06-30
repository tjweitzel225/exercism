use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marks: Vec<bool> = vec![false; (upper_bound + 1) as usize];
    let mut primes = Vec::new();
    for i in 2..=upper_bound as usize {
        let marked = &mut marks[i];
        if !*marked {
            *marked = true;
            primes.push(i as u64);
            let mut p = i;
            while p <= upper_bound as usize {
                marks[p] = true;
                p += i;
            }
        }
    }
    primes
}
