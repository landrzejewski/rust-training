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

    /* Typy danych

      - muszą być znane/określone w czasie kompilacji - Rust jest statycznie typowany
      - w większości przypadków mogą być wywnioskowane automatycznie przez kompilator
      - typy skalarne/proste - integers, floating-point numbers, booleans and characters
     - typy złożone - tuples, arrays
   */

    /* Integers

     Length	   Signed type    Unsigned type
     8-bit	   i8	          u8
     16-bit	   i16	          u16
     32-bit	   i32 // default u32
     64-bit	   i64	          u64
     128-bit   i128	          u128

     32/64bit  isize	      usize  // używane jako indeksy - nie mogą być ujemne, muszą być pojemne/duże, zależą od arch.

     Number literals	   Example
     Decimal	           98_222_000
     Hex	               0xff
     Octal	               0o77
     Binary	               0b1111_0000
     Byte (u8 only)	       b'A'

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
      
    }

    fn add(value: i32, other_value: i32) -> i32 {
        let result = value + other_value;
        return result;
        //result
        //value + other_value // dodanie średnika spowoduje błąd - funkcja będzie wtedy zwracać ()
    }
