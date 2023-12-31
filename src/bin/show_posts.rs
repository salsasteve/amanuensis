use self::models::*;
use amanuensis::*;
use diesel::prelude::*;

fn main() {
    use self::schema::messages::dsl::*;

    let connection = &mut establish_connection();
    let results = messages
        .limit(5)
        .select(Message::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for message in results {
        println!("{}", message.id);
        println!("----------\n");
        println!("{}", message.username);
    }
}
