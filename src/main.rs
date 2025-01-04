mod user;
use user::create_user;

fn main() {
    for _ in 0..2 {
        match create_user() {
            Ok(user) => {
                println!(
                    "\nUser Profile Created: Name: {}, Age: {}, Gender: {:?}, Favorite Color: {:?} with ID: {}\n",
                    user.name, user.age, user.gender, user.favourite_color, user.id,
                );
            }
            Err(error) => eprintln!("Failed to create the user, because, {}\n", error),
        };
    }
}
