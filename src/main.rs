extern crate rand;
extern crate rcmd;

use rand::{ OsRng, Rng };
use rcmd::RollCommand;
use std::error::Error;

fn main() {
    // attempt to retrieve randomness from the os
    let mut rng = match OsRng::new() {
        Ok(rng) => rng, 
        Err(e) => {
            println!("{}", e.description());
            return;
        }, 
    };

    // 1. get command von args
    // 2. filtermap args as rollcommands, discarding failures.
    // 3. Map commands to results.
    // 4. Collect results into a vector.
    let rolls: Vec<_> = std::env::args()
        .filter_map(|arg| arg.parse::<RollCommand>().ok())
        .map(|cmd| cmd.result(|max| rng.gen_range(0, max) + 1))
        .collect();

    // Print results
    for roll in rolls {
        println!("{}", roll);
    }

}