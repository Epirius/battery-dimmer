use std::process::Command;

use battery::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = battery::Manager::new()?;
    let battery = manager.batteries().unwrap().next().unwrap()?;
    let charge: f32 = battery.state_of_charge().try_into()?;
    let state = battery.state();

    println!("state {:?}", state);
    println!("charge {:?}", charge);
    if state == State::Charging {
        std::process::exit(0);
    }

    let percentage = match charge {
        charge if charge < 0.10 => 25,
        charge if charge < 0.25 => 40,
        _ => std::process::exit(0),
    };

    // TODO find set_brightness in a generic way. maybe via which
    let command = format!("~/bin/set_brightness {}", percentage);
    let output = Command::new("sh").args(["-c", &command]).output();
    match output {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
