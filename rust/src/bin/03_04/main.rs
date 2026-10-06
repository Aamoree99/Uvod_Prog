/*
Cvičení 03 — Úloha 3.4
Název: Komplexní čísla
Body: 1/2 bodu; 1/2 bodu; 1 bod (v pořadí podúloh)

Zadání:
Vytvořte strukturu pro reprezentaci komplexních čísel v algebraickém tvaru. Napište funkce pro
následující operace

a) sčítání komplexních čísel [1/2 bodu]

b) odečítání komplexních čísel [1/2 bodu]

c) násobení komplexních čísel [1 bod]




Ukázka:

x.re = 5.2; x.im = 3.25;

y.re = 1.6; y.im = -4;

a = secti(x, y); // 6.8 - 0.75i

b = odecti(x, y); // 3.6 + 7.25i

c = vynasob(x, y); // 21.32 - 15.6i

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

// Структура объединяет две связанные части комплексного числа:
// re — действительная часть, im — коэффициент при мнимой единице i.
struct Complex {
    re: f64,
    im: f64,
}

// При сложении комплексных чисел отдельно складываются
// действительные и отдельно мнимые части:
// (a + bi) + (c + di) = (a + c) + (b + d)i
fn add(x: Complex, y: Complex) -> Complex {
    return Complex {
        re: x.re + y.re,
        im: x.im + y.im,
    };
}

// При вычитании отдельно вычитаются
// действительные и мнимые части:
// (a + bi) - (c + di) = (a - c) + (b - d)i
fn subtract(x: Complex, y: Complex) -> Complex {
    return Complex {
        re: x.re - y.re,
        im: x.im - y.im,
    };
}

// При умножении раскрываем скобки и используем правило i^2 = -1:
// (a + bi)(c + di) = (ac - bd) + (ad + bc)i
fn multiply(x: Complex, y: Complex) -> Complex {
    return Complex {
        re: x.re * y.re - x.im * y.im,
        im: x.re * y.im + x.im * y.re,
    };
}

fn main() {
    let x = Complex { re: 5.2, im: 3.25 };
    let y = Complex { re: 1.6, im: -4.0 };

    // После передачи структуры в функцию значение считается переданным функции,
    // поэтому для следующей операции в этом простом варианте создаём x и y заново.
    let a = add(x, y);

    let x = Complex { re: 5.2, im: 3.25 };
    let y = Complex { re: 1.6, im: -4.0 };

    let b = subtract(x, y);

    let x = Complex { re: 5.2, im: 3.25 };
    let y = Complex { re: 1.6, im: -4.0 };

    let c = multiply(x, y);

    // {:+} всегда выводит знак числа:
    // положительное как +7.25, отрицательное как -0.75.
    println!("Soucet: {:.4} {:+.4}i", a.re, a.im);
    println!("Rozdil: {:.4} {:+.4}i", b.re, b.im);
    println!("Soucin: {:.4} {:+.4}i", c.re, c.im);
}
