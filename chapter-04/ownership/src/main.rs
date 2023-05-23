fn main() {
    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{}", s); // This will print `hello, world!`
    }

    {
        let x = 5;
        let y = x;

        let mut s1 = String::from("hello");
        let s2 = s1;

        // println!("{}", s1)
    }

    {
        let s1 = String::from("hel\0lo");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward

        fn takes_ownership(some_string: String) {
            // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing
          // memory is freed.

        fn makes_copy(some_integer: i32) {
            // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.
    }

    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String

            (s, length)
        }
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
        other(&s);

        println!("{}", s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        fn other(string: &String) {
            //
        }
    }

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        // println!("{}, {}, and {}", r1, r2, r3);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    {
        let reference_to_nothing = dangle();

        fn dangle() -> String {
            let s = String::from("hello");

            s
        }
    }

    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error!

        println!("the first word is: {}", word);

        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }

    {
        let s = String::from("hello");

        let slice = &s[0..2];
        let slice = &s[..2];

        let s = String::from("hello");

        let len = s.len();

        let slice = &s[3..len];
        let slice = &s[3..];

        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
        let slice = &s;
    }
    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error!

        // println!("the first word is: {}", word);

        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }

    {
        let s = "Hello, world!";
    }

    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);

        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}
