"""
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
"""

def is_leap_year(year):
    return year % 400 == 0 or (year % 100 != 0 and year % 4 == 0)


def days_in_month(month, year):
    if month in (1, 3, 5, 7, 8, 10, 12):
        return 31
    if month in (4, 6, 9, 11):
        return 30
    return 29 if is_leap_year(year) else 28


def date_after_days(date, day_count):
    day, month, year = date
    while day_count > 0:
        remaining = days_in_month(month, year) - day
        if day_count <= remaining:
            day += day_count
            day_count = 0
        else:
            day_count -= remaining + 1
            day = 1
            month += 1
            if month > 12:
                month = 1
                year += 1
    return day, month, year


def main():
    date = (16, 7, 2015)
    day, month, year = date_after_days(date, 500)
    print("Original date: 16.07.2015")
    print("Number of days: 500")
    print(f"Date after days: {day:02}.{month:02}.{year}")

if __name__ == "__main__":
    main()
