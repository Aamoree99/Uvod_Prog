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

package exercises.task_03_07;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    // Дата хранит день, месяц и год.
    static class Date {
        public int day; public int month; public int year;
        public Date(int day, int month, int year) { this.day=day; this.month=month; this.year=year; }
    }
    // Год високосный при делимости на 400 либо на 4, но не на 100.
    static boolean isLeapYear(int year) {
        if (year % 400 == 0) return true;
        if (year % 100 == 0) return false;
        if (year % 4 == 0) return true;
        return false;
    }
    // Февраль зависит от года; остальные месяцы имеют фиксированную длину.
    static int daysInMonth(int month, int year) {
        if (month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12) return 31;
        if (month == 4 || month == 6 || month == 9 || month == 11) return 30;
        return isLeapYear(year) ? 29 : 28;
    }
    // Переносим дату по месяцам, а не перебираем каждый отдельный день.
    static Date dateAfterDays(Date date, int dayCount) {
        // Java передаёт ссылку; копируем дату, чтобы не изменить исходную.
        date=new Date(date.day,date.month,date.year);
        while (dayCount > 0) {
            int remainingDays = daysInMonth(date.month, date.year) - date.day;
            if (dayCount <= remainingDays) { date.day += dayCount; dayCount = 0; }
            else {
                // Переход на первое число следующего месяца тоже занимает один день.
                dayCount -= remainingDays + 1; date.day = 1; date.month += 1;
                if (date.month > 12) { date.month = 1; date.year += 1; }
            }
        }
        return date;
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        Date d=new Date(16,7,2015);
        int dayCount=500;
        Date d1=dateAfterDays(d,dayCount);
        System.out.printf(Locale.ROOT, "Original date:   %02d.%02d.%s%n", d.day, d.month, d.year);
        System.out.printf(Locale.ROOT, "Number of days:       %s%n", dayCount);
        System.out.printf(Locale.ROOT, "Date after days:  %02d.%02d.%s%n", d1.day, d1.month, d1.year);
    }
}
