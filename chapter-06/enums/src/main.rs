fn main() {
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_kind: IpAddrKind) {}
    }

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
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                println!("{:?}", &self)
                // method body would be defined here
            }
        }

        let msg = Message::Write(String::from("Hello"));
        msg.call()
    }

    {
        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;
    }
}
