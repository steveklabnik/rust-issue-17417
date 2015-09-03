extern crate libc;

extern {
    fn c_func(x: *mut *mut libc::c_void);
}

#[cfg(feature = "fail")]
fn main() {
    let x = 0 as *mut u8;
    unsafe { c_func(&mut (x as *mut libc::c_void)); } ;
    println!("new pointer is {:p}", x);
}

#[cfg(not(feature = "fail"))]
fn main() {
    let mut x = 0 as *mut u8;
    unsafe { c_func((&mut x) as *mut _ as *mut *mut libc::c_void); } ;
    println!("new pointer is {:p}", x);
}
