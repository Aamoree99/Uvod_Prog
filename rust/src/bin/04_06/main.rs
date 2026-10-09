/*
Cvičení 04 — Úloha 4.6
Název: Nejčastější prvek.
Body: 3 body

Zadání:
Napište funkci, která v zadaném poli najde nejčastěji se vyskytující hodnotu prvku. Vyskytují-li se
dvě hodnoty stejně často, přednost má první vyskytnuvši se hodnota. [3 body]


Ukázka:

int[] pole = { 2, 7, 6, 6, 6, 1, 7, 7, 2};

modus = NejcastejsiPrvek(pole); // 7

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

// Функция ищет самый часто встречающийся элемент.
// Перед первым вызовом frequency нужно установить в 0: функция улучшает переданный максимум.
//
// array  - массив, который только читаем
// frequency - сюда запишем, сколько раз встретился самый частый элемент
// most_frequent   - сюда запишем сам самый частый элемент
fn most_frequent_element(array: &[i32], frequency: &mut i32, most_frequent: &mut i32) {
    // Индекс в Rust должен быть usize,
    // потому что array.len() тоже возвращает usize.
    let mut i: usize = 0;

    // Проходим по каждому элементу массива.
    while i < array.len() {
        // Для каждого нового элемента начинаем считать заново.
        let mut occurrence_count: i32 = 0;

        // Вторым циклом проходим по всему массиву
        // и считаем, сколько раз встречается array[i].
        let mut j: usize = 0;

        while j < array.len() {
            if array[i] == array[j] {
                occurrence_count += 1;
            }

            j += 1;
        }

        // Если текущий элемент встречается чаще,
        // чем предыдущий лидер,
        // запоминаем его.
        //
        // Именно >, а не >=,
        // потому что при одинаковой частоте
        // должен остаться тот элемент,
        // который встретился раньше.
        if occurrence_count > *frequency {
            *frequency = occurrence_count;
            *most_frequent = array[i];
        }

        // Переходим к следующему элементу массива.
        i += 1;
    }
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let array = [2, 7, 6, 6, 6, 1, 7, 7, 2];

    // Сюда функция запишет результат.
    let mut frequency: i32 = 0;
    let mut most_frequent: i32 = 0;

    // Передаём ссылки, потому что функция должна
    // изменить frequency и most_frequent.
    most_frequent_element(&array, &mut frequency, &mut most_frequent);

    println!("Array: {:?}", array);
    println!(
        "The most frequent number is {}, occurring {} times",
        most_frequent, frequency
    );
}
