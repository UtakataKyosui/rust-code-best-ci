use vstd::prelude::*;

verus! {

fn increment(value: u64) -> (result: Option<u64>)
    ensures
        match result {
            Some(next) => next as int == value as int + 1,
            None => value == u64::MAX,
        },
{
    if value < u64::MAX {
        Some(value + 1)
    } else {
        None
    }
}

}

fn main() {}
