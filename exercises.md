1. Write a generator of consecutive elements of the Fibonacci sequence (loops, recursion)
   https://en.wikipedia.org/wiki/Fibonacci_sequence

2. Write a game of tic-tac-toe
* The board is 3 x 3 fields
* Players take turns to occupy the vacant fields, placing their sign (circle or cross) on them
* The game ends when all fields are occupied or one player occupies the winning sequence (column, row or diagonal)
* The game interface should be based on the command line / terminal

3. Create a type that represents money (monetary amount)
* Money can come in different currencies
* Money can be exchanged/converted to another currency at the indicated exchange rate
* Money can be added and subtracted with each other

4. Implement the following system commands in Rust:
   * echo - prints the text given as an argument to the standard output
   * cat - prints the contents of the indicated files on the standard output, allows optional line numbering, line numbering can be disabled for blank lines
   * wc - prints the number of bytes, characters, words and lines for the indicated files
   * find - searches and prints the paths of files and/or directories whose names match the indicated patterns (use walkdir and regex libs, use iterators)
   * grep - finds and prints lines containing the indicated text/pattern from the indicated files/paths

5. Write an application to record receipts/expenditures for the household budget. The application should record the amount,
   type of operation and its description (given as command line arguments) and generate a report/table in terminal.
   Report should contain all the operations and a summary/final balance. The application should save the data entered by the user in a plain text file

6. Implement Linked list (different variations)

7. Creating a mini system for employee work time recording and analysis 
Write a CLI application that:
   * Reads data from the `work_log.csv` file, containing work time entries in the following format:
   ```
   employee_id, date, start_time, end_time
   E001,2025-07-29,09:00,17:30
   E002,2025-07-29,08:45,17:15
   ...
   ```
   * Calculates:
   - total time worked per employee
   - average workday length
   - days with overtime (e.g., > 8 hours).
   * Generates a report (`report.txt`) with the results
