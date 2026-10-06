/*
Cvičení 03 — Úloha 3.2
Název: BMI
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
BMI (Body Mass Index) se počítá: váha v kilogramech vydělená druhou mocninou výšky v metrech.

a) Napište funkci, která pro zadanou hodnotu výšky a váhy spočte BMI. [1 bod]

b) Napište funkci, která místo číselné hodnoty bude vracet výčtový typ. [1 bod]


BMI                výčtový typ

[0; 16,5)          těžká podvýživa

[16,5; 18,5)     podváha

[18,5; 25)        optimální váha

[25; 30)           nadváha

[30; 35)           obezita prvního stupně

[35; 40)           obezita druhého stupně

[40; oo)           obezita třetího stupně


Ukázka:

x = SpoctiBMI1(1.70, 61.5); // 21.28

y = SpoctiBMI2(1.67, 80); // Nadvaha

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

// enum хранит категорию результата вместо произвольной строки.
enum BmiType {
    TezkaPodvyziva,
    Podvaha,
    OptimalniVaha,
    Nadvaha,
    ObezitaPrvnihoStupne,
    ObezitaDruhehoStupne,
    ObezitaTretihoStupne,
}

// По учебной формуле делим вес в килограммах на квадрат роста в метрах.
// Для этих функций предполагаются положительный рост и неотрицательный вес.
fn bmi1(height: f64, weight: f64) -> f64 {
    return weight / (height * height);
}

fn bmi2(height: f64, weight: f64) -> BmiType {
    let bmi = weight / (height * height);

    // Проверяем верхние границы по возрастанию. Предыдущие ветки уже исключили меньшие
    // значения, поэтому нижнюю границу повторно проверять не нужно. 25 относится к следующей категории.
    if bmi < 16.5 {
        return BmiType::TezkaPodvyziva;
    } else if bmi < 18.5 {
        return BmiType::Podvaha;
    } else if bmi < 25.0 {
        return BmiType::OptimalniVaha;
    } else if bmi < 30.0 {
        return BmiType::Nadvaha;
    } else if bmi < 35.0 {
        return BmiType::ObezitaPrvnihoStupne;
    } else if bmi < 40.0 {
        return BmiType::ObezitaDruhehoStupne;
    } else {
        return BmiType::ObezitaTretihoStupne;
    }
}

fn main() {
    let x: f64 = bmi1(1.70, 61.5);
    let y: BmiType = bmi2(1.67, 80.0);

    println!("BMI je {x:.2}.");

    // match сопоставляет возвращённую категорию с текстом для вывода.
    match y {
        BmiType::TezkaPodvyziva => println!("Kategorie BMI: tezka podvyziva."),
        BmiType::Podvaha => println!("Kategorie BMI: podvaha."),
        BmiType::OptimalniVaha => println!("Kategorie BMI: optimalni vaha."),
        BmiType::Nadvaha => println!("Kategorie BMI: nadvaha."),
        BmiType::ObezitaPrvnihoStupne => println!("Kategorie BMI: obezita prvniho stupne."),
        BmiType::ObezitaDruhehoStupne => println!("Kategorie BMI: obezita druheho stupne."),
        BmiType::ObezitaTretihoStupne => println!("Kategorie BMI: obezita tretiho stupne."),
    }
}
