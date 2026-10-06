/*
Cvičení 03 — Úloha 3.3
Název: Aritmetický výraz
Body: 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce bez použití funkcí z Math. Návratové hodnoty funkcí budou vždy reálná
čísla. Reálná čísla jsou schopna reprezentovat vyšší číselné hodnoty než čísla celá

a) Funkci, která pro zadané celé číslo spočte jeho faktoriál. [1 bod]

b) Funkci, která pro zadané reálné číslo spočte jeho nezápornou celočíselnou mocninu [1 bod]

c) Funkci, která pro zadané reálné číslo n spočte hodnotu e^n, která je přibližně dána níže
uvedeným vztahem. [1 bod]

e^n ≈ Σ_{k=0}^{100} n^k / k!



d) Funkci, která pro zadaná celá čísla n a k spočte jejich kombinační číslo. Pro dvojice n a k
nesplňující podmínku n ≥ k ≥ 0 je kombinační číslo rovno 0, pro ostatní dvojice je kombinační číslo
dané níže uvedeným vztahem. [1 bod]

(n nad k) = n! / (k! (n − k)!)

e^n ≈ Σ_{k=0}^{100} n^k / k!


Ukázka:

x = faktorial(11); // 39916800

y = mocnina(5, 3); // 125.0

z = mocnina(2.69, 7); //1019.215

a = vyraz(5.14); // 170.715

b = kombinace(49, 6); // 13983816.0

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

// Факториал — произведение от 1 до n. Возвращаем f64 по условию задания,
// поэтому результат приближённый для больших чисел; 0 для отрицательного n — условный ответ.
fn factorial(n: i32) -> f64 {
    if n < 0 {
        return 0.0;
    }

    // Начальное произведение равно 1: это также обеспечивает 0! = 1.
    let mut result: f64 = 1.0;
    let mut i: i32 = 1;

    while i <= n {
        result = result * i as f64;
        i += 1;
    }

    return result;
}

// Степень считаем повторным умножением. По условию exponent должен быть >= 0;
// нулевая степень возвращает 1, поскольку цикл не выполняется.
fn power(number: f64, exponent: i32) -> f64 {
    let mut result: f64 = 1.0;
    let mut i: i32 = 0;

    while i < exponent {
        result = result * number;
        i += 1;
    }

    return result;
}

// Складываем 101 член ряда от k = 0 до k = 100 включительно.
// Используем написанные выше функции; это приближение e^n, а не точное равенство.
fn expression(n: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let mut k: i32 = 0;

    while k <= 100 {
        sum = sum + power(n, k) / factorial(k);
        k += 1;
    }

    return sum;
}

// Комбинации допустимы только при n >= k >= 0. Для остальных входов возвращаем 0.
// Для допустимых применяем n! / (k! * (n-k)!).
fn combination(n: i32, k: i32) -> f64 {
    if n < k || k < 0 {
        return 0.0;
    }

    return factorial(n) / (factorial(k) * factorial(n - k));
}

fn main() {
    let x = factorial(11);
    let y = power(5.0, 3);
    let z = power(2.69, 7);
    let a = expression(5.14);
    let b = combination(49, 6);

    println!("11! = {x:.0}");
    println!("5^3 = {y:.1}");
    println!("2.69^7 = {z:.3}");
    println!("e^5.14 ≈ {a:.3}");
    println!("C(49, 6) = {b:.1}");
}
