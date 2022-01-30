pub fn permutations<T: Copy>(elems: &[T]) -> Vec<Vec<T>> {
    if elems.is_empty() {
        // Just the empty permutation.
        return vec![vec![]];
    }

    let n = elems.len();
    let mut ret = Vec::with_capacity((1..=n).product()); // n!

    let x = elems[0];
    for sub_perm in permutations(&elems[1..]) {
        for i in 0..=sub_perm.len() {
            let mut perm = Vec::with_capacity(n);
            perm.extend(&sub_perm[..i]);
            perm.push(x);
            perm.extend(&sub_perm[i..]);

            ret.push(perm);
        }
    }

    ret
}
