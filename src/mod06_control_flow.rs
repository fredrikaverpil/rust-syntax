#![allow(dead_code)]

fn if_statement() {
    let temp = 15;

    if temp > 30 {
        println!("hot");
    } else if temp < 10 {
        println!("cold");
    } else {
        println!("ok")
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    );

    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    );
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        } // take control flow back to top of loop
        println!("x = {}", x);
    }

    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10
        // 1^10
        {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11
    // range
    {
        if x == 3 {
            continue;
        } else if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..40).enumerate()
    // 1: 31, 2: 32, 3: 33 .. 9: 39
    {
        println!("{}: {}", pos, y)
    }
}

fn match_statement() {
    let country_code = 1;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        1..=999 => "Unknown", // includes 999
        _ => "Invalid",
    };
    println!("Country with code {} is {}", country_code, country);
}

enum State {
    Locked,
    Failed,
    Unlocked,
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

pub fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
    // combination_lock();
}
