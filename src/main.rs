/*
    Homework for the University, using arrays and CRUD operations
    Author: Gustavo Melhorança Cardoso 
*/

// Estruturas
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    rented: bool
}

impl Book {
    fn new() -> Book {
        let n = Book {title: String::from(""), author: String::from(""), rented: false};
        n
    }
}



fn main() {

    // Array of books
    let mut all_books: Vec<Book> = Vec::new();

    // Option for the user
    let mut option = String::new();

    // Counter for all the books in store
    let mut book_counter: u8 = 0;

    
    loop {
        // Present menu to user
        menu();
        println!("Insert your option: ");
        // Clear the input buffer
        option.clear();

        std::io::stdin().read_line(&mut option)
             .expect("Failed to read line");

        // if let 3 = str_to_u32(&mut option) {
        //     println!("Three");
        // } else {
        //     break;
        // }
        //println!("Your option is: {}", option);
        let option_int: u32 = str_to_u32(&mut option);
        

        match option_int {
            0 => break,
            1 => {
                // Create a new book and get the data from the user
                let new_book = get_book_data();

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
                
            }
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
fn get_book_data() -> Book {
    let mut new = Book::new();
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


// Function to



