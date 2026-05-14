use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

// =================================================================================================
// Section 1: Text Processing Methods
// =================================================================================================

/*
## Text Processing Methods

Building on module 002 (which covered `len`, `chars`, `contains`, `starts_with`, `trim`, `split`,
`push`, `push_str`, `to_uppercase`, `to_lowercase`) and module 005 (escape sequences, raw strings,
`format!`), this section covers additional string methods useful for text processing tasks.

- **`.replace(pattern, replacement)`** returns a new `String` with all occurrences of `pattern`
  replaced by `replacement`. **`.replacen(pattern, replacement, count)`** replaces only the first
  `count` occurrences.
- **`.lines()`** splits a string on line boundaries (`\n` and `\r\n`) and returns an iterator of
  `&str` slices. The line terminators are **not** included in the yielded slices. This is the
  idiomatic way to iterate over lines of text loaded into a `String` from a file.
- **`.split_whitespace()`** splits on any Unicode whitespace and returns an iterator of `&str`
  slices. Unlike `.split(' ')` which only splits on ASCII space, `.split_whitespace()` handles tabs,
  newlines, and other whitespace characters, and it skips consecutive whitespace (no empty strings
  in the output).
- **`.trim_start()` / `.trim_end()`** remove leading / trailing whitespace respectively. Module 002
  covered `.trim()` which removes both; these give directional control.
- **`.repeat(n)`** returns a new `String` consisting of the original string repeated `n` times.
  Useful for building separators, padding, or visual elements.
- **`.join(separator)`** is a method on **slices of strings** (`[&str]`, `[String]`), not on
  `String` itself. It concatenates all elements with `separator` between them: `["a", "b",
  "c"].join(", ")` produces `"a, b, c"`.
*/

fn text_processing_methods() {
    // --- replace and replacen ---
    let text = "the cat sat on the mat";
    let replaced = text.replace("the", "a");
    println!("replace all: \"{replaced}\""); // "a cat sat on a mat"

    // replacen limits the number of replacements
    let replaced_first = text.replacen("the", "a", 1);
    println!("replacen(1): \"{replaced_first}\""); // "a cat sat on the mat"
    println!("replace all: \"{replaced}\""); // "a cat sat on a mat"

    // --- lines() ---
    // lines() handles both \n and \r\n, strips terminators
    let multiline = "first line\nsecond line\r\nthird line";
    let line_count = multiline.lines().count();
    println!("line count: {line_count}"); // 3

    for (i, line) in multiline.lines().enumerate() {
        println!("  line {}: \"{line}\"", i + 1);
    }

    // --- split_whitespace() ---
    // Splits on any whitespace, skips consecutive whitespace
    let sentence = "  hello   world\t  rust  ";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("words: {words:?}"); // ["hello", "world", "rust"]
    println!("word count: {}", words.len()); // 3

    // --- trim_start() / trim_end() ---
    let padded = "   hello   ";
    println!("trim_start: \"{}\"", padded.trim_start()); // "hello   "
    println!("trim_end: \"{}\"", padded.trim_end()); // "   hello"

    // --- repeat() ---
    let separator = "-".repeat(40);
    println!("{separator}");
    println!("  centered text");
    println!("{separator}");

    // --- join() on slices ---
    // join is called on a slice of strings, not on a single string
    let parts = ["hello", "world", "rust"];
    let joined = parts.join(" ");
    println!("joined: \"{joined}\""); // "hello world rust"

    let csv_line = ["Alice", "30", "Engineer"].join(";");
    println!("csv: \"{csv_line}\""); // "Alice;30;Engineer"
}

// =================================================================================================
// Section 2: Command-Line Arguments and Environment Variables
// =================================================================================================

/*
## Command-Line Arguments and Environment Variables

- **`std::env::args()`** returns an iterator of `String` values representing the program's
  command-line arguments. The first element (`args[0]`) is the program name or path. The iterator is
  lazy — you can use `.skip()`, `.nth()`, `.collect()`, etc.
- Common pattern: `env::args().skip(1).collect::<Vec<String>>()` to get all arguments except the
  program name.
- **`std::env::args_os()`** is like `args()` but returns `OsString` instead of `String`. It handles
  non-UTF-8 arguments that might occur on some platforms. `args()` panics on non-UTF-8 arguments.
- **`std::env::var("NAME")`** reads a single environment variable and returns `Result<String,
  VarError>`. Use `unwrap_or_else` for a fallback value.
- **`std::env::vars()`** iterates over all environment variables as `(String, String)` pairs.
- **`std::env::current_dir()`** returns `Result<PathBuf>` with the current working directory.
- **`std::env::current_exe()`** returns `Result<PathBuf>` with the path to the running executable.
- **`std::env::temp_dir()`** returns the platform's temporary directory as a `PathBuf`.
- For real CLI tools, prefer the `clap` crate (section 3) over manual argument parsing. Manual
  `env::args()` is fine for simple scripts or learning purposes.

### Exit Codes and Top-Level Error Handling

- **Non-zero exit status convention**. Command-line tools signal failure to the calling process
  (shell, script, CI runner) by exiting with a **non-zero** code. The convention: `0` for success,
  `1` (or higher) for any error. The shell uses this code for constructs like `cmd && next` (run
  `next` only if `cmd` succeeds) and `set -e` (abort script on first failure).
- **`std::process::exit(code)`** stops the program immediately and returns `code` as the process
  exit status. Unlike `panic!`, it does **not** produce a backtrace, the `thread 'main' panicked at
  ...` header, or unwind the stack — making it the right tool for user-facing errors (the ones where
  you want a clean message instead of a developer stack trace). Cross-reference: module 004 mentions
  `process::exit` as a diverging function; here we see its role in CLI tools.
- **Idiomatic top-level error pipeline**. A well-structured binary puts the logic in a `run()`
  function returning `Result<(), Box<dyn Error>>` (module 009 §4b) and keeps `main` responsible only
  for calling `run` and reporting failures. Two common shapes:
  - **Parse/build errors**: `let config = Config::build(&args) .unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}"); std::process::exit(1); });` — when a failed parse
    returns a value you still need, use `unwrap_or_else` with a closure that prints and exits.
  - **Runtime errors**: `if let Err(e) = run(config) { eprintln!( "Application error: {e}");
    std::process::exit(1); }` — when `run` returns `Result<(), E>` (unit success), you don't need
    the unwrapped value, so `if let Err` is the right shape.
- Both branches print to **stderr** via `eprintln!` (module 005) and exit non-zero. The user can
  then redirect stdout to a file (`./tool > results.txt`) and still see errors on the terminal.
- `env::var("NAME").is_ok()` is the idiomatic **presence check** for a feature-flag-style
  environment variable: you do not care about the value, only whether the variable is set. Example:
  `let verbose = env::var("VERBOSE").is_ok();` — setting `VERBOSE=1 ./tool` or `VERBOSE= ./tool` or
  `VERBOSE=anything ./tool` all enable the flag; unsetting the variable disables it. Contrast with
  `unwrap_or_else(|_| default)` when you actually need the value.
*/

fn command_line_and_environment() {
    // --- Collecting command-line arguments ---
    // skip(1) removes the program name, leaving only user arguments
    let args: Vec<String> = env::args().skip(1).collect();
    println!("user arguments: {args:?}");

    // Partitioning arguments into flags and positional args
    // (this pattern is used in the cat exercise)
    let (flags, positional): (Vec<_>, Vec<_>) = args.iter().partition(|arg| arg.starts_with('-'));
    println!("flags: {flags:?}, positional: {positional:?}");

    // --- Reading environment variables ---
    // env::var returns Result<String, VarError>
    match env::var("HOME") {
        Ok(home) => println!("HOME: {home}"),
        Err(_) => println!("HOME not set"),
    }

    // unwrap_or_else provides a fallback value
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    println!("SHELL: {shell}");

    // --- Iterating environment variables with a filter ---
    // Show only variables whose name contains "RUST" or "CARGO"
    println!("Rust-related env vars:");
    for (key, value) in env::vars() {
        if key.contains("RUST") || key.contains("CARGO") {
            // Truncate long values for display
            let display_value = if value.len() > 60 {
                let truncated: String = value.chars().take(60).collect();
                format!("{truncated}...")
            } else {
                value
            };
            println!("  {key} = {display_value}");
        }
    }

    // --- Current directory and executable path ---
    if let Ok(cwd) = env::current_dir() {
        println!("current directory: {}", cwd.display());
    }
    if let Ok(exe) = env::current_exe() {
        println!("executable path: {}", exe.display());
    }

    // --- Temporary directory ---
    let tmp = env::temp_dir();
    println!("temp directory: {}", tmp.display());
}

// --- stdin (interactive, not called from run) ---
// Not called from run() because it blocks waiting for user input.
// Run manually with: cargo run, then type input when prompted.
#[allow(dead_code)]
fn stdin_demo() {
    use std::io::{self, Write};

    // Basic read_line — reads one line including the trailing \n
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // flush so the prompt appears before input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let name = input.trim(); // remove trailing newline
    println!("Hello, {name}!");

    // Parsing input to a number
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut num_input = String::new();
    io::stdin()
        .read_line(&mut num_input)
        .expect("failed to read");
    match num_input.trim().parse::<i32>() {
        Ok(n) => println!("you entered: {n}"),
        Err(e) => println!("not a valid number: {e}"),
    }
}

// =================================================================================================
// Section 3: clap Crate — Derive-Based Argument Parsing
// =================================================================================================

/*
## clap Crate — Derive-Based Argument Parsing

- **`clap`** is the standard Rust crate for parsing command-line arguments. It provides automatic
  `--help`, `--version`, type validation, and descriptive error messages.
- Two API styles exist: **builder** (procedural, method chains) and **derive** (declarative with
  `#[derive(Parser)]`). The derive API is more concise and idiomatic for most use cases.
- **Cargo.toml requirement:** `clap = { version = "4", features = ["derive"] }`
- Core derive attributes:
  - `#[derive(Parser)]` on a struct turns it into a CLI parser.
  - `#[command(name, about, version)]` sets metadata shown in `--help` and `--version` output.
  - `#[arg(short, long)]` makes a field a named option/flag (`-v` / `--verbose`).
  - `#[arg(short, long, default_value_t = value)]` provides a default when the flag is not given.
  - Fields without `#[arg(short/long)]` become **positional** arguments.
  - `Vec<T>` fields accept **multiple** values.
  - `Option<T>` fields are **optional** (not required).
  - `bool` fields with `#[arg(short, long)]` become flags (present = true, absent = false).
- **`#[derive(clap::ValueEnum)]`** on an enum allows it to be used as an argument value. clap
  handles parsing and error messages for invalid values.
- **`Cli::parse()`** reads from `std::env::args()` — use this in real `main()` functions.
- **`Cli::parse_from(iter)`** parses from an explicit iterator instead — useful for testing or when
  you cannot intercept real args.
- **`Cli::try_parse_from(iter)`** returns `Result` instead of exiting on error — useful for
  demonstrating error handling.
*/

// A simple CLI definition using clap's derive API
#[derive(Parser, Debug)]
#[command(name = "search", about = "Search for patterns in files")]
struct SearchCli {
    /// The pattern to search for
    pattern: String,

    /// Files or directories to search in
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Maximum number of results to show
    #[arg(short, long, default_value_t = 100)]
    max_results: usize,
}

// An enum that can be used as a command-line argument value
#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

// A CLI that uses an enum argument
#[derive(Parser, Debug)]
#[command(name = "report", about = "Generate a report")]
struct ReportCli {
    /// Input file path
    input: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

fn clap_argument_parsing() {
    // parse_from lets us demonstrate clap without intercepting real args
    let cli = SearchCli::parse_from([
        "search",          // program name (argv[0])
        "TODO",            // positional: pattern
        "src/",            // positional: first path
        "tests/",          // positional: second path
        "--verbose",       // flag
        "--max-results=5", // named option with value
    ]);
    println!("parsed SearchCli: {cli:?}");
    println!(
        "  pattern={}, paths={:?}, verbose={}, max={}",
        cli.pattern, cli.paths, cli.verbose, cli.max_results
    );

    // Enum-based argument parsing
    let report = ReportCli::parse_from(["report", "data.csv", "--format=json"]);
    println!("parsed ReportCli: {report:?}");

    // try_parse_from returns Result instead of exiting on error
    let result = SearchCli::try_parse_from(["search"]); // missing required args
    match result {
        Ok(cli) => println!("parsed: {cli:?}"),
        Err(e) => println!("parse error (expected): {e}"),
    }

    // --- Subcommands ---
    // Use #[derive(clap::Subcommand)] on an enum, and
    // #[command(subcommand)] on the parent struct field.
    #[derive(clap::Parser, Debug)]
    #[command(name = "git-lite")]
    struct GitCli {
        #[command(subcommand)]
        command: GitCommand,
    }

    #[derive(clap::Subcommand, Debug)]
    enum GitCommand {
        /// Clone a repository
        Clone {
            #[arg(help = "Repository URL")]
            url: String,
        },
        /// Push to remote
        Push {
            #[arg(short, long, default_value = "origin")]
            remote: String,
        },
    }

    let cli = GitCli::parse_from(["git-lite", "clone", "https://example.com/repo"]);
    println!("subcommand: {cli:?}");
    let cli = GitCli::parse_from(["git-lite", "push", "--remote", "upstream"]);
    println!("subcommand: {cli:?}");
}

// =================================================================================================
// Section 4: File Reading
// =================================================================================================

/*
## File Reading

- **`std::fs::read_to_string(path)`** reads an entire file into a `String` in one call. It is the
  simplest approach but loads the whole file into memory. Returns `Result<String, io::Error>`.
- **`File::open(path)`** opens a file for reading and returns `Result<File, io::Error>`. The `File`
  type implements `Read` but does **not** buffer reads internally — each `.read()` call goes
  directly to the OS.
- **`BufReader`** wraps any `Read` type (including `File`) with an internal buffer for efficient
  I/O. It provides two main reading patterns:
  - **`.lines()`** returns an iterator of `Result<String, io::Error>`. Each `String` does **not**
    include the line terminator. Allocates a new `String` per line.
  - **`.read_line(&mut buf)`** reads one line into an existing `String` buffer (including the line
    terminator). Returns `Result<usize>` where the `usize` is the number of **bytes** read. Returns
    `Ok(0)` at EOF. The caller must call `buf.clear()` between iterations to avoid accumulating
    lines. More efficient than `.lines()` because it reuses the buffer.
- **When to use which:**
  - `read_to_string`: small files, quick scripts, when you need the whole content at once.
  - `BufReader::lines()`: line-by-line processing, when you want clean strings without terminators.
  - `BufReader::read_line()`: when you need byte counts per line (like the `wc` command), or when
    performance matters and you want to reuse a single buffer.

### Low-Level Read Trait Methods

`File` implements the **`std::io::Read`** trait. The convenience helpers above
(`fs::read_to_string`, `BufReader::lines`) are layered on top of a small set of primitive methods.
Knowing them is useful when writing your own generic `Read` adapters or parsing binary protocols.

- **`reader.read(&mut buf)`** — the raw primitive. Reads **up to** `buf.len()` bytes into `buf` and
  returns `io::Result<usize>` with the number of bytes actually read. Returns `Ok(0)` at EOF. A
  single call may legitimately return fewer bytes than requested even when not at EOF (the "short
  read" case) — callers must be prepared to loop.
- **`reader.read_to_end(&mut vec)`** — appends the full remaining content to a `Vec<u8>`. Returns
  `io::Result<usize>` with the total bytes appended.
- **`reader.read_to_string(&mut s)`** — appends the full remaining content to a `String`. Fails with
  `ErrorKind::InvalidData` if the input is not valid UTF-8.
- **`reader.read_exact(&mut buf)`** — fills `buf` *completely* or returns
  `Err(ErrorKind::UnexpectedEof)`. Use this to read fixed-size records.
- **`reader.bytes()`** — iterator of `io::Result<u8>`. Convenient for tiny inputs, but yields one
  `Result` per byte, so wrap the reader in a `BufReader` before calling `.bytes()` on a `File`.
- **`reader.take(n)`** — returns an adapter that reads **at most `n` bytes** before reporting EOF.
  Useful for protocols that embed a length prefix: read the length, then `(&mut
  reader).take(len).read_to_end(&mut buf)`.
- **`reader.chain(other)`** — returns an adapter that reads from `reader` first, then switches to
  `other` at its EOF. Handy for splicing a header in front of an existing stream.

### Interrupted Reads and `io::Error::kind()`

- A real OS can legitimately interrupt a blocking read — typically when a signal is delivered — and
  return an error with `ErrorKind::Interrupted`. This is **not** a failure; the correct response is
  to retry the operation immediately. Idiomatic Rust loops use `io::Error::kind()` to classify
  errors and continue on `Interrupted`, propagating everything else.
- This is the exact pattern `std::io::copy` uses internally. It is also worth replicating in any
  custom loop that calls `.read()` directly.
*/

fn file_reading() -> Result<(), Box<dyn std::error::Error>> {
    // --- read_to_string: simplest approach ---
    // Read the entire file into a String, then use .lines() to iterate
    let content = fs::read_to_string("Cargo.toml")?;
    println!("first 3 lines of Cargo.toml (via read_to_string):");
    for line in content.lines().take(3) {
        println!("  {line}");
    }

    // --- BufReader with .lines(): line-by-line with line numbers ---
    // This pattern is used in the cat exercise for line numbering
    let file = File::open("Cargo.toml")?;
    let reader = BufReader::new(file);
    println!("lines 1-5 of Cargo.toml (via BufReader::lines):");
    for (index, line_result) in reader.lines().enumerate().take(5) {
        let line = line_result?;
        println!("  {:>4}: {line}", index + 1);
    }

    // --- BufReader with .read_line(): byte-aware reading ---
    // This pattern is used in the wc exercise for counting
    // bytes, chars, words, and lines
    let file = File::open("Cargo.toml")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut total_bytes: usize = 0;
    let mut total_chars: usize = 0;
    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;

    loop {
        buf.clear(); // reuse the buffer — avoids per-line allocation
        let bytes_read = reader.read_line(&mut buf)?;
        if bytes_read == 0 {
            break; // EOF
        }
        total_bytes += bytes_read;
        total_chars += buf.chars().count();
        total_words += buf.split_whitespace().count();
        total_lines += 1;
    }
    println!(
        "Cargo.toml stats: {total_lines} lines, {total_words} words, \
         {total_chars} chars, {total_bytes} bytes"
    );

    // --- Handling missing files gracefully ---
    match fs::read_to_string("nonexistent.txt") {
        Ok(content) => println!("content: {content}"),
        Err(e) => println!("file not found (expected): {e}"),
    }

    // --- Low-level Read trait methods ---
    // `read_to_end` — append whole remaining content to a Vec<u8>.
    let mut file = File::open("Cargo.toml")?;
    let mut bytes = Vec::new();
    let n = file.read_to_end(&mut bytes)?;
    println!("read_to_end: {n} bytes into Vec<u8>");

    // `read_to_string` — same, but into a String (fails on invalid UTF-8).
    let mut file = File::open("Cargo.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("read_to_string: {} bytes", contents.len());

    // `read_exact` — fill the buffer completely or fail.
    // Here we read the first 12 bytes of Cargo.toml (the "[workspace]"
    // header plus its newline; padding is fine for the demo).
    let mut file = File::open("Cargo.toml")?;
    let mut header = [0u8; 12];
    file.read_exact(&mut header)?;
    println!(
        "read_exact first 12 bytes: {:?}",
        std::str::from_utf8(&header).unwrap_or("<non-utf8>")
    );

    // `read` — the raw primitive, returns however many bytes it got.
    // A single call may return fewer bytes than requested even when
    // not at EOF, so callers must loop until Ok(0) = EOF.
    let mut file = File::open("Cargo.toml")?;
    let mut buf = [0u8; 64];
    let mut total = 0usize;
    loop {
        match file.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => total += n,
            // Correct handling of signal interruption: retry, don't fail.
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(Box::new(e)),
        }
    }
    println!("manual read loop: {total} bytes total");

    // `.bytes()` — iterator over `io::Result<u8>`. Wrap in BufReader
    // when calling on a File so each byte doesn't syscall.
    let count = BufReader::new(File::open("Cargo.toml")?)
        .bytes()
        .filter_map(Result::ok)
        .filter(|&b| b == b'\n')
        .count();
    println!("newline count via .bytes(): {count}");

    // `.take(n)` — adapter that stops after n bytes, regardless of EOF.
    let mut file = File::open("Cargo.toml")?;
    let mut head = String::new();
    (&mut file).take(32).read_to_string(&mut head)?;
    println!("first 32 bytes via .take(): {head:?}");

    // `io::copy` — generic pipe: copies from any Read to any Write
    // until EOF. Returns the number of bytes copied. This is the
    // zero-ceremony way to forward one stream into another.
    let mut src = File::open("Cargo.toml")?;
    let tmp = env::temp_dir().join("rust_mod010_copy.toml");
    let mut dst = File::create(&tmp)?;
    let copied = io::copy(&mut src, &mut dst)?;
    println!("io::copy moved {copied} bytes -> {}", tmp.display());
    fs::remove_file(&tmp)?;

    Ok(())
}

// =================================================================================================
// Section 5: File Writing
// =================================================================================================

/*
## File Writing

- **`File::create(path)`** creates a new file or **truncates** an existing one to zero length.
  Returns `Result<File, io::Error>`. The file is opened for writing only.
- **`OpenOptions`** provides fine-grained control over how a file is opened. Key options (chained):
  - `.write(true)` — enable writing.
  - `.create(true)` — create the file if it does not exist.
  - `.append(true)` — write at the end instead of overwriting.
  - `.truncate(true)` — truncate to zero length on open.
  - `.read(true)` — also enable reading.
  - Finalize with `.open("path")` which returns `Result<File>`.
- **`write!` / `writeln!`** macros format text directly into any type implementing the `Write` trait
  (files, buffers, network streams). Same formatting syntax as `print!` / `println!` but the first
  argument is the writer. They return `Result`.
- **`BufWriter`** wraps a `File` (or any `Write`) with an internal buffer. Writes are batched and
  flushed to the OS in larger chunks, which is more efficient when writing many small pieces.
  `BufWriter` flushes automatically when dropped, but an explicit `.flush()` call is recommended
  before checking for errors.
- **`.write_all(bytes)`** writes raw `&[u8]` bytes. Useful when you already have byte data (e.g.,
  from `.as_bytes()`).
- **`fs::write(path, contents)`** creates or truncates a file and writes the entire contents in one
  call. It is the write-side complement to `fs::read_to_string`. Returns `Result<()>`.
- Use `File::create` when you want to overwrite the whole file. Use `OpenOptions` with
  `.append(true)` when you want to add content to an existing file (e.g., appending log entries or
  budget operations).
*/

fn file_writing() -> Result<(), Box<dyn std::error::Error>> {
    // Use the system's temp directory to avoid polluting the project
    let temp_dir = env::temp_dir();
    let temp_file = temp_dir.join("rust_mod010_example.txt");

    // --- fs::write — one-liner for simple file writing ---
    // Complement to fs::read_to_string: creates/truncates and writes in one call.
    let simple_file = temp_dir.join("rust_mod010_simple.txt");
    fs::write(&simple_file, "hello from fs::write\n")?;
    println!(
        "fs::write content: {}",
        fs::read_to_string(&simple_file)?.trim()
    );
    fs::remove_file(&simple_file)?;

    // --- File::create + writeln! ---
    // Creates or truncates the file, then writes formatted lines
    {
        let mut file = File::create(&temp_file)?;
        writeln!(file, "Line 1: Hello from Rust")?;
        writeln!(file, "Line 2: Written with writeln!")?;
        writeln!(file, "Line 3: Value = {}", 42)?;
    } // file is closed when dropped

    // Verify by reading back
    let content = fs::read_to_string(&temp_file)?;
    println!("after File::create + writeln!:");
    for line in content.lines() {
        println!("  {line}");
    }

    // --- OpenOptions with append mode ---
    // Useful for log files or any scenario where new entries are
    // added without disturbing existing content.
    {
        let mut file = OpenOptions::new()
            .append(true) // write at the end
            .create(true) // create if it doesn't exist
            .open(&temp_file)?;
        writeln!(file, "Line 4: Appended later")?;
    }

    // Verify the append worked
    let content = fs::read_to_string(&temp_file)?;
    println!("after appending:");
    for line in content.lines() {
        println!("  {line}");
    }

    // --- BufWriter for efficient batched writes ---
    {
        let file = File::create(&temp_file)?;
        let mut writer = BufWriter::new(file);
        for i in 1..=5 {
            writeln!(writer, "Buffered line {i}")?;
        }
        writer.flush()?; // explicit flush before checking for errors
    }

    let content = fs::read_to_string(&temp_file)?;
    println!("after BufWriter:");
    for line in content.lines() {
        println!("  {line}");
    }

    // --- write_all for raw bytes ---
    {
        let mut file = File::create(&temp_file)?;
        file.write_all(b"raw bytes written with write_all\n")?;
        file.write_all("text converted to bytes".as_bytes())?;
    }

    let content = fs::read_to_string(&temp_file)?;
    println!("after write_all: {content:?}");

    // Clean up the temporary file
    fs::remove_file(&temp_file)?;
    println!("temp file cleaned up");

    Ok(())
}

// =================================================================================================
// Section 6: Paths and Directory Operations
// =================================================================================================

/*
## Paths and Directory Operations

- **`Path`** (borrowed, like `&str`) and **`PathBuf`** (owned, like `String`) are Rust's types for
  filesystem paths. `PathBuf` implements `Deref<Target = Path>`, so it can be used wherever `&Path`
  is expected.
- **Path inspection methods:**
  - `.exists()` — returns `bool`, checks if the path points to an existing file or directory.
  - `.is_file()` / `.is_dir()` / `.is_symlink()` — type checks.
  - `.parent()` — returns `Option<&Path>`, the containing directory.
  - `.file_name()` — returns `Option<&OsStr>`, the final component.
  - `.file_stem()` — filename without extension.
  - `.extension()` — returns `Option<&OsStr>`.
  - `.display()` — returns a displayable wrapper for printing paths that might contain non-UTF-8
    characters.
- **Path building:**
  - `PathBuf::from("some/path")` creates an owned path.
  - `.push("segment")` appends a path component (mutating).
  - `.join("segment")` returns a new `PathBuf` (non-mutating).
  - `.set_extension("txt")` changes the extension.
  - An iterator of path segments can be collected into a `PathBuf`.
- **`std::fs` operations:**
  - `fs::metadata(path)` — file size, timestamps, permissions. Returns `Result<Metadata>`.
  - `fs::create_dir(path)` — creates one directory (parent must exist). `fs::create_dir_all(path)`
    creates parent directories recursively.
  - `fs::read_dir(path)` — returns an iterator of `Result<DirEntry>` for the **immediate** children
    of a directory (non-recursive).
  - `fs::remove_file(path)` / `fs::remove_dir(path)` — delete a file or an **empty** directory.
    `fs::remove_dir_all(path)` deletes a directory and all its contents recursively.
  - `fs::rename(from, to)` — move or rename a file/directory.
  - `fs::copy(from, to)` — copy file contents, returns the number of bytes copied.
- **Cross-platform:** always use `Path::join()` or `PathBuf::push()` to build paths. Never hardcode
  `/` or `\` separators — Rust's path types handle platform differences automatically.

### Extended Path Inspection

Beyond the basic methods above, `Path` offers a few iterators and conversions that are worth
knowing:

- **`.components()`** — iterator of `Component` values, each variant classifying one segment:
  `Prefix` (Windows drive letters), `RootDir`, `CurDir` (`.`), `ParentDir` (`..`), and
  `Normal(&OsStr)` for regular segments. Use this whenever you need to reason about a path
  symbolically instead of as a string.
- **`.ancestors()`** — iterator that yields the path itself, then its parent, then grandparent, and
  so on up to the root. Great for "walk upward" tasks like searching for a project marker
  (`Cargo.toml`, `.git/`) starting from a child directory.
- **`.is_absolute()`** / **`.is_relative()`** — platform-aware absolute-path check. On Unix,
  "absolute" means starting with `/`; on Windows, it means having a drive prefix *and* a root.
- **`.to_str()`** — returns `Option<&str>`. The `Option` is `None` only when the path contains
  non-UTF-8 bytes (possible on Unix, where filenames are arbitrary byte sequences).
- **`.to_string_lossy()`** — returns `Cow<str>`. Invalid UTF-8 is replaced with U+FFFD replacement
  characters. Use when you need a `str` no matter what and a lossy conversion is acceptable.
- **`.display()`** — returns a formatting wrapper that implements `Display`. Use this in `println!`
  / `write!` to print a path — it handles non-UTF-8 paths gracefully without allocating a new
  `String` the way `to_string_lossy` does.

### `OsStr`, `OsString`, and Platform Strings

- The operating system does not share Rust's "everything is UTF-8" worldview. **Unix** filenames are
  arbitrary byte sequences (no encoding guarantee); **Windows** filenames are UTF-16 and may contain
  unpaired surrogate code units that are not valid Unicode. Converting such a filename to `&str`
  must be allowed to fail — hence `Path::to_str` returns `Option<&str>`.
- **`OsStr`** is an unsized type representing a *platform string*: a superset of UTF-8 on every
  target. **`OsString`** is the owned, growable counterpart — `OsStr` is to `OsString` as `str` is
  to `String`. `Path` is simply a thin wrapper around `OsStr` that adds path-specific methods, and
  `PathBuf` wraps `OsString`.
- You rarely construct an `OsString` by hand. Most standard-library I/O APIs accept `impl
  AsRef<Path>` or `impl AsRef<OsStr>`, so passing `&str` / `String` / `PathBuf` / `&Path` all works
  uniformly. Reach for `OsString` only when you must handle filenames that can't round-trip through
  UTF-8.

### The `AsRef<Path>` Pattern

- Most functions in `std::fs` are declared as `pub fn read_to_string<P: AsRef<Path>>(path: P) ->
  io::Result<String>`. That is why a string literal, a `String`, a `&String`, a `PathBuf`, and a
  `&Path` can all flow into the same parameter — every one of those types implements `AsRef<Path>`,
  directly or via the blanket `impl<T: AsRef<U>> AsRef<U> for &T`.
- The `AsRef<T>` trait itself is covered in module 008 section 17 — the `std::fs` API is one of its
  most visible users.

### Copying, Renaming, and Canonicalizing

- **`fs::copy(src, dst)`** copies file contents from one path to another. Returns the number of
  bytes copied as `io::Result<u64>`. Overwrites `dst` if it exists. Does not preserve timestamps on
  every platform.
- **`fs::rename(src, dst)`** moves or renames a file/directory. On the same filesystem this is an
  atomic metadata change; across filesystems it may fail on some platforms (you would need to fall
  back to copy + delete).
- **`fs::canonicalize(path)`** resolves symlinks and returns the absolute, canonical form of the
  path. The file must exist.
*/

fn paths_and_directories() -> Result<(), Box<dyn std::error::Error>> {
    // --- Path inspection ---
    let path = Path::new("Cargo.toml");
    println!("path: {}", path.display());
    println!("  exists: {}", path.exists());
    println!("  is_file: {}", path.is_file());
    println!("  parent: {:?}", path.parent());
    println!("  file_name: {:?}", path.file_name());
    println!("  file_stem: {:?}", path.file_stem());
    println!("  extension: {:?}", path.extension());

    // --- Path building with push (mutating) ---
    let mut built = PathBuf::from("project");
    built.push("src");
    built.push("main.rs");
    println!("built with push: {}", built.display()); // project/src/main.rs

    // --- Path building with join (non-mutating) ---
    let base = Path::new("project");
    let joined = base.join("src").join("main.rs");
    println!("built with join: {}", joined.display()); // project/src/main.rs

    // --- set_extension ---
    let mut file_path = PathBuf::from("report.txt");
    file_path.set_extension("csv");
    println!("after set_extension: {}", file_path.display()); // report.csv

    // --- Collecting path segments into a PathBuf ---
    let segments = ["usr", "local", "bin", "rustc"];
    let collected: PathBuf = segments.iter().collect();
    println!("collected: {}", collected.display()); // usr/local/bin/rustc

    // --- File metadata ---
    let meta = fs::metadata("Cargo.toml")?;
    println!("Cargo.toml metadata:");
    println!("  size: {} bytes", meta.len());
    println!("  is file: {}", meta.is_file());
    if let Ok(modified) = meta.modified() {
        println!("  last modified: {modified:?}");
    }

    // --- read_dir: list immediate directory contents ---
    println!("project root contents (first 8 entries):");
    for entry in fs::read_dir(".")?.take(8) {
        let entry = entry?;
        let file_type = if entry.file_type()?.is_dir() {
            "dir "
        } else {
            "file"
        };
        println!("  [{file_type}] {}", entry.file_name().to_string_lossy());
    }

    // --- Creating and removing directories ---
    let temp_dir = env::temp_dir().join("rust_mod010_dirs");
    let nested = temp_dir.join("level1").join("level2");

    // create_dir_all creates parent directories recursively
    fs::create_dir_all(&nested)?;
    println!("created nested dirs: {}", nested.display());
    println!("  nested exists: {}", nested.exists());

    // remove_dir_all removes the directory and all contents
    fs::remove_dir_all(&temp_dir)?;
    println!("  after removal: {}", temp_dir.exists());

    // --- Extended path inspection ---
    // components() classifies each segment as Prefix, RootDir,
    // CurDir, ParentDir, or Normal(&OsStr).
    let p = Path::new("/usr/local/bin/./rustc");
    println!("components of {}:", p.display());
    for comp in p.components() {
        println!("  {comp:?}");
    }

    // ancestors() walks upward from the path toward the root.
    let nested = Path::new("/a/b/c/d");
    println!("ancestors of {}:", nested.display());
    for anc in nested.ancestors() {
        println!("  {}", anc.display());
    }

    // is_absolute / is_relative are platform-aware.
    let abs = Path::new("/etc/hosts");
    let rel = Path::new("src/main.rs");
    println!("is_absolute {abs:?} = {}", abs.is_absolute());
    println!("is_relative {rel:?} = {}", rel.is_relative());

    // to_str / to_string_lossy / display — three ways to get text.
    let utf8_path = Path::new("reports/2024.txt");
    println!("to_str:          {:?}", utf8_path.to_str()); // Some("reports/2024.txt")
    println!("to_string_lossy: {}", utf8_path.to_string_lossy()); // owned or borrowed str
    println!("display:         {}", utf8_path.display()); // Display wrapper

    // --- fs::copy / fs::rename / fs::canonicalize ---
    let copy_src = env::temp_dir().join("rust_mod010_src.txt");
    let copy_dst = env::temp_dir().join("rust_mod010_dst.txt");
    let renamed = env::temp_dir().join("rust_mod010_renamed.txt");

    fs::write(&copy_src, "source contents\n")?;
    let copied = fs::copy(&copy_src, &copy_dst)?;
    println!("fs::copy copied {copied} bytes");

    fs::rename(&copy_dst, &renamed)?;
    println!("fs::rename -> {}", renamed.display());

    // canonicalize resolves symlinks and relative segments into an
    // absolute path. The file must exist.
    let canonical = fs::canonicalize(&renamed)?;
    println!("fs::canonicalize -> {}", canonical.display());

    fs::remove_file(&copy_src)?;
    fs::remove_file(&renamed)?;

    Ok(())
}

// =================================================================================================
// Section 7: walkdir Crate — Recursive Directory Traversal
// =================================================================================================

/*
## walkdir Crate — Recursive Directory Traversal

- **`std::fs::read_dir`** (section 6) lists only the **immediate** children of a directory. For
  recursive traversal, use the **`walkdir`** crate.
- **`WalkDir::new(path)`** creates a recursive directory walker. Calling `.into_iter()` produces an
  iterator of `Result<walkdir::DirEntry, walkdir::Error>`.
- **Configuration methods** (called on `WalkDir` before iterating):
  - `.max_depth(n)` — limit recursion depth (1 = root + immediate children; combine with
    `.min_depth(1)` for `read_dir`-equivalent).
  - `.min_depth(n)` — skip entries above this depth (1 = skip the root directory entry itself).
  - `.follow_links(true)` — follow symbolic links (default: false).
- **`walkdir::DirEntry`** methods:
  - `.path()` — returns `&Path`.
  - `.file_name()` — returns `&OsStr`.
  - `.file_type()` — returns `FileType` with `.is_file()`, `.is_dir()`, `.is_symlink()`.
  - `.depth()` — recursion depth from the root (0 = root itself).
- **Common patterns:**
  - `.into_iter().filter_map(Result::ok)` — skip unreadable entries (e.g., permission errors).
  - Filter by extension: `entry.path().extension().is_some_and(|ext| ext == "rs")`
  - Filter by file type: `entry.file_type().is_file()`
- `walkdir` is used in the **find** exercise (with regex name matching and type filtering) and in
  the **grep** exercise (to recursively find files before searching their contents).
*/

fn walkdir_traversal() {
    // --- Walk src/ and list all .rs files with depth indentation ---
    println!("Rust files in src/ (walkdir):");
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "rs"))
        .take(10)
    // limit output
    {
        let indent = "  ".repeat(entry.depth());
        println!("{indent}{}", entry.path().display());
    }

    // --- Walk with max_depth(1) to mimic non-recursive listing ---
    println!("top-level items in src/ (max_depth=1):");
    for entry in WalkDir::new("src")
        .max_depth(1)
        .min_depth(1) // skip the "src" root entry itself
        .into_iter()
        .filter_map(Result::ok)
    {
        let kind = if entry.file_type().is_dir() {
            "dir "
        } else {
            "file"
        };
        println!("  [{kind}] {}", entry.file_name().to_string_lossy());
    }

    // --- Count files vs directories in src/ ---
    let mut file_count = 0;
    let mut dir_count = 0;
    for entry in WalkDir::new("src")
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() {
            file_count += 1;
        } else if entry.file_type().is_dir() {
            dir_count += 1;
        }
    }
    println!("src/ contains: {file_count} files, {dir_count} directories");

    // --- Find the largest .rs file ---
    let largest = WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|e| {
            let size = e.path().metadata().ok()?.len();
            Some((e.path().to_path_buf(), size))
        })
        .max_by_key(|(_, size)| *size);

    if let Some((path, size)) = largest {
        println!("largest .rs file: {} ({size} bytes)", path.display());
    }
}

// =================================================================================================
// Section 8: Regex Crate — Pattern Matching
// =================================================================================================

/*
## Regex Crate — Pattern Matching

- **`regex`** is Rust's standard regular expression crate. Rust has no built-in regex support — you
  must add `regex` to Cargo.toml.
- **`Regex::new(pattern)`** compiles a pattern and returns `Result<Regex, Error>`. Compilation is
  expensive — compile the pattern **once** and reuse the `Regex` object.
- **Matching methods:**
  - `.is_match(text)` — returns `bool`.
  - `.find(text)` — returns `Option<Match>` for the first match. `Match` has `.start()`, `.end()`,
    `.as_str()`.
  - `.find_iter(text)` — iterator of all non-overlapping `Match` values.
  - `.captures(text)` — returns `Option<Captures>` for the first match with capture groups.
  - `.captures_iter(text)` — iterator of `Captures` for all matches.
- **Capture groups:** parentheses `()` create numbered groups. Access with `caps[0]` (full match),
  `caps[1]`, `caps[2]`, etc. **Named groups:** `(?P<name>...)` are accessed with `caps["name"]`.
- **Replacement:**
  - `.replace(text, rep)` — replaces the first match, returns `Cow<str>` — borrows the input when no
    match is found (zero allocation) or returns an owned String when replacements occur.
  - `.replace_all(text, rep)` — replaces all matches.
  - Backreferences in replacement: `$1`, `$2` or `${name}`.
- **Common pattern syntax:** `\d` (digit), `\w` (word char), `\s` (whitespace), `[a-z]` (character
  class), `[^...]` (negated class), `^` (start), `$` (end), `+` (one or more), `*` (zero or more),
  `?` (optional), `{n,m}` (repetition), `|` (alternation), `()` (grouping/capture).
- **Performance tip:** for static patterns used in functions that may be called repeatedly, store
  the compiled `Regex` in a `LazyLock<Regex>` (from module 001) so it is compiled exactly once.
- **Limitation:** Rust's regex crate does **not** support lookahead or lookbehind assertions (unlike
  PCRE/Perl regex). It guarantees **linear-time** matching — patterns cannot cause catastrophic
  backtracking.
*/

// A compile-once regex using LazyLock — the pattern is compiled on
// first access and reused for all subsequent calls
static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\w+([.\-+]\w+)*@\w+([.\-]\w+)*\.\w{2,}$").unwrap());

fn regex_patterns() {
    // --- is_match: simple validation ---
    println!("email validation with LazyLock<Regex>:");
    let emails = ["user@example.com", "not-an-email", "a+b@c.org", "missing@"];
    for email in &emails {
        let valid = EMAIL_REGEX.is_match(email);
        println!("  {email:25} -> valid: {valid}");
    }

    // --- find_iter: finding all matches ---
    let re = Regex::new(r"\d{4}").unwrap();
    let text = "Call 1234 or 5678, not 99 or 123.";
    let numbers: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();
    println!("4-digit numbers found: {numbers:?}"); // ["1234", "5678"]

    // --- captures with named groups: structured extraction ---
    let date_re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let text = "Events: 2024-12-25 and 2025-01-01";
    println!("parsed dates:");
    for caps in date_re.captures_iter(text) {
        println!(
            "  year={}, month={}, day={}",
            &caps["year"], &caps["month"], &caps["day"]
        );
    }

    // --- captures with numbered groups ---
    let time_re = Regex::new(r"(\d{2}):(\d{2}):(\d{2})").unwrap();
    let log = "Started at 08:30:00, finished at 17:45:30";
    for caps in time_re.captures_iter(log) {
        println!(
            "  time: {} (h={}, m={}, s={})",
            &caps[0], &caps[1], &caps[2], &caps[3]
        );
    }

    // --- replace_all: text transformation ---
    let phone_re = Regex::new(r"\d{3}[-.\s]?\d{3}[-.\s]?\d{3}").unwrap();
    let text = "Contact: 123-456-789 or 987.654.321";
    let redacted = phone_re.replace_all(text, "[REDACTED]");
    println!("redacted: {redacted}");

    // --- replace_all with backreferences ---
    // Reformat dates from YYYY-MM-DD to DD/MM/YYYY
    let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "Dates: 2024-12-25 and 2025-01-01";
    let reformatted = date_re.replace_all(text, "$3/$2/$1");
    println!("reformatted dates: {reformatted}");

    // --- Splitting with regex ---
    let re = Regex::new(r"[;,\s]+").unwrap();
    let text = "one,two;  three  four,five";
    let parts: Vec<&str> = re.split(text).collect();
    println!("regex split: {parts:?}"); // ["one", "two", "three", "four", "five"]
}

// =================================================================================================
// Section 9: Seek, Cursor, and Locked Standard Streams
// =================================================================================================

/*
## Seek, Cursor, and Locked Standard Streams

### The `Seek` Trait

- **`std::io::Seek`** adds a cursor-relative-position concept to readers and writers that support
  random access. It is implemented by `File`, `Cursor<T>`, and any adapter that wraps such a type.
- The single required method is `fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>`, which
  returns the **new** absolute byte offset.
- **`SeekFrom`** is an enum describing where to seek to:
  - `SeekFrom::Start(n)` — absolute offset `n` (u64) from the beginning. The most common form.
    `seek(SeekFrom::Start(0))` is "rewind to the beginning".
  - `SeekFrom::End(n)` — signed offset (i64) from the end of the stream. `SeekFrom::End(0)` is
    end-of-file; `SeekFrom::End(-4)` goes to four bytes before EOF (useful for reading a trailing
    length/checksum field).
  - `SeekFrom::Current(n)` — signed offset (i64) from the current position. Handy when skipping a
    variable-length field.
- **Performance note**: seeking is cheap when the target byte is already in the kernel's page cache,
  but can trigger a disk read otherwise. For streaming workloads prefer forward-only reading.

### `Cursor<T>` — an in-memory Reader/Writer

- **`std::io::Cursor<T>`** wraps an in-memory buffer (`Vec<u8>`, `&[u8]`, or `&mut [u8]`) and
  exposes it as a type implementing `Read + Write + Seek`. The name "cursor" comes from the tracked
  byte offset inside the buffer.
- Typical uses:
  - **Read from a byte slice**: `Cursor<&[u8]>` is how you parse a decoded payload without writing
    it to disk first.
  - **Write to a growable buffer**: `Cursor<Vec<u8>>` gives you a seekable `Write` — perfect for
    building a binary blob with `writeln!`, `write_all`, and the occasional `seek` to patch a
    header.
  - **Unit tests for generic code**: any function generic over `R: Read` or `W: Write` can be
    exercised with a `Cursor` instead of a real file.

### Locked Standard Streams

- **`io::stdin()` / `io::stdout()` / `io::stderr()`** return owned handles that internally share a
  mutex. Every `read_line`, `write`, or similar call locks that mutex, performs the I/O, and
  releases it. For many small calls in a tight loop that per-call locking is wasteful.
- **`stdin().lock()`** returns an `StdinLock` that acquires the mutex **once** and holds it until
  dropped. Every subsequent read is lock-free, which is noticeably faster for line-oriented tools.
  `StdinLock` implements `BufRead`, so you can call `.lines()` on it directly without wrapping in a
  `BufReader`. The same pattern applies to `stdout().lock()` — the canonical way to write a lot of
  output fast.
*/

fn seek_cursor_and_locked_streams() -> Result<(), Box<dyn std::error::Error>> {
    // --- Cursor<Vec<u8>>: write, seek, read back ---
    // Cursor makes a Vec<u8> behave like a seekable File, which is
    // the canonical way to exercise Read/Write/Seek code in tests.
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    writeln!(cursor, "first line")?;
    writeln!(cursor, "second line")?;
    println!(
        "Cursor position after writes: {}",
        cursor.stream_position()?
    );

    // Rewind and read back everything we just wrote.
    cursor.seek(SeekFrom::Start(0))?;
    let mut echoed = String::new();
    cursor.read_to_string(&mut echoed)?;
    println!("Cursor read-back:\n{echoed}");

    // --- Cursor<&[u8]>: read-only in-memory reader ---
    let payload: &[u8] = b"alpha,beta,gamma";
    let mut reader = Cursor::new(payload);
    let mut first_word = [0u8; 5];
    reader.read_exact(&mut first_word)?;
    println!(
        "Cursor<&[u8]> first 5 bytes: {:?}",
        std::str::from_utf8(&first_word)?
    );

    // --- Seek on an actual File ---
    // Create a temp file with a known layout, then bounce the
    // position around with SeekFrom::{Start, End, Current}.
    let temp = env::temp_dir().join("rust_mod010_seek.bin");
    fs::write(&temp, b"0123456789")?; // 10 bytes, each digit its own byte

    let mut file = File::open(&temp)?;
    let mut one_byte = [0u8; 1];

    // SeekFrom::Start(n) — absolute offset from the beginning.
    file.seek(SeekFrom::Start(4))?;
    file.read_exact(&mut one_byte)?;
    println!("byte at offset 4: {:?}", one_byte[0] as char); // '4'

    // SeekFrom::End(-1) — 1 byte before end of file.
    file.seek(SeekFrom::End(-1))?;
    file.read_exact(&mut one_byte)?;
    println!("byte at EOF-1:    {:?}", one_byte[0] as char); // '9'

    // SeekFrom::Current — relative to the current cursor.
    // After reading at EOF-1 the cursor is at EOF; back up 3 bytes.
    file.seek(SeekFrom::Current(-3))?;
    file.read_exact(&mut one_byte)?;
    println!("byte after rel -3: {:?}", one_byte[0] as char); // '7'

    fs::remove_file(&temp)?;

    // --- stdin().lock() / stdout().lock() ---
    // Acquire the stdout mutex once, then fire many writes without
    // re-locking. This is how high-throughput CLI tools avoid the
    // per-call locking overhead of plain println! in tight loops.
    // We also demonstrate that StdinLock implements BufRead — no
    // explicit BufReader wrapping required.
    {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        writeln!(out, "locked stdout: write #1")?;
        writeln!(out, "locked stdout: write #2")?;
        writeln!(out, "locked stdout: write #3")?;
        // `out` drops here → mutex released.
    }

    // StdinLock: not called from run() because it would block on input.
    // The snippet below shows the pattern you would use in a real CLI:
    //
    //     let stdin = io::stdin();
    //     let handle = stdin.lock();              // StdinLock: BufRead
    //     for line in handle.lines() {            // .lines() from BufRead
    //         let line = line?;
    //         println!("got: {line}");
    //     }

    println!("seek_cursor_and_locked_streams section executed");
    Ok(())
}

// =================================================================================================
// pub fn run()
// =================================================================================================

pub fn run() {
    text_processing_methods();
    command_line_and_environment();
    clap_argument_parsing();

    if let Err(e) = file_reading() {
        eprintln!("file_reading error: {e}");
    }
    if let Err(e) = file_writing() {
        eprintln!("file_writing error: {e}");
    }
    if let Err(e) = paths_and_directories() {
        eprintln!("paths_and_directories error: {e}");
    }

    walkdir_traversal();
    regex_patterns();

    if let Err(e) = seek_cursor_and_locked_streams() {
        eprintln!("seek_cursor_and_locked_streams error: {e}");
    }
}
