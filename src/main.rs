// 


use std::io::{self, Read};

fn main() {

    let mut vector_of_notes: Vec<String> = Vec::new();

    loop {

        println!("\n-> Do you want to add a note or read? (add/read)\n");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        println!("");

        if command.trim() == "add" {
            add(&mut vector_of_notes);
        } else if command.trim() == "read" {
            read(&vector_of_notes);
        } else {
            println!("Invalid command");
        }

    }
    
}

fn add(vector_of_notes: &mut Vec<String>) {
    println!("-> Enter your note. Press Ctrl+D when done.\n");
    let mut note = String::new();

    io::stdin()
        .read_to_string(&mut note)
        .expect("Failed to read line");

    println!("");

    vector_of_notes.push(note);

}

fn read(vector_of_notes: &Vec<String>) {
    println!("-> Here are your notes.\n");
    for note in vector_of_notes {
        println!("{}\n", note);
    }
}
