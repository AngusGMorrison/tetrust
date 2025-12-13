use tetrust::{Block, Blocks};

fn main() {
    {
        let mut o = Blocks::O;
        println!("{}", o);

        o.rotate();
        println!("{}", o)
    }

    {
        let mut i = Blocks::I { orientation_idx: 0 };
        for j in 0..4 {
            println!("{}", i);
            i.rotate();
        }
    }
}
