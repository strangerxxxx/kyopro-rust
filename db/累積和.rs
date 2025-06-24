fn main() {
    let mut s = iter::once(0)
        .chain(a.iter().scan(0, |state, &x| {
            *state = *state + x;
            Some(*state)
        }))
        .collect::<Vec<_>>();
}
use std::ops::Add;
fn prefix_sum_with_zero<T>(a: &[T], start: T) -> Vec<T>
where
    T: Add<Output = T> + Clone,
{
    iter::once(start)
        .chain(a.iter().scan(start.clone(), |state, x| {
            *state = state.clone() + x.clone();
            Some(state.clone())
        }))
        .collect()
}
