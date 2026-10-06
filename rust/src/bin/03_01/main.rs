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

fn main() {
    let x = dph1(56.20, 21.0);
    let y1 = dph2(115.0, Dph::Zakladni);
    let y2 = dph2(115.0, Dph::PrvniSnizena);
    let y3 = dph2(115.0, Dph::DruhaSnizena);

    println!("Cena 56.20 Kč se sazbou DPH 21 % je {x:.2} Kč.");
    println!("Cena 115.00 Kč se základní sazbou DPH 21 % je {y1:.2} Kč.");
    println!("Cena 115.00 Kč s první sníženou sazbou DPH 15 % je {y2:.2} Kč.");
    println!("Cena 115.00 Kč s druhou sníženou sazbou DPH 10 % je {y3:.2} Kč.");
}

// enum ограничивает выбор тремя именованными вариантами ставки из учебного задания.
enum Dph {
    Zakladni,
    PrvniSnizena,
    DruhaSnizena,
}

// Функция принимает цену и процент и возвращает f64. Налог = цена * процент / 100.
// Прибавляем налог к исходной цене; ставки здесь взяты из условия упражнения.
fn dph1(price: f64, dph: f64) -> f64 {
    return price + price * dph / 100.0;
}

fn dph2(price: f64, dph: Dph) -> f64 {
    // match переводит выбранный вариант enum в числовой процент.
    // Формат :.2 в main округляет только отображение, а не результат функции.
    let percent: f64 = match dph {
        Dph::Zakladni => 21.0,
        Dph::PrvniSnizena => 15.0,
        Dph::DruhaSnizena => 10.0,
    };

    return price + price * percent / 100.0;
}
