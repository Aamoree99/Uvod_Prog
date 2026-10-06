/*
Cvičení 03 — Úloha 3.8
Název: Den v týdnu
Body: 4 body

Zadání:
Napište funkci, která k zadanému datu (pomocí struktury) určí den v týdnu (výčtový typ). Funkce
zohledňuje přestupné roky.  [4 body]


Správnost programu si můžete ověřit na online datumové kalkulačce

http://www.timeanddate.fasterreader.eu/pages/cs/day-of-week-cs.html


Ukázka:

d.den = 15; d.mesic = 3; d.rok = 1966;

den = DenVTydnu(d); // Utery

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

#[derive(Clone, Copy)]
struct Datum {
    den: i32,
    mesic: i32,
    rok: i32,
}

// День недели.
#[derive(Clone, Copy)]
enum DenVTydnu {
    Pondeli,
    Utery,
    Streda,
    Ctvrtek,
    Patek,
    Sobota,
    Nedele,
}

// Проверяем високосный год.
fn je_prestupny_rok(rok: i32) -> bool {
    if rok % 400 == 0 {
        return true;
    }

    if rok % 100 == 0 {
        return false;
    }

    if rok % 4 == 0 {
        return true;
    }

    false
}

// Возвращаем количество дней в месяце.
fn pocet_dnu_v_mesici(mesic: i32, rok: i32) -> i32 {
    if mesic == 1
        || mesic == 3
        || mesic == 5
        || mesic == 7
        || mesic == 8
        || mesic == 10
        || mesic == 12
    {
        return 31;
    }

    if mesic == 4 || mesic == 6 || mesic == 9 || mesic == 11 {
        return 30;
    }

    // Остался февраль.
    if je_prestupny_rok(rok) == true {
        return 29;
    }

    28
}

// Определяем день недели.
fn den_v_tydnu(datum: Datum) -> DenVTydnu {
    let mut pocet_dnu = 0;

    // Сначала считаем все дни полных лет,
    // начиная с 1900 года.
    let mut rok = 1900;

    while rok < datum.rok {
        if je_prestupny_rok(rok) == true {
            pocet_dnu += 366;
        } else {
            pocet_dnu += 365;
        }

        rok += 1;
    }

    // Теперь считаем дни полных месяцев
    // текущего года.
    let mut mesic = 1;

    while mesic < datum.mesic {
        pocet_dnu += pocet_dnu_v_mesici(mesic, datum.rok);
        mesic += 1;
    }

    // Добавляем дни текущего месяца.
    //
    // Например, если дата 15 марта,
    // то до неё прошло 14 полных дней марта.
    pocet_dnu += datum.den - 1;

    // 1.1.1900 был понедельник.
    //
    // Остаток от деления на 7 показывает,
    // насколько дней мы сдвинулись от понедельника.
    match pocet_dnu % 7 {
        0 => DenVTydnu::Pondeli,
        1 => DenVTydnu::Utery,
        2 => DenVTydnu::Streda,
        3 => DenVTydnu::Ctvrtek,
        4 => DenVTydnu::Patek,
        5 => DenVTydnu::Sobota,
        _ => DenVTydnu::Nedele,
    }
}

// Переводим enum в текст.
fn den_na_text(den: DenVTydnu) -> &'static str {
    match den {
        DenVTydnu::Pondeli => "Pondeli",
        DenVTydnu::Utery => "Utery",
        DenVTydnu::Streda => "Streda",
        DenVTydnu::Ctvrtek => "Ctvrtek",
        DenVTydnu::Patek => "Patek",
        DenVTydnu::Sobota => "Sobota",
        DenVTydnu::Nedele => "Nedele",
    }
}

fn main() {
    let d = Datum {
        den: 6,
        mesic: 10,
        rok: 2026,
    };

    let den = den_v_tydnu(d);

    println!("Datum:      {:02}.{:02}.{}", d.den, d.mesic, d.rok);
    println!("Den tydne:  {}", den_na_text(den));
}
