use row::Row;

mod table;
mod row;
mod storage;

use table::Table;

fn main() {
    let mut table = Table::new();
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        handler(&input.trim(), &mut table);
    }
}

fn handler(command: &str, table: &mut Table) {
    if command.starts_with("INSERT") {

        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() != 4 {
            println!("Syntax error. Expected INSERT <id> <username> <email>");
            return;
        }
        let id = parts[1].parse::<u32>().expect("Expected u32 for id");
        let username = parts[2].to_string();
        let email = parts[3].to_string();
        let row = Row::new(id, username, email);
        table.insert(row);
        println!("Inserted row");
    } else if command == "SELECT *" {
        for row in table.get_all_rows() {
            println!("id: {}, username: {}, email: {}", row.id, row.username, row.email);
        }
    }

    // TODO: Add a SELECT command

    else {
        println!("Unrecognized command {}", command);
    }
}