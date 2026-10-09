/*
Cvičení 02 — Úloha 2.2
Název: Tři čísla vzestupně
Body: 3 body

Zadání:
Uživatel zadá tři čísla z klávesnice. Setřiďte tato číslo ve vnitřní paměti. Vypište tato čísla
setříděná vzestupně dle jejich hodnot. [3 body]

Ukázka:

Setridime tri cisla vzestupne

Zadejte prvni cislo: 33

Zadejte druhe cislo: 6

Zadejte treti cislo: 14

Setridena cisla jsou 6, 14, 33

Požadavky / omezení:
a) K řešení příkladů je možné používat pouze syntaktických konstrukcí a funkcí probraných na
přednáškách. K řešení cvičení s nižším číslem nelze používat učivo hodin s vyšším číslem.

b) Řešení musí být procedurální, nesmí být objektové. Je zakázáno použití cyklu foreach. Je
zakázáno použití kolekcí (Dictionary, List, LinkedList, SortedList, atd.) a jejich obdoby v jiných
programovacích jazycích.

c) Příklady můžete řešit i v jiném jazyku než C#, pokud tento jazyk umožňuje procedurální
programování, např. C, C++, Java. Při této volbě zdůrazňuji platnost pravidla 2a, lze používat
pouze ekvivalenty syntaktických konstrukcí a funkcí probraných na přednášce.

d) Úloha musí být funkční. Otestujte ji minimálně na datech uvedených v zadání příkladu.

e) Zdrojové kódy musí být formátovány jednotným způsobem, zejména odsazování.

f) Proměnné a funkce musejí mít vhodné názvy

g) Zdrojové kódy musí být vhodně členěny do funkcí (platí pro 3. cvičení a následující)

h) Pro každé cvičení vytvořte vlastní Solution (Rešení). Jednotlivé příklady budou jako samostatné
Projekty.

i) Je zakázáno používat složené příkazy { }, pokud obsahují pouze jeden příkaz.
*/

// Вводим три целых числа, каждое на своей строке; unwrap предполагает корректный ввод.
use std::io;

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    println!("Sort three numbers in ascending order");

    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a: i32 = input.trim().parse().unwrap();

    println!("Enter the second number:");
    // read_line дописывает текст, поэтому перед следующим числом очищаем input.
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut b: i32 = input.trim().parse().unwrap();

    println!("Enter the third number:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut c: i32 = input.trim().parse().unwrap();

    // Три сравнения с обменом упорядочивают числа: сначала a и b, затем b и c,
    // потом снова a и b, потому что второй обмен мог нарушить их порядок.
    // В saved_number сохраняем заменяемое значение, чтобы не потерять его.
    if a > b {
        let saved_number = a;
        a = b;
        b = saved_number;
    }

    if b > c {
        let saved_number = b;
        b = c;
        c = saved_number;
    }

    if a > b {
        let saved_number = a;
        a = b;
        b = saved_number;
    }

    println!("The sorted numbers are {a}, {b}, {c}");
}
