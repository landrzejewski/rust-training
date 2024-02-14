### Zadania
1. Napisz generator kolejnych elementów ciągu Fibonacciego (pętle, rekurencja)

2. Stwórz typ reprezentujący pieniądze
* Pieniądze mogą występować w różnych walutach
* Pieniądze mogą być wymieniane / konwertowane do innej waluty po wskazanym kursie
* Pieniądze można ze sobą dodawać, odejmować

3. Napisz grę kółko i krzyżyk
* Plansza ma rozmiar 3 x 3 pola
* Gracze zajmują na przemian wolne pola, stawiając na nich swój znak (kółko lub krzyżyk)
* Gra kończy się, kiedy wszystkie pola zostaną zajęte lub jeden z graczy zajmie sekwencję wygrywającą (kolumna, wiersz lub przekątna)
* Interfejs gry powinien opierać się linię komend / terminal

4. Napisz aplikacje umożliwiającą rejestrowanie wpływów/wydatków dla budżetu domowego. Aplikacja powinna rejestrować typ, rodzaj operacji oraz jej opis (podawane jako argumenty z linii poleceń)
oraz generować raport/tabelkę (konsola) z listą wszystkich operacji oraz podsumowaniem / bilansem końcowym. Aplikacja powinna utrwalać dane wprowadzone przez użytkownika w pliku tekstowym

5. Zaimplementuj następujące polecenia systemowe w Rust:
- echo - drukuje tekst podany jako argument na standardowym wyjściu
- cat - drukuje zawartość wskazanych plików na standardowym wyjściu, pozwala na opcjonalne numerowanie linii, numerowanie linii może być wyłączone dla linii pustych
- wc - drukuje ilość bytów, znaków, słów i linii dla wskazanych plików
- find - wyszukuje i drukuje ścieżki plików i/lub katalogów, których nazwy są zgodne ze wskazanymi wzorcami
- grep - wyszukuje i drukuje linie zawierające wskazany tekst/pattern ze wskazanych plików/ścieżek

6. Napisz prostą bazę danych, przechowującą dane w postaci binarnej z wykorzystaniem swobodnego dostępu. Przykład odczytu danych
   z określonej pozycji w  pliku znajduje się poniżej. Na podobnej zasadzie należy zrealizować zapis.
   Baza powinna umożliwiać następujące operacje: odczy rekordu po id, aktualizacja rekordu, usunięcie rekordu, dodanie rekordu.
   W celu optymalizacji działania bazy wprowadź indeksowanie pozycji rekordów oraz prosty cache oparty o HashMap.
   Pomyśl nad optymalnym sposobem usuwania rekordów oraz ponownym wykorzystaniem tego obszaru pliku.
   Rekordy powinny mieć stałą długość poszczególnych pól, a ich definicja powinna znajdować się w sekcji nagłówkowej pliku.
   Wystaw funkcjonalność stworzonej bazy za pomocą rest api, wykorzystując jeden z poznanych frameworków (actix, rocket)

id:i64
first_name: string
last_name: string
is_active: bool
age: u8

Odczy danych z określonej pozycji pliki
```
let start = 10;
let count = 10;

let mut f = File::open("/etc/passwd")?;
f.seek(SeekFrom::Start(start))?;
let mut buf = vec![0; count];
f.read_exact(&mut buf)?;
```

Przykład konwersji bytes na typ całkowity i odwrotnie

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

Konwersja bytes na String
```
let bytes = vec![0x41, 0x42, 0x43];
let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
println!("{}", s);
```

https://docs.rust-embedded.org/book/start/hardware.html
