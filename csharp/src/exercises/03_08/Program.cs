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

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // Дата хранит день, месяц и год.
    struct Date {
        public int day; public int month; public int year;
        public Date(int day, int month, int year) { this.day=day; this.month=month; this.year=year; }
    }
    // Год високосный при делимости на 400 либо на 4, но не на 100.
    static bool IsLeapYear(int year) {
        if (year % 400 == 0) return true;
        if (year % 100 == 0) return false;
        if (year % 4 == 0) return true;
        return false;
    }
    // Февраль зависит от года; остальные месяцы имеют фиксированную длину.
    static int DaysInMonth(int month, int year) {
        if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) return 31;
        if (month == 4 || month == 6 || month == 9 || month == 11) return 30;
        return IsLeapYear(year) ? 29 : 28;
    }
    enum DayOfWeek { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday }
    // Как в Rust-решении, дата предполагается корректной и не раньше 1900 года.
    static DayOfWeek GetDayOfWeek(Date date) {
        int dayCount=0, year=1900;
        while (year < date.year) { dayCount += IsLeapYear(year) ? 366 : 365; year += 1; }
        int month=1;
        while (month < date.month) { dayCount += DaysInMonth(month,date.year); month += 1; }
        // До первого числа месяца прошло ноль полных дней этого месяца.
        dayCount += date.day - 1;
        // 1 января 1900 года — понедельник; остаток даёт смещение по неделе.
        int day=dayCount % 7;
        if (day==0) return DayOfWeek.Monday;
        if (day==1) return DayOfWeek.Tuesday;
        if (day==2) return DayOfWeek.Wednesday;
        if (day==3) return DayOfWeek.Thursday;
        if (day==4) return DayOfWeek.Friday;
        if (day==5) return DayOfWeek.Saturday;
        return DayOfWeek.Sunday;
    }
    // Возвращаем английское название переданного дня недели.
    static string DayText(DayOfWeek day) {
        if (day==DayOfWeek.Monday) return "Monday";
        if (day==DayOfWeek.Tuesday) return "Tuesday";
        if (day==DayOfWeek.Wednesday) return "Wednesday";
        if (day==DayOfWeek.Thursday) return "Thursday";
        if (day==DayOfWeek.Friday) return "Friday";
        if (day==DayOfWeek.Saturday) return "Saturday";
        return "Sunday";
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Date d=new Date(6,10,2026);
        DayOfWeek day=GetDayOfWeek(d);
        Console.WriteLine("Date:      {0:D2}.{1:D2}.{2}", d.day, d.month, d.year);
        Console.WriteLine("Day of week:  {0}", DayText(day));
    }
}
