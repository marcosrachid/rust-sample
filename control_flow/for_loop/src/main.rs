fn for_loop() {
    for x in 1..11 
    {
        if x == 3 { continue; }

        println!("x = {}", x);

        if x == 8 { break; }
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn main() {
    for_loop();
}
