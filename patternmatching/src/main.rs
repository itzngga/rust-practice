enum Coin {
    Head,
    Flag(Asia),
}

#[derive(Debug)]
enum Asia {
    Indonesia,
    Singapore,
    Malaysia,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match coin {
    //     Coin::Head => {
    //         println!("You are lucky!");
    //         return 1
    //     },
    //     Coin::Flag => 5,
    // }

    // match coin {
    //     Coin::Head => 1,
    //     Coin::Flag(state) => {
    //         println!("You are from {:?}", state);
    //       return  25
    //     },
    // }

    let mut count: u8 = 0;
    if let Coin::Flag(location) = Coin::Flag(Asia::Singapore) {
        println!("Location from {:?}", location);
        count
    } else {
        count += 1;
        count
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    value_in_cents(Coin::Flag(Asia::Singapore));
   //
   //  let five = Some(5);
   //  let six = plus_one(five);
   //  println!("{:?}", six.unwrap());
   //
   //  let dice_roll = 9;
   //  match dice_roll {
   //      3 => buy_property(),
   //      7 => buy_ticket(),
   //      _ => reroll(),
   //      // other => go_to_jail(other),
   //  }
   //
   //  let config_max = Some(100u8);
   //  match config_max {
   //      Some(max) => println!("The maximum is {}", max),
   //      _ => (),
   //  }
   //
   // if let Some(max) = config_max {
   //     println!("The maximum is {}", max);
   // }

}

fn reroll(){}
fn buy_ticket() {}
fn buy_property() {}
fn go_to_jail(other: i32) {}