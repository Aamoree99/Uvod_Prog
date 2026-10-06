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
enum Tah {
    Kamen,
    Nuzky,
    Papir,
}

// Возможные результаты раунда.
enum Vysledek {
    Nerozhodne,
    Hrac,
    Pocitac,
}

// Очищаем терминал.
//
// \x1B[2J     -> очистить экран
// \x1B[1;1H   -> переместить курсор в левый верхний угол
fn vycisti_obrazovku() {
    print!("\x1B[2J\x1B[1;1H");

    // print! сам по себе может не вывести текст сразу,
    // поэтому принудительно очищаем буфер вывода.
    io::stdout().flush().unwrap();
}

// Возвращаем текстовое название хода.
fn tah_na_text(tah: Tah) -> &'static str {
    match tah {
        Tah::Kamen => "Kamen",
        Tah::Nuzky => "Nuzky",
        Tah::Papir => "Papir",
    }
}

// Возвращаем emoji для хода.
fn tah_emoji(tah: Tah) -> &'static str {
    match tah {
        Tah::Kamen => "🪨",
        Tah::Nuzky => "✂️",
        Tah::Papir => "📄",
    }
}

// ASCII-art для выбранного хода.
//
// Функция возвращает строку &'static str,
// потому что все картинки заранее записаны прямо в программе.
fn tah_ascii(tah: Tah) -> &'static str {
    match tah {
        Tah::Kamen => {
            r#"
    _______
---'   ____)
      (_____)
      (_____)
      (____)
---.__(___)
"#
        }

        Tah::Nuzky => {
            r#"
    _______
---'   ____)____
          ______)
       __________)
      (____)
---.__(___)
"#
        }

        Tah::Papir => {
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
// KAMEN
// NUZKY
// PAPIR
//
// С паузой между кадрами.
fn animace() {
    vycisti_obrazovku();

    println!();
    println!("             KAMEN!");
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

    vycisti_obrazovku();

    println!();
    println!("             NUZKY!");
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

    vycisti_obrazovku();

    println!();
    println!("             PAPIR!");
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
// Some(Tah) -> если ввод правильный
// None      -> если неправильный
fn nacti_tah_hrace() -> Option<Tah> {
    let mut vstup = String::new();

    print!("Zadejte svuj tah [kamen / nuzky / papir]: ");

    // Чтобы prompt появился до ожидания ввода.
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut vstup)
        .expect("Chyba pri cteni vstupu");

    let vstup = vstup.trim().to_lowercase();

    match vstup.as_str() {
        // Можно писать полное слово.
        "kamen" => Some(Tah::Kamen),
        "nuzky" => Some(Tah::Nuzky),
        "papir" => Some(Tah::Papir),

        // И короткий вариант.
        "k" => Some(Tah::Kamen),
        "n" => Some(Tah::Nuzky),
        "p" => Some(Tah::Papir),

        // Всё остальное неверно.
        _ => None,
    }
}

// Компьютер случайно выбирает ход.
fn tah_pocitace() -> Tah {
    // Генерируем число:
    // 0
    // 1
    // 2
    let cislo = rand::thread_rng().gen_range(0..3);

    match cislo {
        0 => Tah::Kamen,
        1 => Tah::Nuzky,
        _ => Tah::Papir,
    }
}

// Определяем результат раунда.
fn vysledek_kola(hrac: Tah, pocitac: Tah) -> Vysledek {
    // match можно делать сразу по двум значениям.
    match (hrac, pocitac) {
        // Одинаковые ходы = ничья.
        (Tah::Kamen, Tah::Kamen) => Vysledek::Nerozhodne,
        (Tah::Nuzky, Tah::Nuzky) => Vysledek::Nerozhodne,
        (Tah::Papir, Tah::Papir) => Vysledek::Nerozhodne,

        // Победы игрока.
        (Tah::Kamen, Tah::Nuzky) => Vysledek::Hrac,
        (Tah::Nuzky, Tah::Papir) => Vysledek::Hrac,
        (Tah::Papir, Tah::Kamen) => Vysledek::Hrac,

        // Все остальные варианты = победа компьютера.
        _ => Vysledek::Pocitac,
    }
}

// Красиво показываем текущий счёт.
fn zobraz_skore(skore_hrac: i32, skore_pocitac: i32) {
    println!();
    println!("========================================");
    println!("        HRAC {} : {} POCITAC", skore_hrac, skore_pocitac);
    println!("========================================");
    println!();
}

// Показываем выбор игрока и компьютера.
fn zobraz_tahy(hrac: Tah, pocitac: Tah) {
    println!();
    println!("HRAC                         POCITAC");
    println!(
        "{} {}                     {} {}",
        tah_emoji(hrac),
        tah_na_text(hrac),
        tah_na_text(pocitac),
        tah_emoji(pocitac)
    );

    println!();

    // Пока ASCII выводим один под другим.
    //
    // Это проще и надёжнее, чем пытаться
    // склеивать многострочные строки по горизонтали.
    println!("HRAC:");
    println!("{}", tah_ascii(hrac));

    println!("POCITAC:");
    println!("{}", tah_ascii(pocitac));
}

// Главное меню перед игрой.
fn zobraz_uvod() {
    vycisti_obrazovku();

    println!("========================================");
    println!("         KAMEN - NUZKY - PAPIR");
    println!("========================================");
    println!();
    println!("Hrajeme na dve vitezna kola.");
    println!();
    println!("Muzete zadat:");
    println!("  kamen / k");
    println!("  nuzky / n");
    println!("  papir / p");
    println!();
}

// Одна целая игра до двух побед.
fn hraj_hru() {
    let mut skore_hrac = 0;
    let mut skore_pocitac = 0;

    zobraz_uvod();

    // Игра продолжается,
    // пока кто-то не získá 2 победы.
    while skore_hrac < 2 && skore_pocitac < 2 {
        let hrac;

        // Запрашиваем ввод,
        // пока пользователь не введёт допустимый ход.
        loop {
            match nacti_tah_hrace() {
                Some(tah) => {
                    hrac = tah;
                    break;
                }

                None => {
                    println!();
                    println!("Tento tah je neplatny.");
                    println!();
                }
            }
        }

        // Компьютер выбирает ход.
        let pocitac = tah_pocitace();

        // Показываем небольшую анимацию.
        animace();

        // Очищаем экран после анимации
        // и показываем реальные ходы.
        vycisti_obrazovku();

        zobraz_tahy(hrac, pocitac);

        // Определяем победителя.
        match vysledek_kola(hrac, pocitac) {
            Vysledek::Nerozhodne => {
                println!("Toto kolo je nerozhodne.");
            }

            Vysledek::Hrac => {
                println!("Toto kolo vyhral hrac.");
                skore_hrac += 1;
            }

            Vysledek::Pocitac => {
                println!("Toto kolo vyhral pocitac.");
                skore_pocitac += 1;
            }
        }

        // Показываем счёт.
        zobraz_skore(skore_hrac, skore_pocitac);

        // Если игра ещё не закончена,
        // небольшая пауза перед следующим раундом.
        if skore_hrac < 2 && skore_pocitac < 2 {
            thread::sleep(Duration::from_millis(1200));
        }
    }

    // Финальный результат.
    println!();

    if skore_hrac == 2 {
        println!("CELKOVYM VITEZEM JE HRAC!");
    } else {
        println!("CELKOVYM VITEZEM JE POCITAC!");
    }

    println!();
}

// Спрашиваем, хочет ли пользователь сыграть ещё раз.
fn chce_hrat_znovu() -> bool {
    let mut vstup = String::new();

    print!("Hrat znovu? [a/n]: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut vstup)
        .expect("Chyba pri cteni vstupu");

    let vstup = vstup.trim().to_lowercase();

    if vstup == "a" || vstup == "ano" {
        return true;
    }

    false
}

fn main() {
    loop {
        hraj_hru();

        if chce_hrat_znovu() == false {
            break;
        }
    }

    println!();
    println!("Diky za hru.");
}
