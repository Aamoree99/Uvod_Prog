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
struct Date {
    day: i32,
    month: i32,
    year: i32,
}

// Проверяем, високосный ли год.
//
// Правила:
// - делится на 400 -> високосный
// - иначе делится на 100 -> НЕ високосный
// - иначе делится на 4 -> високосный
// - иначе обычный
fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }

    if year % 100 == 0 {
        return false;
    }

    if year % 4 == 0 {
        return true;
    }

    false
}

// Возвращаем количество дней в месяце.
fn days_in_month(month: i32, year: i32) -> i32 {
    // Месяцы с 31 днём.
    if month == 1
        || month == 3
        || month == 5
        || month == 7
        || month == 8
        || month == 10
        || month == 12
    {
        return 31;
    }

    // Месяцы с 30 днями.
    if month == 4 || month == 6 || month == 9 || month == 11 {
        return 30;
    }

    // Остался февраль.
    if is_leap_year(year) == true {
        return 29;
    }

    28
}

// Добавляем к дате заданное количество дней.
fn date_after_days(mut date: Date, mut day_count: i32) -> Date {
    // Пока ещё остались дни, которые надо добавить.
    while day_count > 0 {
        // Узнаём, сколько дней в текущем месяце.
        let month_length = days_in_month(date.month, date.year);

        // Сколько дней осталось до конца текущего месяца.
        //
        // Например:
        // дата 16.7.
        // в июле 31 день
        //
        // 31 - 16 = 15 дней до конца месяца.
        let remaining_days = month_length - date.day;

        // Если нужное количество дней помещается
        // в текущий месяц, просто увеличиваем день.
        if day_count <= remaining_days {
            date.day += day_count;
            day_count = 0;
        } else {
            // Иначе переходим на первый день
            // следующего месяца.
            //
            // +1 потому что:
            //
            // 31.7. -> 1.8.
            //
            // Это тоже один день.
            day_count -= remaining_days + 1;

            date.day = 1;
            date.month += 1;

            // Если месяц стал 13,
            // значит переходим на январь следующего года.
            if date.month > 12 {
                date.month = 1;
                date.year += 1;
            }
        }
    }

    date
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let d = Date {
        day: 16,
        month: 7,
        year: 2015,
    };

    let day_count = 500;

    let d1 = date_after_days(d, day_count);

    println!("Original date:   {:02}.{:02}.{}", d.day, d.month, d.year);
    println!("Number of days:       {}", day_count);
    println!(
        "Date after days:  {:02}.{:02}.{}",
        d1.day, d1.month, d1.year
    );
}
