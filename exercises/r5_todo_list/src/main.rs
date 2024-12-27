mod database;

use std::env;
use database::Database;
fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: todo[ add | rm | ls] [args]");
        return;
    }

    let command = &args[1];
    let mut db = Database::open(".todorc");

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!(" Usage: todo add [contentx]");
                return;
            }
            let contents = &args[2..].join(" ");
            let id = db.read_records().last().map(|r| r.id+1).unwrap_or(1);
            db.add_record(&database::Record{id, content: contents.to_string()});
        }
        "rm" => {
            if args.len() < 3 {
                println!(" Usage: todo rm [id]");
                return;
            }
            println!("Remove");
            let id = args[2].parse::<i32>().unwrap();
            db.remove_record(id);
        }
        "ls" => {
            println!("List");
            let records = db.read_records();
            if records.is_empty() {
                println!("No records. you can add one with `toto add [content]`");
                return;
            }
            for record in records {
                println!("{}:{}", record.id, record.content);
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
