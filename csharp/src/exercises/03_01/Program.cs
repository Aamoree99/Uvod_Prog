/*
Cvičení 03 — Úloha 3.1
Název: DPH
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadanou cenu zboží a číselně zadanou sazbu DPH spočte cenu zboží
s DPH. [1 bod]

b) Napište funkci, která pro sazbu DPH použije výčtový typ. Existují tři sazby DPH: základní 21%,
první snížená 15% a druhá snížená 10% [1 bod]:

Ukázka:

x = SpoctiDPH1(56.20, 21); // 68.002

y = SpoctiDPH2(115, DPH.PrvniSnizena); // 132.25

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

    // Перечисление ограничивает выбор учебными ставками из условия.
    enum VatRate { Standard, FirstReduced, SecondReduced }
    // Прибавляем к цене налог, вычисленный как цена * процент / 100.
    static double PriceWithVat(double price, double vat) { return price + price * vat / 100.0; }
    // Возвращаем цену с налогом по именованной ставке учебного задания.
    static double PriceWithVatRate(double price, VatRate vat) {
        double percent;
        if (vat == VatRate.Standard) percent = 21.0;
        else if (vat == VatRate.FirstReduced) percent = 15.0;
        else percent = 10.0;
        return price + price * percent / 100.0;
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        double x = PriceWithVat(56.20, 21.0);
        double y1 = PriceWithVatRate(115.0, VatRate.Standard);
        double y2 = PriceWithVatRate(115.0, VatRate.FirstReduced);
        double y3 = PriceWithVatRate(115.0, VatRate.SecondReduced);
        Console.WriteLine("The price 56.20 CZK with 21 % VAT is {0:F2} CZK.", x);
        Console.WriteLine("The price 115.00 CZK with the standard 21 % VAT rate is {0:F2} CZK.", y1);
        Console.WriteLine("The price 115.00 CZK with the first reduced 15 % VAT rate is {0:F2} CZK.", y2);
        Console.WriteLine("The price 115.00 CZK with the second reduced 10 % VAT rate is {0:F2} CZK.", y3);
    }
}
