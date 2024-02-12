#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
   // proste zmienne mogą być wydrukowane za pomocą makra println! oraz {} (wymagana implementacja Display trait)
    // zmienne złożone jak np. struktury wymagają implementacji Debug trait i użycia {:?} lub {:#?} (pretty printing)
    // możliwe jest także formatowanie {variable name:padding value alignment(<^>) minimum.maximum}

    println!("Hello, world!");

    // Zmienne
    let x = 10; // deklaracja zmiennej niemutowalnej, typ jest wnioskowany automatycznie, ale może być opcjonalnie zdefiniowany
    println!("The value of x is: {x}");
    // x = 6; błąd kompilacji - zmienna nie może zmienić swojej wartości

    let y; // nie ma konieczności deklaracji z jednoczesną inicjalizacją, ale musi ona nastąpić przed pierwszym użyciem
    y = 10;
    println!("The value of y is: {y}");

    let mut z = 10; // deklaracja zmiennej mutowalnej
    z = 30; // można zmienić wartość, ale nie typ
    println!("The value of z is: {z}");

    let _o = 10; // zmienna nie będzie powodować warning, nawet jeśli nigdy nie będzie użyta

     // Shadowing
     let x = 20;
     {
         let mut x:f32 = x as f32 * 3.0;
         println!("The value of x in the inner scope {x}")
     }
     println!("The value of x in the outer scope {x}");
 
     // Shadowing przydaje się kiedy chcemy pracować ze zmienną i nie za bardzo przejmować się etapami pośrednimi
     let some_result = 5;
     let mut some_result = add(some_result, 3);
     let some_result = some_result + 5;

      /* Stałe

        - muszą mieć określony typ - nie jest on wnioskowany automatycznie
        - ich wartość musi być znana w czasie kompilacji
        - nie mogą zmieniać swojej wartości (użycie mut nie jest dozwolone)
        - mogą mieć dowolny zasięg, także globalny
    */
      const MONTH_OF_THE_YEAR: i8 = 4;
      const TIMEOUT: i64 = 3600 * 10;

     // https://github.com/landrzejewski/rust-training

    /* Typy danych

      - muszą być znane/określone w czasie kompilacji - Rust jest statycznie typowany
      - w większości przypadków mogą być wywnioskowane automatycznie przez kompilator
      - typy skalarne/proste - integers, floating-point numbers, booleans and characters
      - typy złożone - tuples, arrays
    */

    /* Integers

     Length	   Signed type    Unsigned type
     8-bit	   i8	            u8
     16-bit	   i16	          u16
     32-bit	   i32 // default u32
     64-bit	   i64	          u64
     128-bit   i128	          u128

     32/64bit  isize	        usize  // używane jako indeksy - nie mogą być ujemne, muszą być pojemne/duże, zależą od arch.

     Number literals	   Example
     Decimal	           98_222_000
     Hex	               0xff
     Octal	             0o77
     Binary	             0b1111_0000
     Byte (u8 only)	     b'A'

     let small_number = 10u8;
     let big_number = 100_000_000_i32;

     - w trybie debug kompilator dodaje weryfikację wystąpienia integer overflow (asercja) i przerywa wykonanie programu
       w przypadku jego wystąpienia
     */

    // let a: u8 = 300; // integer overflow

    /* Floating-point numbers

     - zgodne ze standardem IEEE-754

     Length	   Type
     32-bit	   f32
     64-bit	   f64 // default
     */

    println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);
    println!("The smallest i16: {} The biggest i16: {}", i16::MIN, i16::MAX);
    println!("The smallest u16: {} and the biggest u16: {}", u16::MIN, u16::MAX);
    println!("The smallest i32: {} The biggest i32: {}", i32::MIN, i32::MAX);
    println!("The smallest u32: {} The biggest u32: {}", u32::MIN, u32::MAX);
    println!("The smallest i64: {} The biggest i64: {}", i64::MIN, i64::MAX);
    println!("The smallest u64: {} The biggest u64: {}", u64::MIN, u64::MAX);
    println!("The smallest i128: {} The biggest i128: {}", i128::MIN, i128::MAX);
    println!("The smallest u128: {} The biggest u128: {}", u128::MIN, u128::MAX);
      
    let value = 14.3;
    let other_value = 14.;
    let small_value: f32 = 0.1;

    // Wspierane są standardowe operatory matematyczne https://rust-book.cs.brown.edu/appendix-02-operators.html
    // Uwaga: w przeciwieństwie do innych języków należy zadbać o jawną konwersję typów

    let sum = 5 + 10;
    let difference = 11.5 - 4 as f32;
    let product = 4.0 * 12.5;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // obcięcie wyniku do 0
    let remainder = 54 % 5;

    let my_float: f32 = 5.0;
    let my_other_float = 8.5; // rust wnioskuje typ jako f32 z kontekstu (operacja poniżej)
    let result = my_float + my_other_float;

    /* Boolean

     - dopuszczalne wartości to true i false
     */

    let positive_result = true;
    let negative_result: bool = false;

    /* char

    - reprezentuje Unicode Scalar Value (może przechowywać złożone znaki)
    - ma rozmiar 4 bajtów
    */

    let c = 'a';
    let z: char = 'ℤ';
    let cat = '😻';

    // W przeciwieństwie do chars ciągi znaków mogą zajmować różną ilość bytów w pamięci (od 1 do 4 bytów optymalizacja)

    let str1 = "Lukasz";
    println!("str1 is {} bytes and {} chars", str1.len(), str1.chars().count());
    println!("str1 {:?} bytes.", "L".as_bytes());
    let str2 = "Łukasz";
    println!("str1 is {} bytes and {} chars", str2.len(), str2.chars().count());
    println!("str2 {:?} bytes.", "Ł".as_bytes());


    /* Tuple

     - grupuje elementy dowolnego typu
     - ma z góry określoną i niezmienną długość
     - dostęp do poszczególnych elementów odbywa się przez indeksy/pozycję lub destrukcję
     */

    let tuple = (11, 11.0, 11);
    let (a, b, c) = tuple; // destrukturyzacja krotki, ilość elementów musi się zgadzać, można zignorować wybrane elementy
    let (_, b, c) = tuple;
    

    let unit = (); // pusta krotka, unit

    /* Array

    - grupuje elementy tego samego typu
    - ma z góry określoną i niezmienną długość
    - dostęp do elementów odbywa się przez indeksy, przekroczenie poprawnego zakresu kończy się przerwaniem programu (panic)
    - alokacja pamięci następuje na stosie (stack) jak dla typów prymitywnych
    */

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let matrix = [[1, 2], [3, 4]];

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("First number: {}", numbers[0]);
    let zeros = [0; 7]; // stworzenie tablicy o danym rozmiarze, wypełnionej 0
    println!("Array of zeros: {:?}", zeros);

    // możemy stworzyć/użyć slice array (trzeba użyć & bo rozmiar slice jest określany dynamicznie i znieznany w czasie kompilacji)
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let two_to_five_included = &array_of_ten[2..=5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    let test:[i32;0] = [];

    println!("Two to five: {two_to_five:?}, Start at one: {start_at_one:?}, End at five: {end_at_five:?}, Everything: {everything:?}");  
    
     // Funkcje i wyrażenia

     println!("Addition result: {}", add(1, 2));
     println!("Validation result: {}", validate(10, is_even));
     println!("Validation result: {}", validate(10, |value| value % 2 == 0));
 
     let score = {
         let x = 3;
         x * 3
     };
 
 // Control flow

    /* Wyrażenia warunkowe

     - pozwalają na wykonywanie sekcji kodu po spełnieniu/niespełnieniu określonego warunku
     - wynik wyrażenia warunkowego musi być typu bool (nie ma niejawnej konwersji ja w niektórych językach)
     - zwracają rezultat, który może zostać przypisany do zmiennej lub wykorzystany w inny sposób
       (zwracane wyniki muszą być tego samego typu, trzeba zadbać o wszystkie możliwe scenariusze)
    */

    let number = 3;
    if number < 5 {
        println!("Number is lower than 5");
    } else if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is equal 5");
    }

    let some_condition = true;
    let option: char = if some_condition { 'a' } else { 'b' };

    // Pattern matching

    let dice_roll = 3;
    let roll_result = match dice_roll {
        6 => {
            println!("You won!");
            "Win"
        },
        val @ 1 => {
            println!("You lost! {val}");
            "Loose"
        },
        value=> {  // jeśli wartość nas nie interesuje można użyć _
            println!("You hit {value}, try again");
            "Try again"
        }
    };

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),  // match guard
        Some(x) => println!("The number {} is odd", x),
        _ => ()
    }

    // Pętle

    // loop

    let mut counter = 0;
    let result = 'myLoop: loop {
        counter += 1;
        if counter == 20 {
            break 'myLoop counter * 2;
        }
    };
    println!("The loop result is: {result}");

    // while

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // for in

    let elements = [1, 2, 3, 4, 5];

    for index in 0..4 {
        println!("Current element: {}", elements[index]);
    }

    for element in elements {
        println!("Current element: {}", element);
    }

    for _ in (0..elements.len()).step_by(2) {
        println!("Next loop");
    }
    
    }

    fn add(value: i32, other_value: i32) -> i32 {
        let result = value + other_value;
        return result;
        //result
        //value + other_value // dodanie średnika spowoduje błąd - funkcja będzie wtedy zwracać ()
    }


fn validate<T>(value: T, predicate: fn(T) -> bool) -> bool {
  predicate(value)
}

fn is_even(value: i32) -> bool {
  value % 2 == 0
}

