1. Write a generator of consecutive elements of the Fibonacci sequence (loops, recursion)
   https://en.wikipedia.org/wiki/Fibonacci_sequence

2. Create a type that represents money (monetary amount)
* Money can come in different currencies
* Money can be exchanged/converted to another currency at the indicated exchange rate
* Money can be added and subtracted with each other

3. Write a game of tic-tac-toe
* The board is 3 x 3 fields
* Players take turns to occupy the vacant fields, placing their sign (circle or cross) on them
* The game ends when all fields are occupied or one player occupies the winning sequence (column, row or diagonal)
* The game interface should be based on the command line / terminal

4. Write an application to record receipts/expenditures for the household budget. The application should record the amount,
   type of operation and its description (given as command line arguments) and generate a report/table in terminal.
   Report should contain all the operations and a summary/final balance. The application should save the data entered by the user in a plain text file

5. Implement the following system commands in Rust:
   * echo - prints the text given as an argument to the standard output
   * cat - prints the contents of the indicated files on the standard output, allows optional line numbering, line numbering can be disabled for blank lines
   * wc - prints the number of bytes, characters, words and lines for the indicated files
   * find - searches and prints the paths of files and/or directories whose names match the indicated patterns (use walkdir and regex libs, use iterators)
   * grep - finds and prints lines containing the indicated text/pattern from the indicated files/paths

6. Write a simple database that stores data in binary form using random access. An example of reading data
   from a specific position in the file is given below. A write should be implemented on a similar basis.
   The database should allow the following operations: read a record by id, update a record, delete a record, add a record.
   To optimize the performance of the database, introduce record position indexing and a simple cache using HashMap.
   Think about the optimal way to delete records and reuse this area of the file. Records should have a fixed length of individual
   fields, and their definition should be in the header section of the file.
   Extra task: Expose the functionality of the created database via rest api, using one of the frameworks you have learned (actix, rocket)

Example record:
```
id:i64
first_name: string
last_name: string
is_active: bool
age: u8
```

Reading data from a specific file position
```
let start = 10;
let count = 10;

let mut f = File::open("/etc/passwd")?;
f.seek(SeekFrom::Start(start))?;
let mut buf = vec![0; count];
f.read_exact(&mut buf)?;
```

Example of converting bytes to integer type and vice versa
```
let original_u32: u32 = 1048572;
println!("{}", original_u32);
    
let u32_as_bytes: [u8; 4] = original_u32.to_be_bytes();
println!("{:?}", u32_as_bytes);
    
let back_to_u32: u32 = u32::from_be_bytes(u32_as_bytes);
println!("{}", back_to_u32);
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0bc90209eb1df96ad76d23490d34e8be
to_be_bytes
https://doc.rust-lang.org/std/primitive.u32.html#method.to_be_bytes
from_be_bytes
https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes

Converting bytes to String
```
let bytes = vec![0x41, 0x42, 0x43];
let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
println!("{}", s);
```



Books:
 The Rust Programming Language, 2nd Edition 
 Effective Rust
 Idiomatic Rust
 Rust Atomics and Locks
 Async Rust
 Write Powerful Rust Macros

Memory management: https://www.youtube.com/watch?v=7_o-YRxf_cc

Rust crate for cooking up terminal user interfaces (TUIs):
https://github.com/ratatui/ratatui
Rust embedded:
https://docs.rust-embedded.org/book/start/hardware.html
Testing and benchmarking:
https://github.com/la10736/rstest
https://github.com/becheran/ntest
https://docs.rs/criterion/latest/criterion/
https://github.com/nvzqz/divan