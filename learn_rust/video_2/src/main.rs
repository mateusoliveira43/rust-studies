// #![deny(warnings)]

const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    let total_in_hours = 40;
    println!("Trabalhou {} segundos", total_in_hours * SECONDS_IN_HOUR);
}
