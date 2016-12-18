// Bonus points for macro use and general base_n implementation :)

extern crate libc;
extern crate rpgffi as pg;
use std::ffi::CString;

// Emulate C PG_GETARG_XXX macros from fmgr.h
macro_rules! pg_get_type_fun {
    ($ty:ty) => {|fcinfo: pg::FunctionCallInfo, n: usize| {
       let arg = unsafe {(*fcinfo).arg[n] as $ty};
       arg
     }
   };
}

// Few points here:
// 1. This is the function we have registered in magic.c, this means it will have the fcinfo param passed in, this is equivalent to C PG_FUNCTION_ARGS.
// 2. All unsafeness is where it is necessary, these functions can be.
// 3. #[no_mangle] ensures C will be able to execute.
#[no_mangle]
pub extern fn base36_encode(fcinfo: pg::FunctionCallInfo) -> *mut pg::text {
  // Generate and execute proper get closure for fun and profit
  let pg_get_i32 = pg_get_type_fun!(i32);
  let v = pg_get_i32(fcinfo, 0);

  let result_cstring = base_n_encode(v, 36);
  // Unsafely return, this return is equivalent to C PG_RETURN_TEXT_P
  unsafe {
    pg::cstring_to_text(
      result_cstring.as_ptr()
    )
  }
}

// Keep base function nice and safe, pure rust
// Difference from the ref C implementatin is that we will prepend '-' to negative integers
fn base_n_encode(input: i32, base: i32) -> CString {
  let signed = input < 0;
  let mut inpt = input.abs();
  let alphabet: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
  let mut result_vec: Vec<char> = vec!();
  loop {
    result_vec.push(alphabet[(inpt % base) as usize]);
    match inpt / base {
      x if x >= 1 => inpt = x,
      _ => break
    }
  }
  if signed {
    result_vec.push("-".chars().next().unwrap());
  }
  let result_string: String = result_vec.into_iter().rev().collect();
  match CString::new(result_string) {
    Ok(x) => x,
    Err(e) => panic!("{:?}", e)
  }
}


