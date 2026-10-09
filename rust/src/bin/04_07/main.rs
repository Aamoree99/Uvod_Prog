/*
Cvičení 04 — Úloha 4.7
Název: Skalární součin
Body: 2 body

Zadání:
Napište funkci, která spočte skalární součin dvou stejně dlouhých vektorů. Vektor o n složkách je
reprezentovaný pomocí pole délky n. Skalární součin pro dva vektory délky n se počítá jako součet n
součinů. Jednotlivé součiny vzniknou vynásobením odpovídajících složek prvního a druhého vektoru [2
body].


Ukázka:

int[] u = { 2, 0, -5, 1, 4 };

int[] v = { 7, 4, 2, -2, 0 };

skalar = SkalarniSoucin(u, v); // 2

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

// Функция добавляет скалярное произведение двух векторов к переданной сумме.
// Для получения самого произведения вызывающий код устанавливает sum в 0.
//
// first и second — два массива одинаковой длины.
// sum — переменная, в которую запишем результат.
fn scalar_product(first: &[i32], second: &[i32], sum: &mut i32) {
    // Индекс текущего элемента массива.
    let mut i: usize = 0;

    // Проходим по всем элементам.
    while i < first.len() {
        // Умножаем соответствующие элементы:
        // first[i] * second[i]
        //
        // и добавляем результат к общей сумме.
        *sum += first[i] * second[i];

        // Переходим к следующему элементу.
        i += 1;
    }
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    // Первый вектор.
    let first: [i32; 5] = [2, 0, -5, 1, 4];

    // Второй вектор.
    let second: [i32; 5] = [7, 4, 2, -2, 0];

    // Здесь будет храниться результат.
    let mut sum: i32 = 0;

    // Передаём оба массива и изменяемую ссылку на sum.
    scalar_product(&first, &second, &mut sum);

    // Вывод результата.
    println!("The scalar product is {}", sum);
}
