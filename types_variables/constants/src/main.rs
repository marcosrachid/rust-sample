const MEANING_OF_LIFE:u8 = 42; // no fixed address

static w:i32 = 123;
static mut z:i32 = 456;

fn main() {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", w);
    unsafe
        {
            println!("{}", z); // cannot be mutable but can run inside unsafe statement
        }
}
