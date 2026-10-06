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

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // Дата хранит день, месяц и год.
    struct Datum {
        public int den; public int mesic; public int rok;
        public Datum(int den, int mesic, int rok) { this.den=den; this.mesic=mesic; this.rok=rok; }
    }
    // Год високосный при делимости на 400 либо на 4, но не на 100.
    static bool je_prestupny_rok(int rok) {
        if (rok % 400 == 0) return true;
        if (rok % 100 == 0) return false;
        if (rok % 4 == 0) return true;
        return false;
    }
    // Февраль зависит от года; остальные месяцы имеют фиксированную длину.
    static int pocet_dnu_v_mesici(int mesic, int rok) {
        if (mesic == 1 || mesic == 3 || mesic == 5 || mesic == 7 || mesic == 8 || mesic == 10 || mesic == 12) return 31;
        if (mesic == 4 || mesic == 6 || mesic == 9 || mesic == 11) return 30;
        return je_prestupny_rok(rok) ? 29 : 28;
    }
    // Переносим дату по месяцам, а не перебираем каждый отдельный день.
    static Datum nasledne_datum(Datum datum, int pocet_dnu) {
        // Структура передаётся по значению, поэтому изменяем только локальную копию.
        while (pocet_dnu > 0) {
            int zbyva = pocet_dnu_v_mesici(datum.mesic, datum.rok) - datum.den;
            if (pocet_dnu <= zbyva) { datum.den += pocet_dnu; pocet_dnu = 0; }
            else {
                // Переход на первое число следующего месяца тоже занимает один день.
                pocet_dnu -= zbyva + 1; datum.den = 1; datum.mesic += 1;
                if (datum.mesic > 12) { datum.mesic = 1; datum.rok += 1; }
            }
        }
        return datum;
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        Datum d=new Datum(16,7,2015);
        int pocet_dnu=500;
        Datum d1=nasledne_datum(d,pocet_dnu);
        Console.WriteLine("Puvodni datum:   {0:D2}.{1:D2}.{2}", d.den, d.mesic, d.rok);
        Console.WriteLine("Pocet dni:       {0}", pocet_dnu);
        Console.WriteLine("Nasledne datum:  {0:D2}.{1:D2}.{2}", d1.den, d1.mesic, d1.rok);
    }
}
