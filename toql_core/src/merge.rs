//! Generic merge function called by code from Toql derive.
//! Used to merge a collection of structs into another collection of structs by equal keys

use std::collections::HashMap;
pub fn merge<T, O, K, F, X, Y>(
    this: &mut std::vec::Vec<T>,
    mut other: Vec<O>,
    tkey: X,
    okey: Y,
    assign: F,
) where
    O: Clone,
    K: Eq + std::hash::Hash,
    F: Fn(&mut T, O),
    X: Fn(&T) -> Option<K>,
    Y: Fn(&O) -> Option<K>,
{
    // Build index to lookup all books of an author by author id
    let mut index: HashMap<K, Vec<usize>> = HashMap::new();

    for (i, b) in this.iter().enumerate() {
        match tkey(&b) {
            Some(k) => {
                let v = index.entry(k).or_insert(Vec::new());
                v.push(i);
            }
            None => {}
        }
    }

    // Consume all authors and distribute
    for a in other.drain(..) {
        // Get all books for author id
        match &okey(&a) {
            Some(ok) => {
                let vbi = index.get(ok).unwrap();

                // Clone author for second to last books
                for bi in vbi.iter().skip(1) {
                    if let Some(mut b) = this.get_mut(*bi) {
                        assign(&mut b, a.clone());
                    }
                }

                // Assign drained author for first book
                let fbi = vbi.get(0).unwrap();
                if let Some(mut b) = this.get_mut(*fbi) {
                    assign(&mut b, a.clone());
                }
            }
            None => {}
        }
    }
}
