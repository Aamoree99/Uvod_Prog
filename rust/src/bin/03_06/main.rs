/*
Cvičení 03 — Úloha 3.6
Název: Kámen, nůžky papír
Body: 5 bodů

Zadání:
Napište program, který si s uživatelem zahraje hru kámen, nůžky, papír. Hraje se na dvě vítězná
kola. Použijte výčtový typ. V každém kole si  uživatel zvolí jeden z výše uvedených
předmětů předmětů, počítač svůj předmět vylosuje náhodně. Kámen poráží nůžky, nůžky poráží papír,
papír poráží kámen. Pokud si uživatel i počítač zvolili stejný předmět, dané kolo se opakuje do té
doby, dokud nebudou zvoleny různé předměty. [5 bodů]


Ukázka:

Vitejte ve hre kamen – nuzky – papir

Hrajeme na dve vitezna kola

Zadejte svuj tah: 6

Tento tah je neplatny

Zadejte svuj tah: kamen

Hrac zahral Kamen, pocitac zahral Kamen

Toto kolo je nerozhodne

Celkove skore: hrac 0 – pocitac 0

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Nuzky

Toto kolo vyhral pocitac

Celkove skore: hrac 0 – pocitac 1

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Kamen

Toto kolo vyhral hrac

Celkove skore: hrac 1 – pocitac 1

Zadejte svuj tah: nuzky

Hrac zahral Nuzky, pocitac zahral Papir

Toto kolo vyhral hrac

Celkove skore: hrac 2 – pocitac 1

Celou hru vyhral hrac

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
use std::thread;
use std::time::Duration;

// Возможные ходы.
#[derive(Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Scissors,
    Paper,
}

// Возможные результаты раунда.
enum RoundResult {
    Draw,
    Player,
    Computer,
}

// Очищаем терминал.
//
// \x1B[2J     -> очистить экран
// \x1B[1;1H   -> переместить курсор в левый верхний угол
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");

    // print! сам по себе может не вывести текст сразу,
    // поэтому принудительно очищаем буфер вывода.
    io::stdout().flush().unwrap();
}

// Возвращаем текстовое название хода.
fn move_text(selected_move: Move) -> &'static str {
    match selected_move {
        Move::Rock => "Rock",
        Move::Scissors => "Scissors",
        Move::Paper => "Paper",
    }
}

// Возвращаем emoji для хода.
fn move_emoji(selected_move: Move) -> &'static str {
    match selected_move {
        Move::Rock => "🪨",
        Move::Scissors => "✂️",
        Move::Paper => "📄",
    }
}

// ASCII-art для выбранного хода.
//
// Функция возвращает строку &'static str,
// потому что все картинки заранее записаны прямо в программе.
fn move_ascii(selected_move: Move) -> &'static str {
    match selected_move {
        Move::Rock => {
            r#"
    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
"#
        }

        Move::Scissors => {
            r#"
    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
"#
        }

        Move::Paper => {
            r#"
    _______
---'   ____)____
          ______)
          _______)
         _______)
---.__________)
"#
        }
    }
}

// Анимация перед показом результатов.
//
// Показываем:
// Камень
// Ножницы
// Бумага
//
// С паузой между кадрами.
fn animate_moves() {
    clear_screen();

    println!();
    println!("             ROCK!");
    println!();

    println!(
        r#"
    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
"#
    );

    thread::sleep(Duration::from_millis(600));

    clear_screen();

    println!();
    println!("             SCISSORS!");
    println!();

    println!(
        r#"
    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
"#
    );

    thread::sleep(Duration::from_millis(600));

    clear_screen();

    println!();
    println!("             PAPER!");
    println!();

    println!(
        r#"
    _______
---'   ____)____
          ______)
          _______)
         _______)
---.__________)
"#
    );

    thread::sleep(Duration::from_millis(600));
}

// Читаем ход пользователя.
//
// Возвращаем:
// Some(Move) -> если ввод правильный
// None      -> если неправильный
fn read_player_move() -> Option<Move> {
    let mut input = String::new();

    print!("Enter your move [rock / scissors / paper]: ");

    // Чтобы prompt появился до ожидания ввода.
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim().to_lowercase();

    match input.as_str() {
        // Можно писать полное слово.
        "rock" => Some(Move::Rock),
        "scissors" => Some(Move::Scissors),
        "paper" => Some(Move::Paper),

        // И короткий вариант.
        "r" => Some(Move::Rock),
        "s" => Some(Move::Scissors),
        "p" => Some(Move::Paper),

        // Всё остальное неверно.
        _ => None,
    }
}

// Компьютер случайно выбирает ход.
fn computer_move() -> Move {
    // Генерируем число:
    // 0
    // 1
    // 2
    let number = rand::thread_rng().gen_range(0..3);

    match number {
        0 => Move::Rock,
        1 => Move::Scissors,
        _ => Move::Paper,
    }
}

// Определяем результат раунда.
fn round_result(player: Move, computer: Move) -> RoundResult {
    // match можно делать сразу по двум значениям.
    match (player, computer) {
        // Одинаковые ходы = ничья.
        (Move::Rock, Move::Rock) => RoundResult::Draw,
        (Move::Scissors, Move::Scissors) => RoundResult::Draw,
        (Move::Paper, Move::Paper) => RoundResult::Draw,

        // Победы игрока.
        (Move::Rock, Move::Scissors) => RoundResult::Player,
        (Move::Scissors, Move::Paper) => RoundResult::Player,
        (Move::Paper, Move::Rock) => RoundResult::Player,

        // Все остальные варианты = победа компьютера.
        _ => RoundResult::Computer,
    }
}

// Красиво показываем текущий счёт.
fn show_score(player_score: i32, computer_score: i32) {
    println!();
    println!("========================================");
    println!(
        "        PLAYER {} : {} COMPUTER",
        player_score, computer_score
    );
    println!("========================================");
    println!();
}

// Показываем выбор игрока и компьютера.
fn show_moves(player: Move, computer: Move) {
    println!();
    println!("PLAYER                         COMPUTER");
    println!(
        "{} {}                     {} {}",
        move_emoji(player),
        move_text(player),
        move_text(computer),
        move_emoji(computer)
    );

    println!();

    // Пока ASCII выводим один под другим.
    //
    // Это проще и надёжнее, чем пытаться
    // склеивать многострочные строки по горизонтали.
    println!("PLAYER:");
    println!("{}", move_ascii(player));

    println!("COMPUTER:");
    println!("{}", move_ascii(computer));
}

// Главное меню перед игрой.
fn show_intro() {
    clear_screen();

    println!("========================================");
    println!("         ROCK - SCISSORS - PAPER");
    println!("========================================");
    println!();
    println!("The first to win two rounds wins the game.");
    println!();
    println!("You can enter:");
    println!("  rock / r");
    println!("  scissors / s");
    println!("  paper / p");
    println!();
}

// Одна целая игра до двух побед.
fn play_game() {
    let mut player_score = 0;
    let mut computer_score = 0;

    show_intro();

    // Игра продолжается,
    // пока кто-то не одержит 2 победы.
    while player_score < 2 && computer_score < 2 {
        let player;

        // Запрашиваем ввод,
        // пока пользователь не введёт допустимый ход.
        loop {
            match read_player_move() {
                Some(selected_move) => {
                    player = selected_move;
                    break;
                }

                None => {
                    println!();
                    println!("Invalid move.");
                    println!();
                }
            }
        }

        // Компьютер выбирает ход.
        let computer = computer_move();

        // Показываем небольшую анимацию.
        animate_moves();

        // Очищаем экран после анимации
        // и показываем реальные ходы.
        clear_screen();

        show_moves(player, computer);

        // Определяем победителя.
        match round_result(player, computer) {
            RoundResult::Draw => {
                println!("This round is a draw.");
            }

            RoundResult::Player => {
                println!("The player won this round.");
                player_score += 1;
            }

            RoundResult::Computer => {
                println!("The computer won this round.");
                computer_score += 1;
            }
        }

        // Показываем счёт.
        show_score(player_score, computer_score);

        // Если игра ещё не закончена,
        // небольшая пауза перед следующим раундом.
        if player_score < 2 && computer_score < 2 {
            thread::sleep(Duration::from_millis(1200));
        }
    }

    // Финальный результат.
    println!();

    if player_score == 2 {
        println!("THE PLAYER WINS THE GAME!");
    } else {
        println!("THE COMPUTER WINS THE GAME!");
    }

    println!();
}

// Спрашиваем, хочет ли пользователь сыграть ещё раз.
fn wants_to_play_again() -> bool {
    let mut input = String::new();

    print!("Play again? [y/n]: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        return true;
    }

    false
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    loop {
        play_game();

        if wants_to_play_again() == false {
            break;
        }
    }

    println!();
    println!("Thanks for playing.");
}
