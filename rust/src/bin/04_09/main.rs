/*
Cvičení 04 — Úloha 4.9
Název: Sportka
Body: 10 bodů

Zadání:
Napište funkci, která umožní uživateli si zahrát sportku. Výsledkem funkce bude finanční částka
vyhraná (častěji však prohraná) uživatelem. Uživatel vyplní na sázence 6 různých čísel z čísel 1 až
49 a počet slosování, na které chce čísla vsadit. Cena jednoho slosování je 20 Kč. Pokud uživatel
zadá na sázence neplatnou kombinaci čísel, je mu vrácen vklad. V každém slosování se losují dva
tahy. V každém tahu se ze 49 čísel vylosuje 6 řádných čísel a jedno číslo dodatkové. Tahy jsou
navzájem nezávislé. V každém tahu uživatel obdrží nejvyšší z výher dle tabulky. Výhry z obou tahů
se sčítají. [10 bodů]

Tabulka výher

1. pořadí: 15 382 198 Kč (uhodnuto 6 řádných čísel)

2. pořadí: 815 723 Kč (uhodnuto 5 řádných čísel a dodatkové číslo)

3. pořadí: 24 971 Kč (uhodnuto 5 řádných čísel)

4. pořadí: 619 Kč (uhodnuta 4 řádná čísla)

5. pořadí: 113 Kč (uhodnuta 3 řádná čísla)

Zdroj: http://www.sazka.cz/cz/loterie-a-hry/sportka/jak-hrat/charakteristika/


Ukázka:

int[] sazenka = { 7, 9, 22, 6, 14, 48 };

vys = Sportka(sazenka, 5000); // -74030

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
use rand::Rng;

// Проверяем правильность билета.
//
// Игрок должен выбрать ровно 6 разных чисел
// в диапазоне от 1 до 49.
fn validate_ticket(ticket: &[i32; 6]) -> bool {
    let mut i = 0;

    while i < ticket.len() {
        // Проверяем, что число находится от 1 до 49.
        if ticket[i] < 1 || ticket[i] > 49 {
            return false;
        }

        // Проверяем, что число не повторяется.
        // Сравниваем его только с последующими элементами.
        let mut j = i + 1;

        while j < ticket.len() {
            if ticket[i] == ticket[j] {
                return false;
            }

            j += 1;
        }

        i += 1;
    }

    // Если все проверки прошли, билет правильный.
    true
}

// Генерируем семь различных случайных чисел.
//
// Первые шесть — основные выигрышные числа.
// Седьмое — дополнительное число.
fn generate_draw() -> [i32; 7] {
    // Создаём массив из семи нулей.
    let mut numbers = [0; 7];

    // Генератор случайных чисел.
    let mut rng = rand::thread_rng();

    let mut i = 0;

    // Заполняем массив случайными числами.
    while i < numbers.len() {
        // Генерируем число от 1 до 49 включительно.
        let number = rng.gen_range(1..=49);

        // Предполагаем, что число ещё не встречалось.
        let mut duplicate = false;

        // Проверяем уже заполненные элементы массива.
        let mut j = 0;

        while j < i {
            if numbers[j] == number {
                duplicate = true;
                break;
            }

            j += 1;
        }

        // Если число не повторяется, сохраняем его.
        if duplicate == false {
            numbers[i] = number;
            i += 1;
        }

        // Если число уже было, i не увеличиваем.
        // На следующем проходе генерируем новое число.
    }

    numbers
}

// Сравниваем билет игрока с выигрышными числами.
//
// Возвращаем два значения:
// matches — количество совпавших основных чисел.
// bonus   — совпало ли дополнительное число.
fn count_matches(ticket: &[i32; 6], draw: &[i32; 7]) -> (i32, bool) {
    let mut matches = 0;
    let mut bonus = false;

    let mut i = 0;

    // Проверяем каждое число билета.
    while i < ticket.len() {
        let mut j = 0;

        // Первые шесть чисел тиража — основные.
        while j < 6 {
            if ticket[i] == draw[j] {
                matches += 1;
            }

            j += 1;
        }

        // Седьмое число — дополнительное.
        if ticket[i] == draw[6] {
            bonus = true;
        }

        i += 1;
    }

    // Возвращаем оба результата.
    (matches, bonus)
}

// Определяем выигрыш за один тираж.
//
// Выплачиваем только одну, самую высокую награду.
fn calculate_prize(matches: i32, bonus: bool) -> i64 {
    // Совпали все шесть основных чисел.
    if matches == 6 {
        return 15_382_198;
    }

    // Совпали пять основных и дополнительное.
    if matches == 5 && bonus == true {
        return 815_723;
    }

    // Совпали пять основных чисел.
    if matches == 5 {
        return 24_971;
    }

    // Совпали четыре основных числа.
    if matches == 4 {
        return 619;
    }

    // Совпали три основных числа.
    if matches == 3 {
        return 113;
    }

    // Меньше трёх совпадений — выигрыша нет.
    return 0;
}

// Главная функция лотереи.
//
// ticket — шесть чисел игрока.
// draws  — количество розыгрышей.
//
// Каждый розыгрыш стоит 20 CZK
// и состоит из двух независимых тиражей.
//
// Функция возвращает итоговый финансовый результат:
// положительное число — прибыль,
// отрицательное число — убыток.
fn sportka(ticket: &[i32; 6], draws: i32) -> i64 {
    // Неправильный билет не участвует в лотерее.
    // Ставка возвращается, финансовый результат равен нулю.
    if validate_ticket(ticket) == false {
        return 0;
    }

    // Не проводим розыгрыши при нулевом
    // или отрицательном их количестве.
    if draws <= 0 {
        return 0;
    }

    // Цена одного розыгрыша — 20 CZK.
    //
    // Переводим draws в i64, чтобы безопаснее
    // работать с крупными суммами.
    let cost = draws as i64 * 20;

    // Начинаем с отрицательного баланса:
    // игрок сначала оплачивает все розыгрыши.
    let mut balance: i64 = -cost;

    let mut i = 0;

    // Проводим указанное количество розыгрышей.
    while i < draws {
        // В каждом розыгрыше два независимых тиража.
        let mut round = 0;

        while round < 2 {
            // Генерируем новый набор случайных чисел.
            let draw = generate_draw();

            // Сравниваем билет с выпавшими числами.
            let (matches, bonus) = count_matches(ticket, &draw);

            // Вычисляем выигрыш за этот тираж.
            let prize = calculate_prize(matches, bonus);

            // Добавляем выигрыш к общему балансу.
            balance += prize;

            // Переходим ко второму тиражу.
            round += 1;
        }

        // Переходим к следующему розыгрышу.
        i += 1;
    }

    // Возвращаем итоговую прибыль или убыток.
    return balance;
}

fn main() {
    // Числа игрока из задания.
    let ticket = [7, 9, 22, 6, 14, 48];

    // Количество розыгрышей.
    let draws = 5000;

    // Проверяем правильность билета.
    println!("SPORTKA LOTTERY");
    println!("========================================");
    println!("Your ticket: {:?}", ticket);
    println!("Number of drawings: {}", draws);
    println!("Price per drawing: 20 CZK");
    println!();

    if validate_ticket(&ticket) == false {
        println!("Invalid ticket. Your money has been refunded.");
        return;
    }

    // Считаем стоимость всех розыгрышей.
    let total_cost = draws as i64 * 20;

    // Запускаем симуляцию.
    let result = sportka(&ticket, draws);

    // Считаем общую сумму выигрышей.
    // result = winnings - cost
    // значит winnings = result + cost
    let total_winnings = result + total_cost;

    println!("Ticket validation: VALID");
    println!("Total cost: {} CZK", total_cost);
    println!("Total winnings: {} CZK", total_winnings);
    println!("----------------------------------------");

    // Выводим итоговый результат.
    if result > 0 {
        println!("Net profit: +{} CZK", result);
    } else if result < 0 {
        println!("Net loss: {} CZK", result);
    } else {
        println!("You broke even.");
    }

    println!("========================================");

    // Дополнительная проверка неправильного билета.
    let invalid_ticket = [7, 9, 22, 6, 14, 14];

    println!();
    println!("INVALID TICKET TEST");
    println!("Ticket: {:?}", invalid_ticket);

    let invalid_result = sportka(&invalid_ticket, draws);

    println!("Financial result: {} CZK", invalid_result);
}
