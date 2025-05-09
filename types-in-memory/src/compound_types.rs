fn main() {

    // Tuples allocates each type contiguously in memory, it's like an array but each element can
    // be a different type.
    // The size of a type is fixed and known at compile time thus is stored on the stack frame.
    
    let a: (char, bool, i32) = ('a', true, 48); // -> {1 Byte Of char}{1 Byte of bool}{4 Bytes of int}


    // c is a non-owning pointer, which means the memory c points to is NOT de-allocated when
    // c is dropped, non-owning pointers are simply called references in Rust.
    // A pointer in rust only has a single machine word.
    // 1. the memory address to b which is located in this stack frame (main)
    
    let b: i32 = 15;

    let c: &i32 = &b;

    // An array is has fixed size that is known at compile time so it is allocated in the stack
    // frame.
    // This specific array of i32 which for my cpu is less then the size of a machine word.
    // So this can be defined as an array of machine words.

    let a: [i32; 2] = [22, 33, 44];

    // A vector the resizable version of an array, it's size is not fixed so it's values are stored
    // on the head. The Vec structure is mapped to v, the structure contains 3 machine words that
    // allow the munipulation of the allocated heap data,
    // 1. A pointer to the heap data
    // 2. The capacity of the available memory
    // 3. the length of the vector (current uniform chunks of memory used)

    let v: Vec<i32> = vec![22, 33, 44];
}
