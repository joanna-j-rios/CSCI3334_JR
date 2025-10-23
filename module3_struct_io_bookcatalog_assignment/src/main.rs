use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

//Save all books to a file
fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    // create file
    let mut file = File::create(filename).unwrap();

    for book in books {
        // write to file --> note: "each book should be on a seperate line with fields seperated by commas"
        writeln!(file,"{},{},{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader

    // create books vector
    let mut books = Vec::new();

    // open file
    let file = File::open(filename).unwrap();

    // user buffer for line by line reading
    let reader = BufReader::new(file);
    
    // iterate over lines in file to read --> note: recall one book per line
    for line_file in reader.lines(){

        // read line
        let line = line_file.unwrap();

        // split line into three parts (fields --> recall each is seperated by ',')
        let parts: Vec<&str> = line.split(',').collect();
       
        // add to vector of books
        // note: year is u16 so year string needs to be parsed in to u16 integer --> since this is how its written in to file & declared in struct
        // there are spaces and newline characters that need to be cleaned up --> we can use trim() to do that
        // ...format year
        let year: u16 = parts[2].trim().parse().unwrap();
        // ...format book
        let book = Book{
            title: parts[0].trim().to_string(),
            author: parts[1].trim().to_string(),
            year,
        };
        books.push(book); // adds book to vector
    }
    books // return books vector
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        // books I added for testing:
        Book { title: "Farenheit 451".to_string(), author: "Ray Bradbury".to_string(), year: 1953 },
        Book { title: "Brave New World".to_string(), author: "Aldous Huxley".to_string(), year: 1932 },
        Book { title: "The Handmaid's Tale".to_string(), author: "Margaret Atwood".to_string(), year: 1985 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}