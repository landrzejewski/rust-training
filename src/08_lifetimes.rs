/*
   Lifetime nie zmienia czasu życia referencji, opisuje relacje między czasem życia wielu referencji gwarantując bezpieczeństwo
   Rust pozwala na wyłącznie 1 właściciela pamięci
   Rust pozwala na wiele referencji
   Lifetimes zapewniają to że każda referecja odności się do ważnej / istniejącej pamięci
   Innymi słowy gwarantuje, że pamięc nie będzie wyczyszczona do czasu konieczności użycia/ dostępu
*/
mod main;
mod app;

fn main() {
    let x;
    {
        let y = 10;
        x = &y;
    }
    // println!("x: {x}"); // błąd - próba użycia zmienna, która już nie istnieje

    let a;
    {
        let b = String::from("aaa");
        a = &b;

    }
    println!("{}", a); // bład zmienna właściciel nie istnieje

    let s1 = String::from("abc");
    let result;
    {
        // let s2 = String::from("def");
        // result = get_longer(&s1, &s2);
        let s2 = "def";  // str żyje/istnieje przez cały czas działania aplikacji
        result = get_longer(s1.as_str(), s2);
    }
    println!("{}", result);

    let last_name = String::from("Kowalski");
    let address = String::from("test");

    let client = Person {
        first_name: "Jan",
        last_name: &last_name,
        address,
    };

    // stałe mają domyśly lifetime static
}

// pamięc zostanie zniszczona przed potencjalnym użyciem
/*fn get_ref() -> &i32 {
    let a = 4;
    &a
}*/

// jest ok - zasięgi/scopes/czas zycia wejścia i wyjścia są identyczne
fn get_ref(aa: &i32) -> &i32 {
   aa
}
// to samo z konfiguracją
fn get_ref2<'a>(aa: &'a i32) -> &'a i32 {
    aa
}

/*
  - dla metod, które mają dokładnie jeden argument, kompilator przypisuje ten sam lifetime parameter do argumentu jak i rezultatu
  - dla metod z argumentami, kompilator przypisuje różne (kolejne) lifetime parameters do argumentów oraz rezultatu
  - dla metod z argumentami, które zawierają &self lub &mut self lifetime rezultatu jest taki sam jak dla atrybutu &self lub &mut self
 */

fn get_longer<'a>(text: &'a str, other_text: &'a str) -> &'a str { // w tym przypadku zwracana referencja musi być ważna tak długo jak referencje przekazywanych argumentów
    if text.len() >= other_text.len() { text } else { other_text }
}

// instancja Person nie może przetrwać dłużej niż referencje, które posiada / przechowuje
struct Person<'a, T> {
    first_name: &'a str,
    last_name: &'static str, // static oznacza czas życia całego programu
    address: T,
}