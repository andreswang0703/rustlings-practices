fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;
    let v = a.to_vec();

    (a, v)
}

fn main() {
    let (arr, vect) = array_and_vec();
    println!("{:?}", arr);
    println!("{:?}", vect);

    println!("length of array: {}", arr.len());
    println!("length of vect: {}", vect.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
