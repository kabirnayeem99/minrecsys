use rand::Rng;
use rusqlite::{params, Connection, ToSql};

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub age: u32,
    pub name: String,
    pub favourite_color: Color,
    pub gender: Gender,
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl ToSql for Gender {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let gender_str = match *self {
            Gender::Male => "male",
            Gender::Female => "female",
        };
        Ok(gender_str.to_sql()?)
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
}

impl ToSql for Color {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let color_str = match *self {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
            Color::Yellow => "Yellow",
            Color::Black => "Black",
            Color::White => "White",
        };
        Ok(color_str.to_sql()?)
    }
}

fn generate_random_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(48..845768)
}

pub fn create_user() -> Result<User, String> {
    use std::io::{self, Write};

    let mut name = String::new();
    let mut age: String = String::new();
    let mut favourite_color: String = String::new();
    let mut gender: String = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    print!("Eter your age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap();

    print!("Enter your gender: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut gender).unwrap();
    let gender_input = gender.trim().to_lowercase();
    let gender: Gender = match gender_input.as_str() {
        "male" => Gender::Male,
        "female" => Gender::Female,
        "0" => Gender::Male,
        "1" => Gender::Female,
        _ => Gender::Male,
    };

    print!("Enter your favourite color: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut favourite_color).unwrap();
    let favourite_color_input = favourite_color.trim().to_lowercase();
    let favourite_color: Color = match favourite_color_input.as_str() {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "black" => Color::Black,
        "white" => Color::White,
        _ => Color::Black,
    };

    let user = User {
        id: generate_random_id(),
        age,
        name,
        favourite_color,
        gender,
    };

    return match save_user(&user) {
        Ok(user) => Ok(user),
        Err(_) => Err("Failed to save the user".to_string()),
    };
}

pub fn save_user(user: &User) -> Result<User, ()> {
    let conn = Connection::open("users.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
          id INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          age INTEGER,
          favourite_color TEXT,
          gender TEXT
      )",
        params![],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO user (id, name, age, favourite_color, gender) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &user.id,
            &user.name,
            &user.age,
            &user.favourite_color,
            &user.gender
        ],
    )
    .unwrap();

    Ok(user.clone())
}
