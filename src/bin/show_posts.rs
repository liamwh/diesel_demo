use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true)) //NOTE: Set to false to see all posts
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
