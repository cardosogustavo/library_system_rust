/*
    Homework for the University, using arrays and CRUD operations
    Author: Gustavo Melhorança Cardoso 
*/

// Estruturas
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    id_number: i32,
    rented: bool
}

impl Book {
    fn new(id: i32) -> Book {
        let n = Book {title: String::from(""), author: String::from(""), id_number: id, rented: false};
        n
    }
}



fn main() {

    // Array of books
    let mut all_books: Vec<Book> = Vec::new();

    // Option for the user
    let mut option = String::new();

    // Counter for all the books in store
    let mut book_counter: i32 = 0;

    
    loop {
        // Present menu to user
        menu();
        println!("Insert your option: ");
        // Clear the input buffer
        option.clear();

        std::io::stdin().read_line(&mut option)
             .expect("Failed to read line");

        let option_int: u32 = str_to_u32(&mut option);
        

        match option_int {
            0 => break,
            1 => {
                book_counter += 1;
                // Create a new book and get the data from the user
                let new_book = get_book_data(book_counter);

                // Add book to the vector
                all_books.push(new_book);
            },
            2 => {
                println!("You chose 2");
                break;
            },
            3 => {
                // List all books
                for book in &all_books {
                    println!("{:?}", book);
                }
                
            },
            4 => {
                break;
            },
            5 => {
                let mut input_id = String::new();
                println!("Enter the id of the book: ");
                
                std::io::stdin().read_line(&mut input_id)
                    .expect("Could not read id");

                let input_id_int = str_to_usize(&mut input_id);

                if all_books.get(input_id_int) {

                }

            },
            255 => {
                println!("Parsing error");
                
            },
            _ => {
                println!("Anything else");
            },
        };

    }


}

// Menu with the options
fn menu(){

    println!("------ Library System ------\n");
    println!("1 - Add book to archive ");
    println!("2 - Delete book from archive ");
    println!("3 - List all books ");
    println!("4 - Update info from a book ");
    println!("5 - Search a book ");
    println!("0 - Exit ");

}

// Function to get user's data for a new book
fn get_book_data(id: i32) -> Book {
    let mut new = Book::new(id);
    println!("Input the book's title: ");
    std::io::stdin().read_line(&mut new.title)
        .expect("Could not read title");

    println!("Input the book's author: ");
    std::io::stdin().read_line(&mut new.author)
        .expect("Could not read author");

    new
}

// Function to convert string to u32
fn str_to_u32(str_to_convert: &mut String) -> u32 {
    let str_to_convert: u32 = match str_to_convert.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("str_to_u32 error");
            255
        }
    };
    str_to_convert
}


// Function to convert string to usize
fn str_to_usize(str_to_convert: &mut String) -> usize {
    let str_to_convert: usize = match str_to_convert.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("str_to_usize error");
            255
        }
    };
    str_to_convert
}



