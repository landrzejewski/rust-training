/* Ownership

 - zapewnia bezpieczeństwo (brak nieprzewidywalnych zachowań) dla programów pisanych w Rust https://doc.rust-lang.org/reference/behavior-considered-undefined.html
 - Rust nie pozwala na manualne zarządzanie pamięcią sterty - dzieje się to automatycznie (poziom kompilacji, brak garbage collectora)
 - Zasady:
   - wszystkie dane na stercie mogą mieć tylko jednego właściciela
   - kiedy właściciel przestaje być dostępny (zakończenie zasięgu w którym był zdefiniowany), Rust zwalnia powiązaną z nim pamięć na stercie
   - własność pamięci może zostać przeniesiona w momencie przypisania do innej zmiennej lub wywołania funkcji
   - dostęp do pamięci na stercie może się odbywać wyłącznie za pośrednictwem aktualnego właściciela/posiadacza
 */
fn main() {
   /* 
        Stack variables/memory
        - Szybka alokacja i dostęp
        - Pamięć jest automatycznie zwalniana/odzyskiwana po zakończeniu zasięgu w którym zadeklarowano zmienną
        - Wykorzystywana w przypadku kiedy rozmiar zmiennej jest znany w czasie kompilacji i nie może ulegać zmianie (typy skalarne, tablice i krotki)
        - Przypisanie do nowej zmiennej powoduje skopiowanie wartości
    */

    let mut a = 5;
    let b = a; // copy
    a = 10;
    {
        let c = 10;
    }
    //println!("{}, {}, {}", a, b, c);

    /*
        Heap variables/memory
        - Duża elastyczność - pamięć może być dynamicznie przydzialana/alokowana w razie potrzeby (Vector, HashMap, String...)
          kosztem szybkości alokacji i dostępu
        - Może istnieć poza zasięgiem w jakim została stworzona
        - Pamięć jest zwalniana automatycznie kiedy ostatni właściciel przestaje istnieć
        - Przypisanie do nowej zmiennej powoduje zmianę właściciela
     */

    /*
    let text = String::from("rust");
    let other_text = text; // przeniesienie własnoci
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    let other_text = text.clone(); // skopiowanie wartości
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    show(text); // przeniesienie własnoci
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    let text = show_with_result(text); // przeniesienie własnoci x 2
    println!("{}", text);
    */

    /*
    let text = String::from("rust");
    show_with_ref(&text); // wypozyczenie
    println!("{}", text);
    */

    let mut text = String::from("rust");
    show_with_mut_ref(&mut text); // wypozyczenie
    println!("{}", text);

    let z = Box::new(3);

    /*
        W przypadku typów złożonych, alokowanych na stercie, posługujemy się wskaźnikiem (trzymanym na stosie),
        dzięki temu możliwe jest współdzielenie wartości zmiennej bez konieczności jej kopiowania.
        Dealokacja pamięci odbywa się automatycznie w momencie niszczenia ramki właściciela
        Istnieje możliwość zrobienia kopii typu złożonego jeśli implementowany jest Clone trait (wywołanie metody clone), w wielu
        przypadkach jest to nieefektywne dlatego lepiej użyć referencji
    */

    /*
    let string = String::from("text"); // zawsze na stercie, dynamicznie alokowane
    let str: &str = "Text"; // wskaźnik na pamięc embedded, niemutowalne 
    */

    /* 
    let a = String::from("aaa");
    let b = &a;
    let c = &a;
    println!("{},{},{}", a , b , c); // jest ok, właściciel jest jeden, do tego mamy 2 widoki read only
    */

    let mut a = String::from("aaa");
    let b = &mut a;
    b.push('b'); // b nie jest właścicielem, to tylko referencja read only
    let c = &a; // odczyt/dostęp nie jest mozliwy bo nadal istnieje szansa mutacji (linia ponizej)
    // b.push('c'); // błąd

    /* 
    Dereferencja
    - może się odbywać manualnie z użyciem *
    - może się odbywać automatycznie - użycie operatora . oraz makra
    */

    let mut x = Box::new(1);
    let a = *x; // *x odczytuje wartość ze sterty czyli 1
    *x += 1;

    // utworzenie kopii może być niejawne implementacji wymaga copy trait
    let test = Test { value: 4 };
    mutate_test(a); // niejawny clone
    println!("{:?}", a);
}

fn show(text: String) {
    println!("{text}");
}

fn show_with_result(text: String) -> String {
    println!("{text}");
    text
}

fn show_with_ref(text: &String) {
    println!("{text}");
}

fn show_with_mut_ref(text: &mut String) {
    text.push('!');
    println!("{text}");
}

#[derive(Debug, Clone, Copy)]
struct Test {
    value: i64,
}

fn mutate_test(mut test: Test) {
    test.value = 5;
    println!("{:?}", test);
}