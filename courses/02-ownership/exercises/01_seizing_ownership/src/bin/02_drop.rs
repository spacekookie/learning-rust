// In what order are the println statements printed?

#[derive(Debug)]
struct DropMe;

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Dropping 'DropMe'");
    }
}

fn main() {
    println!("Start");
    {
        let x = DropMe;
        println!("x: {:?}", x);
    }

    println!("End");
}
