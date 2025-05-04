fn main() {
    
    // 1. main() makes a function call to heap()

    // 5. The value stored in the local variable in heap() (which is just and address)
    //    is now allocated on the main stack frame as a return and type inference.
    //    the value stored address stored was passed by value.
    //
    //    thus the memory is shared between the two functions which is the main purpose of the
    //    heap!

    let result = heap();
}

// 2. a stack frame is allocated for heap() in the stack memory region.

fn heap() -> Box<i32> {

    // 3. the function allocates an i32 on the heap by type inference.
    //    the stack allocated enough memory for box in a local variable,
    //    b/c box is just a pointer all it needs is a fixed sized address to a heap memory
    //    location.

    let b = Box::new(5);

    // 4. the function completes and returns the box pointer,
    //    the local variable b becomes logically invalid,
    //    thus the heap memory can no longer be accessed via the local variable b
    //    rust would usally detect this unreachable heap memory and free it but we return it.

    b
}
