use std::collections::*;
use std::*;
fn compress<T: Ord + Hash + Copy>(a: &Vec<T>) -> HashMap<T, usize>
where
    T: PartialEq,
{
    let mut b: Vec<T> = vec![];
    for x in a.iter().copied() {
        b.push(x);
    }
    b.sort();
    b.dedup();

    let map: HashMap<T, usize> = b.iter().enumerate().map(|(i, &s)| (s, i)).collect();
    map
}
