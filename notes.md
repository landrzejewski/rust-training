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
* 
4. Napisz aplikacje umożliwiającą rejestrowanie wpływów i wydatków dla budżetu domowego. Aplikacja powinna rejestrować typ, rodzaj operacji oraz jej opis (podawane jako argumenty z linii poleceń)
oraz generować raport/tabelkę (konsola) z listą wszystkich operacji oraz podsumowaniem / bilansem końcowym. Aplikacja powinna utrwalać dane wprowadzone przez użytkownika w pliku tekstowym

5. Zaimplementuj następujące polecenia systemowe w Rust:
- echo - drukuje tekst podany jako argument na standardowym wyjściu
- cat - drukuje zawartość wskazanych plików na standardowym wyjściu, pozwala na opcjonalne numerowanie linii, numerowanie linii może być wyłączone dla linii pustych
- wc - drukuje ilość bytów, znaków, słów i linii dla wskazanych plików
- find - wyszukuje i drukuje ścieżki plików i/lub katalogów, których nazwy są zgodne ze wskazanymi wzorcami
- grep - wyszukuje i drukuje linie zawierające wskazany tekst/pattern ze wskazanych plików/ścieżek