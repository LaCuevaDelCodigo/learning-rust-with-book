fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        println!("{}", user1.email)
    }

    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

        let user3 = User {
            active: true,
            username: String::from("someusername123"),
            sign_in_count: 1,
            ..user1
        };

        let user4: User = User {
            sign_in_count: 2,
            ..user2
        };
    }

    {
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }
        }

        fn build_user2(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
    }

    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        struct AlwaysEqual;

        let subject = AlwaysEqual;
    }
    {
        // struct User {
        //     active: bool,
        //     username: &str,
        //     email: &str,
        //     sign_in_count: u64,
        // }

        // let user1 = User {
        //     active: true,
        //     username: "someusername123",
        //     email: "someone@example.com",
        //     sign_in_count: 1,
        // };
    }
}
