const STARTING_MISSLES: i32 = 8;
const READY_MISSLES: i32 = 2;

fn main() {
    let (ready, mut missles): (i32,i32) = (READY_MISSLES,STARTING_MISSLES);

    println!("Firing {} of my {} missles...", ready,missles);
    missles = missles - ready;
    println!("{} missles left", missles);

   // println!("{} missles left", missles-ready);
}
