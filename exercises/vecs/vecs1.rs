// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // 方法1：
    let v = Vec::from(a);
    // 方法2：
    // let mut v: Vec<i32> = Vec::new();
    // for ele in a {
    //     v.push(ele);
    // }
    // 方法3:
    // let mut v: Vec<i32> = Vec::new();
    // v.extend(a);
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
