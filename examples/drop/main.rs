// Originally from https://stackoverflow.com/questions/48227347/does-rust-free-up-the-memory-of-overwritten-variables

struct Thing;

impl Drop for Thing {
    fn drop(&mut self) {
        println!("Dropping a Thing");
    }
}

fn main() {
    println!("Example without calling drop() manually:");
    {
        println!("0");
        let _x = Thing;

        println!("1");
        let _x = Thing;
        println!("2");
    }
    println!("End of example calling drop() manually");
    println!("");
    println!("Example with calling drop() manually:");
    {
        println!("0");
        let x = Thing;
        drop(x);
        println!("1");
        let _x = Thing;
        println!("2");
    }
    println!("End with calling drop() manually");
}
