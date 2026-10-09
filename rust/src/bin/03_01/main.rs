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

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let x = price_with_vat(56.20, 21.0);
    let y1 = price_with_vat_rate(115.0, VatRate::Standard);
    let y2 = price_with_vat_rate(115.0, VatRate::FirstReduced);
    let y3 = price_with_vat_rate(115.0, VatRate::SecondReduced);

    println!("The price 56.20 CZK with 21 % VAT is {x:.2} CZK.");
    println!("The price 115.00 CZK with the standard 21 % VAT rate is {y1:.2} CZK.");
    println!("The price 115.00 CZK with the first reduced 15 % VAT rate is {y2:.2} CZK.");
    println!("The price 115.00 CZK with the second reduced 10 % VAT rate is {y3:.2} CZK.");
}

// enum ограничивает выбор тремя именованными вариантами ставки из учебного задания.
enum VatRate {
    Standard,
    FirstReduced,
    SecondReduced,
}

// Функция принимает цену и процент и возвращает f64. Налог = цена * процент / 100.
// Прибавляем налог к исходной цене; ставки здесь взяты из условия упражнения.
fn price_with_vat(price: f64, vat: f64) -> f64 {
    return price + price * vat / 100.0;
}

// Возвращаем цену с налогом по именованной ставке учебного задания.
fn price_with_vat_rate(price: f64, vat: VatRate) -> f64 {
    // match переводит выбранный вариант enum в числовой процент.
    // Формат :.2 в main округляет только отображение, а не результат функции.
    let percent: f64 = match vat {
        VatRate::Standard => 21.0,
        VatRate::FirstReduced => 15.0,
        VatRate::SecondReduced => 10.0,
    };

    return price + price * percent / 100.0;
}
