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

#[derive(Debug)]
enum BookEnum {
    Title(String),
    Author(String),
    Rented(bool)
}



fn main() {

    // Array of books
    let mut all_books: Vec<Book> = Vec::new();
    let mut all_booksEnum: Vec<BookEnum> = Vec::new();

    // Option for the user
    let mut option = String::new();

    
    loop {
        // Present menu to user
        menu();
        println!("Insert your option: ");
        std::io::stdin().read_line(&mut option)
             .expect("Failed to read line");

        

        match str_to_u32(&mut option) {
            1 => {
                // Create a new book and get the data from the user
                let mut new_book = Book {title: String::from(""), author: String::from(""), rented: false}; 

                println!("Input title: ");
                let mut input_title = String::new();
                std::io::stdin().read_line(&mut input_title)
                    .expect("Failed to read line");
                
                new_book.title = input_title;
                
                let mut input_author = String::new();
                std::io::stdin().read_line(&mut input_author)
                    .expect("Failed to read line"); 

                new_book.author = input_author;   

                // Add book to the vector
                all_books.push(new_book);
            },
            2 => {
                println!("You chose 2");
                break;
            },
            3 => {
                // List all books
                for books in all_books.iter() {
                    match books {
                        _ => println!("Book: {:?}", books)
                    }
                }
                break;
            }
            255 => {
                println!("Parsing error");
                break;
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

}

// Function to get user's data for a new book
fn get_book_data(_title: String, _author: String) {
    println!("Input the book's title: ");
}

// Function to convert string to u32
fn str_to_u32(str_to_convert: &mut String) -> u32 {
    let str_to_convert: u32 = match str_to_convert.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Error to convert!");
            255
        }
    };
    str_to_convert
}




