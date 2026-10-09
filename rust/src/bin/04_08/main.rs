/*
Cvičení 04 — Úloha 4.8
Název: Zaplatit mincemi
Body: 4 body; 2 body (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadané pole obsahující sestupně setříděné nominální hodnoty mincí
platících v ČR spočte minimální počet kusů mincí, které je nutno použít pro zaplacení zadané
částky.  [4 body].

b) Najděte sadu mincí (jinou než v ukázce) a částku, pro kterou funkce nenajde optimální řešení.
Nalezená sada pravděpodobně nebude platit v žádné zemi na světě. [2 body]


Ukázka:

int[] mince = { 50, 20, 10, 5, 2, 1 };

pocet = ZaplatitMincemi(mince, 97); // 5

pocet = ZaplatitMincemi(mince, 88); // 6

pocet = ZaplatitMincemi(mince, 26); // 3

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

// ---------------------------------------------------------
// ЖАДНЫЙ АЛГОРИТМ
//
// Именно такой алгоритм подразумевается в пункте a:
// всегда берём максимально возможное количество
// самой крупной доступной монеты.
// ---------------------------------------------------------
// Номиналы должны быть положительными, расположенными по убыванию и включать единицу.
fn pay_with_coins(coins: &[i32], amount: i32) -> i32 {
    // Остаток суммы.
    let mut remainder = amount;

    // Количество использованных монет.
    let mut count = 0;

    // Начинаем с самой крупной монеты.
    let mut i: usize = 0;

    while remainder > 0 {
        // Сколько монет данного номинала можем взять.
        let coin_count = remainder / coins[i];

        // Прибавляем их к общему количеству.
        count += coin_count;

        // Уменьшаем оставшуюся сумму.
        remainder -= coin_count * coins[i];

        // Переходим к следующему номиналу.
        i += 1;
    }

    count
}

// ---------------------------------------------------------
// ОПТИМАЛЬНЫЙ АЛГОРИТМ
//
// Перебирает возможные количества каждой монеты
// и действительно ищет минимальное число монет.
//
// index  - какой номинал сейчас рассматриваем
// amount - какую сумму ещё надо набрать
// ---------------------------------------------------------
fn optimal_coin_count(coins: &[i32], index: usize, amount: i32) -> i32 {
    // Если остаток равен нулю,
    // больше монет не нужно.
    if amount == 0 {
        return 0;
    }

    // Если монеты закончились,
    // а сумма ещё осталась,
    // таким способом сумму собрать нельзя.
    if index >= coins.len() {
        return i32::MAX / 2;
    }

    let coin = coins[index];

    // Максимальное количество текущей монеты,
    // которое вообще помещается в amount.
    let mut count = amount / coin;

    // Пока лучший вариант неизвестен.
    let mut minimum = i32::MAX / 2;

    // Пробуем:
    //
    // максимум текущих монет,
    // потом на одну меньше,
    // потом ещё меньше...
    while count >= 0 {
        // Остаток после использования count текущих монет.
        let remainder = amount - count * coin;

        // Ищем оптимальное решение для остальных номиналов.
        let remaining_count = optimal_coin_count(coins, index + 1, remainder);

        // Если остаток вообще удалось собрать...
        if remaining_count < i32::MAX / 2 {
            let total = count + remaining_count;

            // ...и это лучше предыдущего результата,
            // запоминаем новый минимум.
            if total < minimum {
                minimum = total;
            }
        }

        count -= 1;
    }

    minimum
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    // Стандартные чешские номиналы.
    let coins: [i32; 6] = [50, 20, 10, 5, 2, 1];

    let amount1 = 97;
    let amount2 = 88;
    let amount3 = 26;

    // Пункт a.
    let count1 = pay_with_coins(&coins, amount1);
    let count2 = pay_with_coins(&coins, amount2);
    let count3 = pay_with_coins(&coins, amount3);

    println!("PART A");
    println!();

    println!(
        "The minimum number of coins needed to pay {} CZK is {}.",
        amount1, count1
    );

    println!(
        "The minimum number of coins needed to pay {} CZK is {}.",
        amount2, count2
    );

    println!(
        "The minimum number of coins needed to pay {} CZK is {}.",
        amount3, count3
    );

    // -----------------------------------------------------
    // Пункт b.
    //
    // Нестандартные номиналы:
    // 4, 3, 1
    //
    // Нужно заплатить 6.
    // -----------------------------------------------------

    let strange_coins: [i32; 3] = [4, 3, 1];
    let strange_amount = 6;

    // Жадный алгоритм:
    // 4 + 1 + 1 = 3 монеты.
    let greedy_count = pay_with_coins(&strange_coins, strange_amount);

    // Оптимальный алгоритм:
    // 3 + 3 = 2 монеты.
    let optimal_count = optimal_coin_count(&strange_coins, 0, strange_amount);

    println!();
    println!("PART B");
    println!();

    println!("Coin set: {:?}", strange_coins);
    println!("Amount: {}", strange_amount);
    println!("Greedy algorithm: {} coins", greedy_count);
    println!("Optimal solution: {} coins", optimal_count);
}
