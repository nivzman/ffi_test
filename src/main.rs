extern crate libc;

use libc::{size_t, c_int};

#[link(name = "damm")]
extern "C" {
    fn square(n: size_t) -> size_t;

    fn output_param(input: size_t, out: *mut size_t);

    fn fill(input: *const u8,
            input_len: size_t,
            output: *mut u8,
            output_len: size_t) -> c_int;

    fn get_struct() -> Struct;

    fn get_struct_ptr() -> *mut Struct;
}

#[repr(C)]
#[derive(Debug)]
struct Struct {
    a: c_int,
    b: u8,
}

fn output_param_wrap(n: usize) -> usize {
    unsafe {
        let mut output: size_t = 0;
        output_param(n as size_t, &mut output);
        output as usize
    }
}

fn fill_wrap(input: &[u8], output_len: usize) -> (i32, Vec<u8>) {
    unsafe {
        let input_len = input.len() as size_t;
        let input_ptr = input.as_ptr();

        let mut output = Vec::with_capacity(output_len);

        let output_ptr = output.as_mut_ptr();

        let full_inputs = fill(input_ptr, input_len, output_ptr, output_len as size_t);

        (full_inputs as i32, output)
    }
}

fn main() {
    let x = unsafe { square(5) };
    println!("{}", x);

    let x2 = output_param_wrap(5);
    println!("{}", x2);

    let (full_inputs, mut output) = fill_wrap("ab".as_bytes(), 9);
    println!("full inputs : {}", full_inputs);
    let s = unsafe { String::from_raw_parts(output.as_mut_ptr(), output.capacity(), output.capacity()) };
    println!("{}", s);

    let s2 = unsafe { get_struct() };
    println!("{:?}", s2);

    let s_ptr = unsafe { get_struct_ptr() }; // memory leak, no one cleans this struct
    unsafe {
        println!("{:?}", *s_ptr);
    }
}
