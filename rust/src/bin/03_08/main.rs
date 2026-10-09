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
struct Date {
    day: i32,
    month: i32,
    year: i32,
}

// День недели.
#[derive(Clone, Copy)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Проверяем високосный год.
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

    if month == 4 || month == 6 || month == 9 || month == 11 {
        return 30;
    }

    // Остался февраль.
    if is_leap_year(year) == true {
        return 29;
    }

    28
}

// Определяем день недели.
fn day_of_week(date: Date) -> DayOfWeek {
    let mut day_count = 0;

    // Сначала считаем все дни полных лет,
    // начиная с 1900 года.
    let mut year = 1900;

    while year < date.year {
        if is_leap_year(year) == true {
            day_count += 366;
        } else {
            day_count += 365;
        }

        year += 1;
    }

    // Теперь считаем дни полных месяцев
    // текущего года.
    let mut month = 1;

    while month < date.month {
        day_count += days_in_month(month, date.year);
        month += 1;
    }

    // Добавляем дни текущего месяца.
    //
    // Например, если дата 15 марта,
    // то до неё прошло 14 полных дней марта.
    day_count += date.day - 1;

    // 1.1.1900 был понедельник.
    //
    // Остаток от деления на 7 показывает,
    // насколько дней мы сдвинулись от понедельника.
    match day_count % 7 {
        0 => DayOfWeek::Monday,
        1 => DayOfWeek::Tuesday,
        2 => DayOfWeek::Wednesday,
        3 => DayOfWeek::Thursday,
        4 => DayOfWeek::Friday,
        5 => DayOfWeek::Saturday,
        _ => DayOfWeek::Sunday,
    }
}

// Переводим enum в текст.
fn day_text(day: DayOfWeek) -> &'static str {
    match day {
        DayOfWeek::Monday => "Monday",
        DayOfWeek::Tuesday => "Tuesday",
        DayOfWeek::Wednesday => "Wednesday",
        DayOfWeek::Thursday => "Thursday",
        DayOfWeek::Friday => "Friday",
        DayOfWeek::Saturday => "Saturday",
        DayOfWeek::Sunday => "Sunday",
    }
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let d = Date {
        day: 6,
        month: 10,
        year: 2026,
    };

    let day = day_of_week(d);

    println!("Date:      {:02}.{:02}.{}", d.day, d.month, d.year);
    println!("Day of week:  {}", day_text(day));
}
