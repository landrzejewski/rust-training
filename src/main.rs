/* Enums

 - reprezentują wyliczenie stałych/możliwych wariantów
 - pozwalają na zdefiniowanie metod (tak samo jak w przypadku struktur)
 */

 use std::mem::size_of_val;

 fn main() -> Result<(), String> {
     let qr_code: Barcode = Barcode::Qr(String::from("345345345345"));
     let product_code = Barcode::Product { id: 5, value: String::from("123") };
 
     println!("{}", size_of_val(&qr_code));
     println!("{}", size_of_val(&product_code));
     println!("{}", size_of_val(&Barcode::Other));
     //let a  = [qr_code, product_code, Barcode::Other];
 
     //println!("Value: {}", Values::from_i32(17));

          // Destrukcja elementów wyliczenia
 
          match qr_code {
            Barcode::Other => println!("Other barcode"),
            Barcode::Product { value, id} => println!("Product {id}:{value} "),
            Barcode::Qr(value) => println!("Qr {value} "),
            _ => ()
        }
    
        match product_code {
           Barcode::Product { id: id_value @ 4..=10, value: _ } => println!("Id in big range {id_value}"), // Bindowanie wartości w zakresie
           Barcode::Product { id: 1..=3, value: _ } => println!("Id in small range"),
           _ => {}
        }
 
     /* Jednym z wbudowanych typów wyliczenia jest Option reprezentujący wartość lub jej brak (alternatywa dla null)
 
     enum Option<T> {
         None,
         Some(T),
     }
     */
 
     let result = safe_div(3.0, 3.0)
        .expect("Division by 0");
 
     match safe_div(3.0, 3.0) {
         Some(value) => println!("3.0 / 3.0 = {}", value),
         _ => ()
     }
 
     if let Some(value) = safe_div(3.0, 3.0) {
         println!("3.0 / 3.0 = {}", value);
     }

    let Some(value) = safe_div(3.0, 3.0) else {
        return Err("error".to_string())
    };
    println!("3.0 / 3.0 = {}", value);

    let mut stack = vec![1, 2, 3];
    while let Some(value) = stack.pop() {
        println!("Value: {}", value);
    }

     /*
     enum Result<T, E> {
        Ok(T),
        Err(E),
     }
    */
 
     let result = safe_div_with_result(3.0, 3.0)?; // w przypadku Err zwrócenie/wyjście z funkcji
 
     match safe_div_with_result(3.0, 3.0) {
        Ok(value) => println!("3.0 / 3.0 = {}", value),
        Err(message) => println!("Errro: {message}")
    }
 
    Ok(())
 }
 
 enum Currency {
    Eur,
    Pln,
    Gbp,
 }
 
 enum Values {
    A = 17,
    B = 42,
    C
 }
 
 struct Money {
    value: f64,
    currency: Currency,
 }
 
 #[derive(Debug)]
 enum Barcode {
    Upc(i32, i32, i32, i32),
    Qr(String),
    Product { value: String, id: i64 },
    Other,
 }
 
 impl Barcode {
    fn get_info(&self) -> String {
        format!("Barcode {:?}", self)
    }
 }
 
 fn safe_div(value: f64, dividend: f64) -> Option<f64> {
    if dividend == 0.0 { None } else { Some(value / dividend) }
 }
 
 fn safe_div_with_result(value: f64, dividend: f64) -> Result<f64, String> {
     if dividend == 0.0 { Err("Division by 0".to_string()) } else { Ok(value / dividend) }
 }
 