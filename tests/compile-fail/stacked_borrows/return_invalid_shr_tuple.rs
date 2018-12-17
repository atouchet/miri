// Make sure that we cannot return a `&` that got already invalidated, not even in a tuple.
fn foo(x: &mut (i32, i32)) -> (&i32,) {
    let xraw = x as *mut (i32, i32);
    let ret = (unsafe { &(*xraw).1 },);
    unsafe { *xraw = (42, 23) }; // unfreeze
    ret //~ ERROR is not frozen
}

fn main() {
    foo(&mut (1, 2));
}