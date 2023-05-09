fn main() {
    {
        print_labeled_measurement(5, 'h');
    }

    {
        // let x = (let y = 6);
    }

    {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }

    {
        let x = five();

        println!("The value of x is: {x}");

        fn five() -> i32 {
            5
        }
    }

    {
        let x = plus_one(5);

        println!("The value of x is: {x}");

        fn plus_one(x: i32) -> i32 {
            x + 1
        }
    }

    {
        // hello, world

        // So we’re doing something complicated here, long enough that we need
        // multiple lines of comments to do it! Whew! Hopefully, this comment will
        // explain what’s going on.

        let lucky_number = 7; // I’m feeling lucky today
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
