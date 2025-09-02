fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("{max}"),
    //     _ => ()
    // };
    if let Some(max) = config_max {
        println!("{max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("quarter from {state:?}!"),
    //     _ => count += 1
    // }
    if let Coin::Quarter(state) = coin {
        println!("quater from {state:?}!");
    } else {
        count += 1;
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?}: old"))
    } else {
        Some(format!("{state:?}: new"))
    }
}