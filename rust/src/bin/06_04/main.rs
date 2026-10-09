/*
Cvičení 06 — Úloha 6.4
Název: Ruleta
Body: 10 bodů

Zadání:
Napište program, který umožní uživateli si zahrát evropskou ruletu. Hra probíhá v sázkových kolech.
Uživatel může v každém sázkovém kole uzavírat libovolný počet (a vzájemné kombinace) sázek dle
tabulky sázek a výher. Uživatel v rámci sázkového kola zadá všechny své sázky formou jednoho
textového řetězce. Formát zadávání sázek navrhněte sami, můžete se inspirovat formátem navrženým
v ukázce. Neplatné části řetězce ignorujte. Následně je náhodně vybráno jedno z čísel 0 až 36.
Uživateli je oznámeno, jak dopadly jeho jednotlivé sázky, jsou mu proplaceny případné výhry.
Uživatel může hru v jakémkoliv kole ukončit, případné sázky jsou anulovány. [10 bodů]



Tabulka sázek a výher

a) jednotlivé číslo (0 až 36), výhra 35x vsazená suma

b) liché číslo (1,3,5…35) nebo sudé číslo (2,4,6, …., 36), výhra 1x vsazená suma

c) malé číslo (1 až 18) nebo vysoké číslo (19 až 36), výhra 1x vsazená suma

d) tucet: první (1 až 12), druhý (13 až 24), třetí (25 až 36), výhra 2x vsazená suma


Ukázka:

Vitejte ve hre ruleta:

Sazka: cislo 0 az 36, licha, suda, nizka, vysoka, 1. az 3. tucet. Za sazku vlozte vsazenou sumu,
sazky lze kombinovat.

Sazka? 5 20 1 25 9 30 Suda 100 vysoka 200 hotovo

Na rulete padlo cislo 24.

Sazka na sude cislo ve vysi 100 byla vyhrana. Konto +100.

Sazka na vysoke cislo ve vysi 200 byla vyhrana. Konto +200.

Sazka na cislo 1 ve vysi 25 byla prohrana. Konto -25.

Sazka na cislo 5 ve vysi 20 byla prohrana. Konto -20.

Sazka na cislo 9 ve vysi 30 byla prohrana. Konto -30.

Stav konta je 222.

Sazka? Super jedeme dal 2 tucet 100, 15 50, suda 200

Na rulete padlo cislo 8.

Sazka na sude cislo ve vysi 200 byla vyhran. Konto +200.

Sazka na 2. tucet ve vysi 100 byla prohrana. Konto -100.

Sazka na cislo 15 ve vysi 50 byla prohrana. Konto -50.

Stav konta je 275.

Sazka? Konec, uz me to staci

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
use std::io::{self, Write};

// Максимальное количество ставок в одном раунде.
const MAX_BETS: usize = 100;

// Максимальная длина одной строки ввода в байтах.
const MAX_INPUT: usize = 4096;

// Типы ставок.
const NUMBER: i32 = 0;
const ODD: i32 = 1;
const EVEN: i32 = 2;
const LOW: i32 = 3;
const HIGH: i32 = 4;
const DOZEN1: i32 = 5;
const DOZEN2: i32 = 6;
const DOZEN3: i32 = 7;

// Одна ставка: тип, конкретное число и сумма.
// Структура нужна только для хранения данных.
// Все алгоритмы остаются процедурными.
#[derive(Clone, Copy)]
struct Bet {
    kind: i32,
    number: i32,
    amount: i64,
}

// Создаём пустую ставку для начального заполнения массива.
fn empty_bet() -> Bet {
    Bet {
        kind: NUMBER,
        number: 0,
        amount: 0,
    }
}

// Проверяем, является ли символ пробельным.
fn is_space(character: u8) -> bool {
    character == b' ' || character == b'\t' || character == b'\n' || character == b'\r'
}

// Возвращаем следующее слово из строки.
//
// position — текущая позиция чтения.
// Функция изменяет position и возвращает срез строки.
//
// Не используем split() и не создаём копию строки.
fn next_token<'a>(text: &'a str, position: &mut usize) -> &'a str {
    let bytes = text.as_bytes();

    // Пропускаем пробелы.
    while *position < bytes.len() && is_space(bytes[*position]) {
        *position += 1;
    }

    let start = *position;

    // Ищем конец слова.
    while *position < bytes.len() && is_space(bytes[*position]) == false {
        *position += 1;
    }

    &text[start..*position]
}

// Сравниваем команду без учёта регистра.
// Поддерживаем обычные английские буквы.
fn token_equals(token: &str, expected: &str) -> bool {
    token.eq_ignore_ascii_case(expected)
}

// Определяем тип ставки.
//
// Возвращаем:
// kind >= 0 — допустимый тип.
// -1 — неизвестная команда.
//
// number используется для ставки на конкретное число.
fn parse_bet_type(token: &str, number: &mut i32) -> i32 {
    if token_equals(token, "odd") {
        return ODD;
    }

    if token_equals(token, "even") {
        return EVEN;
    }

    if token_equals(token, "low") {
        return LOW;
    }

    if token_equals(token, "high") {
        return HIGH;
    }

    if token_equals(token, "dozen1") {
        return DOZEN1;
    }

    if token_equals(token, "dozen2") {
        return DOZEN2;
    }

    if token_equals(token, "dozen3") {
        return DOZEN3;
    }

    // Поддерживаем также чешские названия.
    if token_equals(token, "licha") {
        return ODD;
    }

    if token_equals(token, "suda") {
        return EVEN;
    }

    if token_equals(token, "nizka") {
        return LOW;
    }

    if token_equals(token, "vysoka") {
        return HIGH;
    }

    // Пробуем прочитать конкретное число.
    if let Ok(value) = token.parse::<i32>() {
        if value >= 0 && value <= 36 {
            *number = value;
            return NUMBER;
        }
    }

    -1
}

// Преобразуем текст в положительную сумму ставки.
// Некорректная, отрицательная или нулевая сумма отклоняется.
fn parse_amount(token: &str) -> i64 {
    if let Ok(amount) = token.parse::<i64>() {
        if amount > 0 {
            return amount;
        }
    }

    0
}

// Проверяем команду завершения игры.
fn is_quit(token: &str) -> bool {
    token_equals(token, "quit") || token_equals(token, "exit") || token_equals(token, "konec")
}

// Проверяем маркер завершения списка ставок.
fn is_done(token: &str) -> bool {
    token_equals(token, "done") || token_equals(token, "hotovo")
}

// Разбираем всю строку ставок.
//
// Каждая ставка состоит из двух слов:
// тип ставки и сумма.
//
// Например:
// "5 20 even 100 dozen2 50"
//
// Неизвестные фрагменты пропускаем.
//
// Возвращаем количество корректных ставок.
// quit_found сообщает, что пользователь хочет выйти.
fn parse_bets(text: &str, bets: &mut [Bet; MAX_BETS], quit_found: &mut bool) -> usize {
    let mut position = 0;
    let mut count = 0;

    while position < text.len() {
        let token = next_token(text, &mut position);

        if token.is_empty() {
            break;
        }

        // Если пользователь попросил завершить игру,
        // весь раунд отменяется.
        if is_quit(token) {
            *quit_found = true;
            return 0;
        }

        if is_done(token) {
            break;
        }

        let mut number = 0;
        let kind = parse_bet_type(token, &mut number);

        // Неизвестное слово пропускаем.
        if kind == -1 {
            continue;
        }

        // Следующее слово должно быть суммой.
        let amount_token = next_token(text, &mut position);

        if amount_token.is_empty() {
            break;
        }

        // Не даём использовать некорректную сумму.
        let amount = parse_amount(amount_token);

        if amount == 0 {
            continue;
        }

        if count >= MAX_BETS {
            println!("Maximum number of bets reached.");
            break;
        }

        bets[count] = Bet {
            kind,
            number,
            amount,
        };

        count += 1;
    }

    count
}

// Проверяем, выиграла ли ставка при выпавшем числе.
//
// Ноль выигрывает только по ставке на конкретный 0.
// В чётные, нечётные, низкие, высокие и дюжины
// ноль не входит.
fn bet_wins(bet: Bet, result: i32) -> bool {
    match bet.kind {
        NUMBER => result == bet.number,

        ODD => result != 0 && result % 2 == 1,

        EVEN => result != 0 && result % 2 == 0,

        LOW => result >= 1 && result <= 18,

        HIGH => result >= 19 && result <= 36,

        DOZEN1 => result >= 1 && result <= 12,

        DOZEN2 => result >= 13 && result <= 24,

        DOZEN3 => result >= 25 && result <= 36,

        _ => false,
    }
}

// Коэффициент чистого выигрыша.
//
// Конкретное число: 35 к 1.
// Чётное/нечётное, низкое/высокое: 1 к 1.
// Дюжина: 2 к 1.
fn payout_multiplier(kind: i32) -> i64 {
    match kind {
        NUMBER => 35,
        ODD | EVEN | LOW | HIGH => 1,
        DOZEN1 | DOZEN2 | DOZEN3 => 2,
        _ => 0,
    }
}

// Выводим название ставки.
fn print_bet_name(bet: Bet) {
    match bet.kind {
        NUMBER => print!("Number {}", bet.number),
        ODD => print!("Odd"),
        EVEN => print!("Even"),
        LOW => print!("Low (1-18)"),
        HIGH => print!("High (19-36)"),
        DOZEN1 => print!("First dozen (1-12)"),
        DOZEN2 => print!("Second dozen (13-24)"),
        DOZEN3 => print!("Third dozen (25-36)"),
        _ => print!("Unknown"),
    }
}

// Обрабатываем все ставки одного раунда.
//
// При выигрыше начисляем чистую прибыль.
// При проигрыше вычитаем сумму ставки.
//
// Возвращаем изменение баланса за раунд.
fn play_round(bets: &[Bet], result: i32) -> i64 {
    let mut change: i64 = 0;
    let mut i = 0;

    println!();
    println!("Roulette number: {}", result);
    println!("----------------------------------------");

    while i < bets.len() {
        let bet = bets[i];

        print!("Bet: ");
        print_bet_name(bet);
        print!(" | Amount: {}", bet.amount);

        if bet_wins(bet, result) {
            // Проверяем переполнение при расчёте выплаты.
            let profit = bet.amount.checked_mul(payout_multiplier(bet.kind));

            match profit {
                Some(value) => {
                    // При слишком большом общем выигрыше
                    // изменение баланса не должно переполниться.
                    if let Some(new_change) = change.checked_add(value) {
                        change = new_change;
                        println!(" | WIN | +{}", value);
                    } else {
                        println!(" | PAYOUT OVERFLOW");
                    }
                }
                None => {
                    println!(" | PAYOUT OVERFLOW");
                }
            }
        } else {
            if let Some(new_change) = change.checked_sub(bet.amount) {
                change = new_change;
                println!(" | LOSE | -{}", bet.amount);
            } else {
                println!(" | BALANCE OVERFLOW");
            }
        }

        i += 1;
    }

    change
}

// Показываем доступные типы ставок.
fn print_rules() {
    println!("EUROPEAN ROULETTE");
    println!("========================================");
    println!("Bet types:");
    println!("  0..36       Single number (35:1)");
    println!("  odd         Odd number (1:1)");
    println!("  even        Even number (1:1)");
    println!("  low         Numbers 1-18 (1:1)");
    println!("  high        Numbers 19-36 (1:1)");
    println!("  dozen1      Numbers 1-12 (2:1)");
    println!("  dozen2      Numbers 13-24 (2:1)");
    println!("  dozen3      Numbers 25-36 (2:1)");
    println!();
    println!("Format: TYPE AMOUNT [TYPE AMOUNT ...]");
    println!("Example: 5 20 1 25 9 30 even 100 high 200");
    println!("Commands: done, quit");
    println!("Invalid bets are ignored.");
    println!();
}

// MAIN
fn main() {
    let mut rng = rand::thread_rng();

    // Игровой баланс.
    // По условию начальная сумма не указана,
    // поэтому начинаем с нуля и считаем прибыль/убыток.
    let mut balance: i64 = 0;

    // Обычный массив ставок без Vec.
    let mut bets = [empty_bet(); MAX_BETS];

    print_rules();

    loop {
        println!("----------------------------------------");
        println!("Current balance: {}", balance);
        print!("Your bets: ");

        // Принудительно выводим приглашение
        // до ожидания ввода пользователя.
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // Читаем одну строку со всеми ставками.
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF — завершение ввода.
                println!();
                println!("End of input.");
                break;
            }

            Ok(_) => {}

            Err(error) => {
                println!("Input error: {}", error);
                break;
            }
        }

        // Ограничиваем длину строки ввода.
        if input.len() > MAX_INPUT {
            println!("Input is too long. Round cancelled.");
            continue;
        }

        // Разбираем ставки.
        let mut quit_found = false;

        let count = parse_bets(&input, &mut bets, &mut quit_found);

        // Завершаем игру без проведения раунда.
        if quit_found == true {
            println!("Game finished.");
            break;
        }

        // Если корректных ставок нет, рулетку не запускаем.
        if count == 0 {
            println!("No valid bets. Try again.");
            continue;
        }

        // Европейская рулетка содержит 37 чисел: 0..36.
        let result = rng.gen_range(0..=36);

        // Обрабатываем только заполненную часть массива.
        let change = play_round(&bets[..count], result);

        // Обновляем баланс.
        if let Some(new_balance) = balance.checked_add(change) {
            balance = new_balance;
        } else {
            println!("Balance overflow. Game finished.");
            break;
        }

        println!("----------------------------------------");
        println!("Round result: {:+}", change);
        println!("Current balance: {}", balance);
        println!();
    }

    println!();
    println!("FINAL RESULT");
    println!("========================================");
    println!("Final balance: {}", balance);
}
