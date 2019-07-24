fn main() {
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
    }
    {
        // in rust, there is no null. there is only an Option<T> {Some, None};
        // if you use option: you are enforced to implement case for both Some and None.

        let some: Option<i32> = Some(8);
        println!("outcome of Option Some: {}", some.unwrap_or_else(|| 0) + 2);

        let some: Option<i32> = None;
        println!("outcome of Option None: {}", some.unwrap_or_else(|| 0) + 2);

    }
    {

        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    match state {
                        UsState::Alabama => println!("Alabama kult! {:?}", state),
                        UsState::Alaska => println!("Alaska meh! {:?}", state),
                    }
                    25
                },
            }
        }

        println!("value of quarter in cents {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
        println!("value of quarter in cents {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
        println!("value of dime in cents {}", value_in_cents(Coin::Dime));
    }

    {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }

    {
        // if let'
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        if let Some(3) = some_u8_value {
            println!("three");
        }
    }

}
