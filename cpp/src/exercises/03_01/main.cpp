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

#include <iostream>
#include <iomanip>

// Перечисление ограничивает выбор учебными ставками из условия.
enum class VatRate { Standard, FirstReduced, SecondReduced };
// Прибавляем к цене налог, вычисленный как цена * процент / 100.
static double price_with_vat(double price, double vat) { return price + price * vat / 100.0; }
// Возвращаем цену с налогом по именованной ставке учебного задания.
static double price_with_vat_rate(double price, VatRate vat) {
    double percent;
    if (vat == VatRate::Standard) percent = 21.0;
    else if (vat == VatRate::FirstReduced) percent = 15.0;
    else percent = 10.0;
    return price + price * percent / 100.0;
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    double x = price_with_vat(56.20, 21.0);
    double y1 = price_with_vat_rate(115.0, VatRate::Standard);
    double y2 = price_with_vat_rate(115.0, VatRate::FirstReduced);
    double y3 = price_with_vat_rate(115.0, VatRate::SecondReduced);
    std::cout << "The price 56.20 CZK with 21 % VAT is " << std::fixed << std::setprecision(2) << x << std::defaultfloat << " CZK." << '\n';
    std::cout << "The price 115.00 CZK with the standard 21 % VAT rate is " << std::fixed << std::setprecision(2) << y1 << std::defaultfloat << " CZK." << '\n';
    std::cout << "The price 115.00 CZK with the first reduced 15 % VAT rate is " << std::fixed << std::setprecision(2) << y2 << std::defaultfloat << " CZK." << '\n';
    std::cout << "The price 115.00 CZK with the second reduced 10 % VAT rate is " << std::fixed << std::setprecision(2) << y3 << std::defaultfloat << " CZK." << '\n';
}
