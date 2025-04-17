use logomotion::{func, log};

fn foo() {
    let ctx = func!("foo()");
    log!("[foo 1]");
    bar(1);
    log!("[foo 2]")
}

fn bar(a: u32) {
    let ctx = func!("bar(a: {a})");
    log!("[bar 1]");
    baz(1, 2);
    log!("[bar 2]");
}

fn baz(a: u32, b: u32) {
    let ctx = func!("bar(a: {a}, b: {b})");
    log!("[baz]");
}

fn main() {
    let ctx = func!("main()");
    foo();
}
