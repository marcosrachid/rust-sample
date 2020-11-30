fn scope_and_shdowing() {
    let a = 123;

    // let a = 777;

    {
        let b = 456;
        println!("inside b = {}", b);

        let a = 777; // shadow de outer value
        println!("inside a = {}", a);
    }

    // println!("outside b = {}", b); impossible

    println!("outside a = {}", a);
}

fn main() {
    scope_and_shdowing();
}
