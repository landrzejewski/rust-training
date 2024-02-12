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
    }

    fn add(value: i32, other_value: i32) -> i32 {
        let result = value + other_value;
        return result;
        //result
        //value + other_value // dodanie średnika spowoduje błąd - funkcja będzie wtedy zwracać ()
    }
