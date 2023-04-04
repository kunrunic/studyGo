struct Email {
    user: String,
    host: String,
}

impl std::fmt::Debug for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Email")
            .field("user", &self.user)
            .field("host", &self.host)
            .finish()
    }
}

struct User {
    name: String,
    number: i32,
    email: Email,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("Name", &self.name)
            .field("Number", &self.number)
            .field("Email", &self.email)
            .finish()
    }
}

struct SubsInfo {
    user: User,
    pin: String,
}

impl std::fmt::Debug for SubsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // print fmt {:#?}
            f.debug_struct("SubsInfo")
                .field("User", &format!("{:?}", self.user))
                .field("Pin", &self.pin)
                .finish()
        } else {
            // print fmt {:?}
            writeln!(f, "{}", "Test1")?;
            writeln!(f, "{}", "TEST2")?;
            Ok(())
        }
    }
}

fn main() {
    let user = SubsInfo {
        user: User {
            name: "new user".into(),
            number: 3335555,
            email: Email {
                user: "ssss".into(),
                host: "test.com".into(),
            },
        },
        pin: "1234".into(),
    };
    println!("[case fmt 1] \n{:#?}\n", user);
    println!("[case fmt 2] \n{:?}", user);
}
