# Kompletny przewodnik po makrach w Rust

Makra w języku Rust stanowią zaawansowany mechanizm metaprogramowania, umożliwiający generowanie oraz modyfikowanie kodu na etapie kompilacji. Pozwalają one unikać powielania kodu, tworzyć zwięzłe i czytelne interfejsy API, a także implementować wzorce projektowe w sposób deklaratywny. Makra definiowane są zazwyczaj przy użyciu konstrukcji `macro_rules!` lub jako proceduralne rozszerzenia kompilatora, operujące na strumieniach tokenów (`TokenStream`). W przeciwieństwie do funkcji, które są wykonywane w czasie działania programu, makra są rozwijane podczas kompilacji, co eliminuje narzut związany z wywołaniem funkcji.

Makra w Rust wyróżniają się na tle podobnych mechanizmów znanych z innych języków programowania, takich jak C/C++. W odróżnieniu od makr preprocesora, makra w Rust są higieniczne i typowane. Higiena makr zapewnia brak konfliktów nazw poprzez izolację przestrzeni identyfikatorów, natomiast typowanie pozwala zachować zgodność ze składnią języka. W przeciwieństwie do makr w Lispie, operujących na listach symboli, makra w Rust działają na tokenach składniowych, co umożliwia precyzyjne dopasowanie wzorców i kontrolę nad strukturą generowanego kodu.

W języku Rust makra dzielą się na dwie główne kategorie:

1. **Makra deklaratywne** (`macro_rules!`) – definiowane przy użyciu wzorców oraz odpowiadających im szablonów rozwijania.
2. **Makra proceduralne**, które występują w trzech formach:
    * **Makra `derive`** – automatycznie generujące implementacje cech (`trait`) na podstawie adnotacji `#[derive(...)]`.
    * **Makra atrybutowe** – przypisywane do elementów kodu za pomocą atrybutów, np. `#[route("/")]` w frameworkach webowych.
    * **Makra funkcyjne** – wywoływane jak funkcje, np. `html! { ... }`, wykorzystywane do tworzenia wewnętrznych DSL-i oraz zaawansowanych transformacji kodu.

## Wprowadzenie do makr

Makra w Rust operują na poziomie składniowym, transformując kod przed kompilacją. W przeciwieństwie do funkcji, makra mogą:
- Przyjmować zmienną liczbę argumentów
- Generować kod na podstawie wzorców wejściowych
- Pracować z różnymi typami bez ograniczeń typów generycznych
- Znacząco redukować duplikację kodu

## Makra deklaratywne

Makra deklaratywne są definiowane przy użyciu składni `macro_rules!` i działają poprzez dopasowywanie wzorców do tokenów wejściowych.

### Podstawowa składnia

```rust
macro_rules! nazwa_makra {
    (wzorzec) => {
        rozwinięcie
    };
}
```

### Prosty przykład: Tworzenie podstawowego makra drukującego

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!(); // Rozwija się do: println!("Hello, world!");
}
```

**Szczegółowe wyjaśnienie:**
- `macro_rules!` deklaruje makro deklaratywne
- `say_hello` to nazwa makra
- `()` to dopasowywacz wzorca - pasuje, gdy nie podano żadnych argumentów
- `=>` oddziela wzorzec od rozwinięcia
- Kod rozwinięcia `println!("Hello, world!");` jest wstawiany wszędzie tam, gdzie wywoływane jest `say_hello!()`
- Makro jest wywoływane za pomocą `say_hello!()` - zwróć uwagę na znak `!`, który wskazuje wywołanie makra

**Proces kompilacji:**
1. Gdy kompilator napotyka `say_hello!()`, szuka pasującego wzorca
2. Znajduje wzorzec `()` i zastępuje wywołanie makra przez `println!("Hello, world!");`
3. Wynikowy kod jest następnie normalnie kompilowany

### Makro z parametrami

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("Alice");    // Hello, Alice!
    greet!("Bob");      // Hello, Bob!
}
```

**Szczegółowe wyjaśnienie:**
- `$name:expr` przechwytuje wyrażenie i wiąże je ze zmienną `$name`
- `:expr` to specyfikator fragmentu, który pasuje do dowolnego poprawnego wyrażenia Rust
- `$name` w rozwinięciu odnosi się do przechwyconego wyrażenia
- Gdy wywołujesz `greet!("Alice")`, łańcuch znaków `"Alice"` jest przechwytywany jako `$name`
- Rozwinięcie staje się `println!("Hello, {}!", "Alice");`

**Specyfikatory fragmentów:**
- `expr` - wyrażenia jak `5 + 3`, `"hello"`, `my_var`
- `stmt` - instrukcje jak `let x = 5;`
- `ident` - identyfikatory jak `my_variable`
- `ty` - typy jak `i32`, `String`
- `pat` - wzorce jak `Some(x)`, `(a, b)`

## Dopasowywanie wzorców w makrach

Makra używają specyfikatorów fragmentów do dopasowywania różnych typów elementów składni:

### Wiele wzorców

```rust
macro_rules! calculate {
    ($a:expr + $b:expr) => {
        $a + $b
    };
    ($a:expr - $b:expr) => {
        $a - $b
    };
    ($a:expr * $b:expr) => {
        $a * $b
    };
    ($a:expr / $b:expr) => {
        $a / $b
    };
}

fn main() {
    println!("{}", calculate!(5 + 3));  // 8
    println!("{}", calculate!(10 - 4)); // 6
    println!("{}", calculate!(6 * 7));  // 42
    println!("{}", calculate!(15 / 3)); // 5
}
```

**Szczegółowe wyjaśnienie:**
- To makro ma wiele ramion (wzorców), z których każde obsługuje różne operacje
- Kompilator próbuje dopasować wzorce od góry do dołu
- `$a:expr + $b:expr` dopasowuje wyrażenia oddzielone tokenem `+`
- Literały `+`, `-`, `*`, `/` muszą pasować dokładnie
- Każde ramię przechwytuje dwa wyrażenia i wykonuje odpowiednią operację

**Proces dopasowywania wzorców:**
1. `calculate!(5 + 3)` pasuje do pierwszego ramienia
2. `$a` przechwytuje `5`, `$b` przechwytuje `3`
3. Rozwinięcie staje się `5 + 3`
4. Jeśli żaden wzorzec nie pasuje, otrzymujesz błąd w czasie kompilacji

### Powtórzenia

Używaj `$(...)*` dla zero lub więcej powtórzeń, `$(...)+` dla jednego lub więcej:

```rust
macro_rules! vec_create {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec_create![1, 2, 3, 4, 5];
    println!("{:?}", v); // [1, 2, 3, 4, 5]
}
```

**Szczegółowe wyjaśnienie:**
- `$($x:expr),*` oznacza "dopasuj zero lub więcej wyrażeń oddzielonych przecinkami"
- Zewnętrzny `$()` grupuje wzorzec powtórzenia
- `,*` oznacza "oddzielone przecinkami, zero lub więcej razy"
- W rozwinięciu, `$(...)*` powtarza kod wewnątrz dla każdego dopasowanego elementu
- Dodatkowe `{}` tworzy wyrażenie blokowe, które zwraca wektor

**Krok po kroku dla `vec_create![1, 2, 3]`:**
1. Wzorzec dopasowuje z `$x` przechwytującym `1`, potem `2`, potem `3`
2. Rozwinięcie staje się:
```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

### Przykład zaawansowanych powtórzeń: Makro tworzące HashMap

```rust
macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn main() {
    let map = hash_map!{
        "name" => "Alice",
        "age" => "30",
        "city" => "New York"
    };
    println!("{:?}", map);
}
```

**Szczegółowe wyjaśnienie:**
- `$($key:expr => $val:expr),*` dopasowuje pary klucz-wartość oddzielone przecinkami
- Token `=>` jest literałem i musi pojawić się między kluczem a wartością
- Każde powtórzenie przechwytuje zarówno klucz, jak i wartość
- Rozwinięcie wstawia każdą parę klucz-wartość do HashMap

**Dla `hash_map!{"name" => "Alice", "age" => "30"}`:**
1. Pierwsza iteracja: `$key` = `"name"`, `$val` = `"Alice"`
2. Druga iteracja: `$key` = `"age"`, `$val` = `"30"`
3. Generuje dwa wywołania `map.insert()`

## Zaawansowane techniki makr deklaratywnych

### Makra rekurencyjne: Znajdowanie minimum

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(5, 3, 8, 1, 9, 2)); // 1
}
```

**Szczegółowe wyjaśnienie:**
- Pierwsze ramię: przypadek bazowy z pojedynczym wyrażeniem
- Drugie ramię: przypadek rekurencyjny z jednym wyrażeniem, po którym następuje jedno lub więcej innych
- `$($y:expr),+` przechwytuje pozostałe wyrażenia (jedno lub więcej)
- Makro wywołuje samo siebie rekurencyjnie z pozostałymi wyrażeniami
- Używa `std::cmp::min` do porównania pierwszego elementu z minimum reszty

**Śledzenie rekurencji dla `find_min!(5, 3, 8, 1)`:**
1. `find_min!(5, 3, 8, 1)` → `min(5, find_min!(3, 8, 1))`
2. `find_min!(3, 8, 1)` → `min(3, find_min!(8, 1))`
3. `find_min!(8, 1)` → `min(8, find_min!(1))`
4. `find_min!(1)` → `1` (przypadek bazowy)
5. Wynik: `min(5, min(3, min(8, 1)))` = `1`

### Makro liczące

```rust
macro_rules! count {
    () => (0usize);
    ($head:tt $($tail:tt)*) => (1usize + count!($($tail)*));
}

macro_rules! array_len {
    ($($items:expr),*) => {
        {
            const LEN: usize = count!($($items)*);
            let array: [_; LEN] = [$($items),*];
            (array, LEN)
        }
    };
}

fn main() {
    let (arr, len) = array_len![1, 2, 3, 4, 5];
    println!("Array: {:?}, Length: {}", arr, len);
}
```

**Szczegółowe wyjaśnienie:**
- `count!` używa drzew tokenów (`tt`) do liczenia dowolnych tokenów
- `$head:tt` przechwytuje pierwszy token, `$($tail:tt)*` przechwytuje resztę
- Każde rekurencyjne wywołanie dodaje 1 i przetwarza pozostałe tokeny
- `array_len!` używa `count!` do określenia długości tablicy w czasie kompilacji
- `const LEN: usize` tworzy stałą czasu kompilacji
- `[_; LEN]` tworzy tablicę o obliczonej długości

**Liczenie tokenów dla `count!(a b c)`:**
1. `count!(a b c)` → `1 + count!(b c)`
2. `count!(b c)` → `1 + count!(c)`
3. `count!(c)` → `1 + count!()`
4. `count!()` → `0`
5. Wynik: `1 + 1 + 1 + 0 = 3`

### Higiena makr i przechwytywanie zmiennych

```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}

fn main() {
    let four = using_a!(a / 10); // To działa - 'a' jest przechwycone z makra
    println!("{}", four); // 4
    
    // To spowodowałoby błąd kompilacji:
    // let a = 13;
    // let b = using_a!(a); // 'a' odnosi się do 'a' z makra, nie do zewnętrznego 'a'
}
```

***Szczegółowe wyjaśnienie:***

- Rust implementuje higienę makr - mechanizm zapobiegający przypadkowym kolizjom nazw
- Zmienna `a` zdefiniowana wewnątrz makra jest dostępna tylko w kontekście tego makra
- Gdy używasz a w wyrażeniu przekazanym do makra, odnosi się ono do `a` z makra, nie do zewnętrznych zmiennych o tej samej nazwie.
To oznacza, że makra nie mogą przypadkowo "przechwycić" zmiennych z otaczającego kodu. Gdyby istniała zewnętrzna zmienna `a = 13`, makro nadal używałoby swojego własnego `a = 42`

***Dlaczego to ważne:***
- Zapobiega trudnym do znalezienia błędom związanym z zasięgiem zmiennych
- Makra są przewidywalne - zawsze używają zmiennych zdefiniowanych w swoim wnętrzu
- Kod wywołujący makro nie może przypadkowo wpłynąć na jego wewnętrzne zmienne

### Tworzenie DSL za pomocą makr

```rust
macro_rules! sql_select {
    (SELECT $($field:ident),+ FROM $table:ident WHERE $condition:expr) => {
        format!(
            "SELECT {} FROM {} WHERE {}",
            format!("{}", stringify!($($field),+)).replace(" ", ", ")
            stringify!($table),
            $condition
        )
    };
}

fn main() {
    let query = sql_select!(
        SELECT name, age, email 
        FROM users 
        WHERE "age > 18"
    );
    println!("{}", query);
    // Wynik: SELECT name, age, email FROM users WHERE age > 18
}
```

**Szczegółowe wyjaśnienie:**
- Tworzy język dziedzinowy (DSL) dla składni podobnej do SQL
- `$($field:ident),+` przechwytuje jeden lub więcej identyfikatorów pól
- `$table:ident` przechwytuje nazwę tabeli jako identyfikator
- `$condition:expr` przechwytuje warunek WHERE jako wyrażenie
- `stringify!()` konwertuje tokeny na literały łańcuchów znaków
- `.replace(" ", ", ")` naprawia formatowanie listy pól

**Dla przykładowego użycia:**
```rust
sql_select!(SELECT name, age, email FROM users WHERE "age > 18");
```
1. `$field` przechwytuje `name`, `age`, `email`
2. `$table` przechwytuje `users`
3. `$condition` przechwytuje `"age > 18"`
4. Wynik: `"SELECT name, age, email FROM users WHERE age > 18"`

## Makra proceduralne

Makra proceduralne są potężniejsze i bardziej elastyczne niż makra deklaratywne. Operują na strumieniach tokenów i mogą wykonywać dowolne obliczenia.

### Konfiguracja dla makr proceduralnych

Najpierw utwórz nową bibliotekę crate:

```toml
# Cargo.toml
[package]
name = "my_macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
```

## Makra proceduralne funkcyjne

```rust
// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let expanded = quote! {
        format!("The answer to '{}' is 42", #input)
    };
    TokenStream::from(expanded)
}
```

**Szczegółowe wyjaśnienie:**
- `#[proc_macro]` oznacza to jako funkcyjne makro proceduralne
- `TokenStream` reprezentuje tokeny wejściowe i wyjściowe
- `parse_macro_input!` parsuje wejście jako określony typ (`LitStr` = literał łańcuchowy)
- `quote!` to makro, które generuje kod Rust
- `#input` interpoluje sparsowane wejście do generowanego kodu
- Zwraca `TokenStream`, który zastępuje wywołanie makra

**Kroki przetwarzania:**
1. Wejście: `make_answer!("What is life?")`
2. `input` staje się literałem łańcuchowym `"What is life?"`
3. `quote!` generuje: `format!("The answer to '{}' is 42", "What is life?")`
4. Ten kod zastępuje wywołanie makra

Użycie:
```rust
use my_macros::make_answer;

fn main() {
    let answer = make_answer!("What is the meaning of life?");
    println!("{}", answer);
}
```

### Bardziej złożone makro funkcyjne: Operacje matematyczne

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Token};
use syn::parse::{Parse, ParseStream};

struct MathOp {
    left: Expr,
    op: String,
    right: Expr,
}

impl Parse for MathOp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let left: Expr = input.parse()?;
        let op_token: syn::Ident = input.parse()?;
        let right: Expr = input.parse()?;
        
        Ok(MathOp {
            left,
            op: op_token.to_string(),
            right,
        })
    }
}

#[proc_macro]
pub fn math_op(input: TokenStream) -> TokenStream {
    let MathOp { left, op, right } = parse_macro_input!(input as MathOp);
    
    let result = match op.as_str() {
        "add" => quote! { #left + #right },
        "sub" => quote! { #left - #right },
        "mul" => quote! { #left * #right },
        "div" => quote! { #left / #right },
        _ => panic!("Unsupported operation: {}", op),
    };
    
    TokenStream::from(result)
}
```

**Szczegółowe wyjaśnienie:**
- Tworzy niestandardową strukturę parsowania `MathOp`
- Implementacja cechy `Parse` definiuje, jak parsować wejście
- `ParseStream` dostarcza metody do parsowania różnych typów tokenów
- `input.parse()?` próbuje sparsować następne tokeny jako określony typ
- Makro dopasowuje operację i generuje odpowiedni kod
- `#left` i `#right` interpolują sparsowane wyrażenia

**Dla `math_op!(10 add 5)`:**
1. Parser wyodrębnia: `left = 10`, `op = "add"`, `right = 5`
2. Dopasowanie do "add" generuje: `quote! { 10 + 5 }`
3. Wynik końcowy: `10 + 5`

Użycie:
```rust
use my_macros::math_op;

fn main() {
    let result = math_op!(10 add 5);
    println!("{}", result); // 15
}
```

## Makra derive

Makra derive automatycznie implementują cechy (traits) dla struktur i enumów.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let description = match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields) => {
                    let field_names: Vec<_> = fields.named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap().to_string())
                        .collect();
                    format!("Struct {} with fields: {}", name, field_names.join(", "))
                }
                Fields::Unnamed(_) => format!("Tuple struct {}", name),
                Fields::Unit => format!("Unit struct {}", name),
            }
        }
        Data::Enum(_) => format!("Enum {}", name),
        Data::Union(_) => format!("Union {}", name),
    };
    
    let expanded = quote! {
        impl #name {
            pub fn describe() -> &'static str {
                #description
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Szczegółowe wyjaśnienie:**
- `#[proc_macro_derive(Describe)]` tworzy makro derive o nazwie `Describe`
- `DeriveInput` reprezentuje strukturę/enum/union, na której działa derive
- `input.ident` pobiera nazwę typu
- `input.data` zawiera informacje o strukturze
- Dopasowanie wzorca na enum `Data` obsługuje różne rodzaje typów
- Dla struktur, dalsze dopasowanie na `Fields` obsługuje różne typy pól
- `fields.named.iter()` iteruje po nazwanych polach, aby wyodrębnić nazwy pól
- Generuje blok `impl` z metodą `describe()`

**Dla struktury jak:**
```rust
#[derive(Describe)]
struct Person {
    name: String,
    age: u32,
}
```

**Wygenerowany kod:**
```rust
impl Person {
    pub fn describe() -> &'static str {
        "Struct Person with fields: name, age"
    }
}
```

Użycie:
```rust
use my_macros::Describe;

#[derive(Describe)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Describe)]
enum Status {
    Active,
    Inactive,
}

fn main() {
    println!("{}", Person::describe());
    println!("{}", Status::describe());
}
```

### Zaawansowane makro derive z atrybutami: Wzorzec Builder

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Attribute, Meta, Lit};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());
    
    let fields = match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields) => fields.named,
                _ => panic!("Builder only supports named fields"),
            }
        }
        _ => panic!("Builder only supports structs"),
    };
    
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });
    
    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.ok_or(concat!("Field ", stringify!(#name), " is required"))?
        }
    });
    
    let builder_init = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: None }
    });
    
    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields,)*
        }
        
        impl #builder_name {
            #(#builder_methods)*
            
            pub fn build(self) -> Result<#name, &'static str> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
        
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init,)*
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Szczegółowe wyjaśnienie:**
- Automatycznie tworzy wzorzec buildera dla struktur
- `syn::Ident::new()` tworzy nowy identyfikator dla nazwy struktury buildera
- `builder_fields` tworzy pola `Option<T>` dla każdego oryginalnego pola
- `builder_methods` tworzy metody ustawiające, które zwracają `Self` dla łańcuchowania
- `build_fields` wyodrębnia wartości z `Option`, zwracając błędy dla brakujących pól
- `builder_init` inicjalizuje wszystkie pola na `None`
- `#(#iterator,)*` rozwija iteratory w listy oddzielone przecinkami

**Dla:**
```rust
#[derive(Builder)]
struct User {
    name: String,
    email: String,
    age: u32,
}
```

**Wygenerowane:**
```rust
pub struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    pub fn build(self) -> Result<User, &'static str> {
        Ok(User {
            name: self.name.ok_or("Field name is required")?,
            email: self.email.ok_or("Field email is required")?,
            age: self.age.ok_or("Field age is required")?,
        })
    }
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder {
            name: None,
            email: None,
            age: None,
        }
    }
}
```

Użycie:
```rust
use my_macros::Builder;

#[derive(Builder)]
struct User {
    name: String,
    email: String,
    age: u32,
}

fn main() -> Result<(), &'static str> {
    let user = User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .build()?;
    
    println!("User: {} ({})", user.name, user.email);
    Ok(())
}
```

## Makra atrybutowe

Makra atrybutowe mogą być stosowane do funkcji, struktur i innych elementów, aby modyfikować ich zachowanie.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timing(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("Function '{}' took: {:?}", #fn_name_str, duration);
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

**Szczegółowe wyjaśnienie:**
- `#[proc_macro_attribute]` tworzy makro atrybutowe
- Przyjmuje dwa parametry: argumenty atrybutu i element, do którego atrybut jest stosowany
- `ItemFn` reprezentuje element funkcji
- Wyodrębnia komponenty funkcji: widoczność, sygnaturę, nazwę i ciało
- Owija oryginalne ciało funkcji kodem pomiaru czasu
- `(|| #fn_block)()` tworzy i natychmiast wywołuje domknięcie z oryginalnym ciałem
- Zachowuje oryginalną sygnaturę funkcji i wartość zwracaną

**Dla:**
```rust
#[timing]
fn slow_function() -> u32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}
```

**Wygenerowany:**
```rust
fn slow_function() -> u32 {
    let start = std::time::Instant::now();
    let result = (|| {
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    })();
    let duration = start.elapsed();
    println!("Function 'slow_function' took: {:?}", duration);
    result
}
```

Użycie:
```rust
use my_macros::timing;

#[timing]
fn slow_function() -> u32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

fn main() {
    let result = slow_function();
    println!("Result: {}", result);
}
```

### Zaawansowane makro atrybutowe z parametrami: Retry

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let max_attempts = args.iter()
        .find_map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
                if nv.path.is_ident("times") {
                    if let Lit::Int(lit_int) = &nv.lit {
                        return lit_int.base10_parse::<usize>().ok();
                    }
                }
            }
            None
        })
        .unwrap_or(3);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            let mut attempts = 0;
            loop {
                attempts += 1;
                match std::panic::catch_unwind(|| #fn_block) {
                    Ok(result) => return result,
                    Err(_) if attempts < #max_attempts => {
                        println!("Attempt {} failed, retrying...", attempts);
                        continue;
                    },
                    Err(e) => std::panic::resume_unwind(e),
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Szczegółowe wyjaśnienie:**
- Parsuje argumenty atrybutu, aby znaleźć liczbę ponownych prób
- `AttributeArgs` reprezentuje sparsowane argumenty atrybutu
- `find_map()` szuka parametru `times = wartość`
- Dopasowanie wzorca wyodrębnia wartość całkowitą z atrybutu
- `unwrap_or(3)` zapewnia domyślną liczbę ponownych prób
- Używa `std::panic::catch_unwind` do przechwytywania panik i ponownych prób
- `std::panic::resume_unwind` ponownie wywołuje panikę po maksymalnej liczbie prób

**Dla:**
```rust
#[retry(times = 5)]
fn unreliable_function() -> Result<String, &'static str> {
    // ciało funkcji
}
```

**Wygenerowany:**
```rust
fn unreliable_function() -> Result<String, &'static str> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        match std::panic::catch_unwind(|| {
            // oryginalne ciało funkcji
        }) {
            Ok(result) => return result,
            Err(_) if attempts < 5 => {
                println!("Attempt {} failed, retrying...", attempts);
                continue;
            },
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}
```

Użycie:
```rust
use my_macros::retry;

#[retry(times = 5)]
fn unreliable_function() -> Result<String, &'static str> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    if count < 3 {
        Err("Function failed")
    } else {
        Ok("Success!".to_string())
    }
}
```

## Najlepsze praktyki i typowe pułapki

### Najlepsze praktyki

1. Używaj makr deklaratywnych do prostego dopasowywania wzorców
2. Używaj makr proceduralnych do złożonego generowania kodu
3. Zawsze używaj `quote!` do generowania kodu w makrach proceduralnych
4. Dokładnie testuj swoje makra
5. Dobrze dokumentuj swoje makra
6. Uwzględniaj higienę makr i przechwytywanie zmiennych

### Typowe pułapki

1. Zapominanie o obsłudze wszystkich przypadków w dopasowywaniu wzorców
2. Nieuwzględnianie kolejności rozwijania makr
3. Tworzenie zbyt złożonych makr, które są trudne do debugowania
4. Nieużywanie właściwej obsługi błędów w makrach proceduralnych

### Testowanie makr

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macro() {
        assert_eq!(calculate!(5 + 3), 8);
        assert_eq!(calculate!(10 - 4), 6);
        assert_eq!(calculate!(6 * 7), 42);
        assert_eq!(calculate!(15 / 3), 5);
    }
    
    #[test]
    fn test_vec_create_macro() {
        let v = vec_create![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }
}
```

## Przykłady z rzeczywistych zastosowań

### DSL konfiguracyjny

```rust
macro_rules! config {
    (
        $(
            $section:ident {
                $(
                    $key:ident = $value:expr
                ),* $(,)?
            }
        )*
    ) => {
        {
            use std::collections::HashMap;
            let mut config = HashMap::new();
            
            $(
                let mut section = HashMap::new();
                $(
                    section.insert(stringify!($key), $value.to_string());
                )*
                config.insert(stringify!($section), section);
            )*
            
            config
        }
    };
}

fn main() {
    let app_config = config! {
        database {
            host = "localhost",
            port = 5432,
            name = "myapp"
        }
        server {
            host = "0.0.0.0",
            port = 8080
        }
    };
    
    println!("{:#?}", app_config);
}
```

**Szczegółowe wyjaśnienie:**
- Tworzy zagnieżdżoną strukturę: sekcje zawierające pary klucz-wartość
- `$(,)?` oznacza opcjonalny końcowy przecinek
- Zewnętrzne powtórzenie `$(...)*` obsługuje wiele sekcji
- Wewnętrzne powtórzenie `$(...)*` obsługuje wiele par klucz-wartość na sekcję
- Tworzy zagnieżdżone `HashMap<String, HashMap<String, String>>`
- `stringify!()` konwertuje identyfikatory na literały łańcuchów znaków
- `.to_string()` konwertuje wartości na łańcuchy znaków do przechowywania

**Podział struktury:**
- `$section:ident` przechwytuje nazwy sekcji jak `database`, `server`
- `$key:ident = $value:expr` przechwytuje pary klucz-wartość
- Zagnieżdżone pętle tworzą zagnieżdżoną strukturę HashMap

### Makro szablonu HTML

```rust
macro_rules! html {
    ($tag:ident { $($content:tt)* }) => {
        format!("<{}>{}</{}>", stringify!($tag), html!($($content)*), stringify!($tag))
    };
    ($tag:ident[$($attr:ident = $val:expr),*] { $($content:tt)* }) => {
        format!(
            "<{} {}>{}</{}>",
            stringify!($tag),
            vec![$(format!("{}=\"{}\"", stringify!($attr), $val)),*].join(" "),
            html!($($content)*),
            stringify!($tag)
        )
    };
    ($text:expr) => {
        $text.to_string()
    };
    () => {
        String::new()
    };
}

fn main() {
    let page = html! {
        html {
            head {
                title { "My Page" }
            }
            body[class = "main"] {
                h1 { "Welcome!" }
                p { "This is generated HTML." }
            }
        }
    };
    
    println!("{}", page);
}
```

**Szczegółowe wyjaśnienie:**
- Wiele wzorców obsługuje różne konstrukcje HTML
- Pierwszy wzorzec: tagi z zawartością `tag { content }`
- Drugi wzorzec: tagi z atrybutami `tag[attr = value] { content }`
- Trzeci wzorzec: zawartość tekstowa (wyrażenia)
- Czwarty wzorzec: pusta zawartość
- Rekurencyjne wywołania obsługują zagnieżdżone struktury
- `vec![...].join(" ")` tworzy łańcuchy atrybutów

**Priorytet dopasowywania wzorców:**
1. Najpierw najbardziej specyficzne wzorce (z atrybutami)
2. Mniej specyficzne wzorce (bez atrybutów)
3. Przypadki bazowe (tekst, puste)

### Makro do benchmarków

```rust
macro_rules! benchmark {
    ($name:expr, $code:block) => {
        {
            let start = std::time::Instant::now();
            let iterations = 1000;
            
            for _ in 0..iterations {
                $code
            }
            
            let duration = start.elapsed();
            let avg_duration = duration / iterations;
            
            println!(
                "Benchmark '{}': {} iterations, total: {:?}, avg: {:?}",
                $name,
                iterations,
                duration,
                avg_duration
            );
        }
    };
}

fn main() {
    benchmark!("Vector creation", {
        let _v: Vec<i32> = (0..100).collect();
    });
    
    benchmark!("HashMap insertion", {
        let mut map = std::collections::HashMap::new();
        for i in 0..100 {
            map.insert(i, i * 2);
        }
    });
}
```

**Szczegółowe wyjaśnienie:**
- `$name:expr` przechwytuje wyrażenie łańcuchowe dla nazwy benchmarku
- `$code:block` przechwytuje blok kodu do zmierzenia
- Uruchamia blok kodu 1000 razy w pętli
- Mierzy całkowity czas i oblicza średnią na iterację
- Wyrażenie blokowe zwraca `()` (typ jednostkowy)
- Makro tworzy nowy zakres za pomocą `{...}`, aby zawrzeć zmienne

**Wzorzec użycia:**
```rust
benchmark!("Vector creation", {
    let _v: Vec<i32> = (0..100).collect();
});
```

## Podsumowanie

Makra w Rust są potężnym narzędziem, które pozwala na metaprogramowanie i generowanie kodu w czasie kompilacji. Dzięki nim możemy:

1. Redukować powtarzalny kod
2. Tworzyć bardziej zwięzłe i czytelne API
3. Implementować wzorce projektowe w sposób deklaratywny
4. Tworzyć języki dziedzinowe (DSL) wewnątrz Rust

Rust oferuje dwa główne rodzaje makr:
- **Makra deklaratywne** - prostsze w użyciu, bazujące na wzorcach
- **Makra proceduralne** - potężniejsze, działające na poziomie tokenów

Pamiętaj, że chociaż makra są bardzo potężne, należy używać ich rozważnie. Zbyt złożone makra mogą być trudne do debugowania i zrozumienia. Zawsze staraj się najpierw rozwiązać problem za pomocą zwykłych funkcji, a po makra sięgaj, gdy rzeczywiście przyniosą znaczące korzyści w czytelności kodu lub eliminacji powtórzeń.

