pub fn data_types() -> (u8, f64, bool, char) {
    // 1. Define variable of type `u8` and value `42`
    let x: u8 = 42;

    // 2. Define variable of type `f64` and value `3.14`
    let y: f64 = 3.14;

    // 3. Define variable of type `bool` and value `false`
    let z: bool = false;

    // 4. Define variable of type `char` and value `a`
    let w: char = 'a';

    // 5. Return a tuple with the variables in the order they were defined
    
    // The following is the rebose version of the syntax, which is more readable then
    // the simple implicit syntax.
    // let t: (u8, f64, bool, char) = (x, y, z, w);
    // t

    // Rust can infer you want to create and return a tuple by type inference and the last
    // statement has no ;
    // in Rust tuples are created with adding () around a list of values.
    (x, y, z, w)
}
