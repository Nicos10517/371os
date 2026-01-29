fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut [i32]) {

    //Hello! I'm over commenting my code to ensure I understand since I'm doing a bit of catch-up.
    //Hope it's not too atrocious on the eyes.

    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    //slice is pointer to data and the length of a slice
    //as_mut_ptr() returns a raw pointer with type *mut i32


    assert!(mid <= len);

    //asserts that mid index is within the slice
    //slice starts at ptr and is mid items long
    

    //Unsafe in rust just means the compiler cannot verify the memory safety of a code block or
    //function. By nature, the compiler will reject valid programs rather than accept invalid
    //programs. Unsafe rust is essentially an override.

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr,mid),
            std::slice::from_raw_parts_mut(ptr.add(mid),len - mid),

            //add method is also unsafe because it must trust that the offset location is also a
            //valid pointer
        )
    }

    //this setup lets you call this function from safe Rust. "Unsafe code in a safe way", because
    //it creates only valid pointers from the data this function has access to.
}
