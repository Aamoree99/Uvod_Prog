"""
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
"""

def is_leap_year(year):
    return year % 400 == 0 or (year % 100 != 0 and year % 4 == 0)


def days_in_month(month, year):
    if month in (1, 3, 5, 7, 8, 10, 12):
        return 31
    if month in (4, 6, 9, 11):
        return 30
    return 29 if is_leap_year(year) else 28


def day_of_week(date):
    day, month, year = date
    day_count = 0
    current_year = 1900
    while current_year < year:
        day_count += 366 if is_leap_year(current_year) else 365
        current_year += 1
    # Сохраняем опорную дату, но поддерживаем и годы до 1900.
    while current_year > year:
        current_year -= 1
        day_count -= 366 if is_leap_year(current_year) else 365
    current_month = 1
    while current_month < month:
        day_count += days_in_month(current_month, year)
        current_month += 1
    day_count += day - 1
    return day_count % 7


def main():
    names = ("Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday")
    print("Date: 06.10.2026")
    print(f"Day of week: {names[day_of_week((6, 10, 2026))]}")

if __name__ == "__main__":
    main()
