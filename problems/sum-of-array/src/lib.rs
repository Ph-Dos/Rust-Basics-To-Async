pub fn sum_array(arr: &[i32]) -> i32 {
    // TODO: Implement the function here

    let mut sum = 0;

    // The following method has the most syntactic sugar,
    // rust by default does not have C++ type for loops,
    // so rust is implicitly using an iterator and a while loop here.

    // for i in arr {
    //     sum += i;
    // }

    let mut iter = arr.into_iter();

    while let Some(e) = iter.next() {
        sum += e;
    }

    sum
}
