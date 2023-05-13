mod work;

use work::work::Work;

fn main() {
    print!("Hello, world!\n");
    let mut work = Work::new();
    
    while work.parachute < 100 {
        work.roll_contracts();
        work.assign_unions_to_contracts();
        work.roll_attempts();
        if work.money == 0 {
            println!("You have no money left!");
            break;
        }
        work.spend_money();
        work.improve_contracts();
    }
}