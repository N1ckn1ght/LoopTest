use std::io;

fn main() {
    let mut state: u16  = 0;
    let mut clock: u128 = 0;

    loop {
        match state {
            0 => {
                listen(&mut state, &mut clock);
            },
            1 => {
                clock += 1;
                if clock & 2047 == 0 {
                    println!("CURRENT CLOCK: {}", clock);
                    listen(&mut state, &mut clock);
                }
            },
            _ => {
                panic!("Huh?");
            }
        }
    }
}

fn listen(state: &mut u16, clock: &mut u128) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {
            let line = input.trim().split(" ").collect::<Vec<&str>>();
            match line[0] {
                "start" => {
                    *state = 1;
                },
                "stop" => {
                    *state = 0;
                },
                "quit" => {
                    panic!("Ok");
                },
                "set" => {
                    *clock = line[1].parse::<u128>().unwrap();
                }
                _ => {
                    println!("Wrong command");
                }
            }
        },
        Err(_no_updates_is_fine) => {
            
        },
    }
}