/*
Cvičení 03 — Úloha 3.7
Název: Počet dní po datu
Body: 4 body

Zadání:
Napište funkci, která k zadanému datu (pomocí struktury) a počtu dní určí datum, které po zadaném
datu nastane za zadaný počet dní. Funkce zohledňuje přestupné roky. [4 body]


Správnost programu si můžete ověřit na online datumové kalkulačce

http://www.timeanddate.fasterreader.eu/pages/cs/date-after-days-calc-cs.html


Ukázka:

d.den = 16; d.mesic = 7; d.rok = 2015;

d1 = NasledneDatum(d, 500); // 27.11.2016

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

// Проверяем, високосный ли год.
//
// Правила:
// - делится на 400 -> високосный
// - иначе делится на 100 -> НЕ високосный
// - иначе делится на 4 -> високосный
// - иначе обычный
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
    // Месяцы с 31 днём.
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

    // Месяцы с 30 днями.
    if mesic == 4 || mesic == 6 || mesic == 9 || mesic == 11 {
        return 30;
    }

    // Остался февраль.
    if je_prestupny_rok(rok) == true {
        return 29;
    }

    28
}

// Добавляем к дате заданное количество дней.
fn nasledne_datum(mut datum: Datum, mut pocet_dnu: i32) -> Datum {
    // Пока ещё остались дни, которые надо добавить.
    while pocet_dnu > 0 {
        // Узнаём, сколько дней в текущем месяце.
        let dnu_v_mesici = pocet_dnu_v_mesici(datum.mesic, datum.rok);

        // Сколько дней осталось до конца текущего месяца.
        //
        // Например:
        // дата 16.7.
        // в июле 31 день
        //
        // 31 - 16 = 15 дней до конца месяца.
        let zbyva_do_konce_mesice = dnu_v_mesici - datum.den;

        // Если нужное количество дней помещается
        // в текущий месяц, просто увеличиваем день.
        if pocet_dnu <= zbyva_do_konce_mesice {
            datum.den += pocet_dnu;
            pocet_dnu = 0;
        } else {
            // Иначе переходим на первый день
            // следующего месяца.
            //
            // +1 потому что:
            //
            // 31.7. -> 1.8.
            //
            // Это тоже один день.
            pocet_dnu -= zbyva_do_konce_mesice + 1;

            datum.den = 1;
            datum.mesic += 1;

            // Если месяц стал 13,
            // значит переходим на январь следующего года.
            if datum.mesic > 12 {
                datum.mesic = 1;
                datum.rok += 1;
            }
        }
    }

    datum
}

fn main() {
    let d = Datum {
        den: 16,
        mesic: 7,
        rok: 2015,
    };

    let pocet_dnu = 500;

    let d1 = nasledne_datum(d, pocet_dnu);

    println!("Puvodni datum:   {:02}.{:02}.{}", d.den, d.mesic, d.rok);
    println!("Pocet dni:       {}", pocet_dnu);
    println!("Nasledne datum:  {:02}.{:02}.{}", d1.den, d1.mesic, d1.rok);
}
