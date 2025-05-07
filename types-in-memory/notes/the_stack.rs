
// main serves as the entery point for execution,
// b/c this is the first place the stack pointer will point.
// The stack pointer will move up and down and if not invalid logic happens,
// will return back to main() and end the program.
//
// main() contains the follwing in it's stack frame.
// 1. local value a (b/c fixed size known a compile time)
// 2. local value b

fn main() {
    let a = 3;
    let b = sqaure(a);
}


// This contains with in its stack frame
// 1. the parameter passed
// 2. the value from doing the operation
// 3. the return address to main() which is the next instruction main has, which is to end

fn sqaure(num: i32) -> i32 {
    let c = num * num;

    // -> &c 
    // Returing a reference to a local variable would not compile,
    // when sqaure() completes, the stack pointer moves back down to main()
    // this causes the stack frame of sqaure() (the memory allocated for this function)
    // to be logically marked as invalid (due to undefined behavior), and will wait to be overwritten be some other stack frame
    // by a preceding function call.
    // This is b/c the stack pointer does not actually deallocate/earase stack frame memory.

    c
}
