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

#include <iostream>
#include <string>
#include <optional>
#include <random>
#include <thread>
#include <chrono>
#include <cctype>
#include <stdexcept>

// Возможные ходы и результаты раунда представлены перечислениями.
enum class Move { Rock, Scissors, Paper };
enum class RoundResult { Draw, Player, Computer };
// Возвращаем случайное число от 0 до 2 для выбора хода.
static int random_move() {
    static std::mt19937 rng(std::random_device{}());
    return std::uniform_int_distribution<int>(0,2)(rng);
}
// Читаем строку и убираем окружающие пробелы.
static std::string read_line() {
    std::string value;
    if (!std::getline(std::cin,value)) throw std::runtime_error("End of input");
    auto begin=value.find_first_not_of(" \t\r\n");
    if (begin==std::string::npos) return "";
    value=value.substr(begin,value.find_last_not_of(" \t\r\n")-begin+1);
    std::size_t i=0;
    while (i < value.size()) { value[i]=static_cast<char>(std::tolower(static_cast<unsigned char>(value[i]))); ++i; }
    return value;
}
// Приостанавливаем выполнение на заданное число миллисекунд.
static void pause(int ms) { std::this_thread::sleep_for(std::chrono::milliseconds(ms)); }
// ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
static void clear_screen() { std::cout << "\x1B[2J\x1B[1;1H" << std::flush; }
// Возвращаем английское название выбранного хода.
static std::string move_text(Move move) {
    if (move==Move::Rock) return "Rock";
    if (move==Move::Scissors) return "Scissors";
    return "Paper";
}
// Возвращаем символ для выбранного хода.
static std::string move_emoji(Move move) {
    if (move==Move::Rock) return "🪨";
    if (move==Move::Scissors) return "✂️";
    return "📄";
}
// Многострочные рисунки соответствуют исходной Rust-версии.
static std::string move_ascii(Move move) {
    if (move==Move::Rock) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
    if (move==Move::Scissors) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
    return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
}
// Три кадра с той же паузой 600 мс, что в Rust.
static void animate_moves() {
    clear_screen(); std::cout << '\n'; std::cout << "             ROCK!" << '\n'; std::cout << '\n'; std::cout << move_ascii(Move::Rock) << '\n'; pause(600);
    clear_screen(); std::cout << '\n'; std::cout << "             SCISSORS!" << '\n'; std::cout << '\n'; std::cout << move_ascii(Move::Scissors) << '\n'; pause(600);
    clear_screen(); std::cout << '\n'; std::cout << "             PAPER!" << '\n'; std::cout << '\n'; std::cout << move_ascii(Move::Paper) << '\n'; pause(600);
}
// Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
static std::optional<Move> read_player_move() {
    std::cout << "Enter your move [rock / scissors / paper]: "; std::cout.flush();
    std::string input=read_line();
    if (input == "rock" || input == "r") return Move::Rock;
    if (input == "scissors" || input == "s") return Move::Scissors;
    if (input == "paper" || input == "p") return Move::Paper;
    return std::nullopt;
}
// Случайное целое 0, 1 или 2 определяет ход компьютера.
static Move computer_move() {
    int number=random_move();
    if (number==0) return Move::Rock;
    if (number==1) return Move::Scissors;
    return Move::Paper;
}
// Возвращаем исход раунда по ходам игрока и компьютера.
static RoundResult round_result(Move player,Move computer) {
    if (player==computer) return RoundResult::Draw;
    if ((player==Move::Rock && computer==Move::Scissors) || (player==Move::Scissors && computer==Move::Paper) || (player==Move::Paper && computer==Move::Rock)) return RoundResult::Player;
    return RoundResult::Computer;
}
// Выводим текущие очки игрока и компьютера.
static void show_score(int player_score,int computer_score) {
    std::cout << '\n'; std::cout << "========================================" << '\n';
    std::cout << "        PLAYER " << player_score << " : " << computer_score << " COMPUTER" << '\n';
    std::cout << "========================================" << '\n'; std::cout << '\n';
}
// Выводим названия, символы и рисунки обоих ходов.
static void show_moves(Move player,Move computer) {
    std::cout << '\n'; std::cout << "PLAYER                         COMPUTER" << '\n';
    // Возвращаем символ для выбранного хода.
    std::cout << move_emoji(player) << " " << move_text(player) << "                     " << move_text(computer) << " " << move_emoji(computer) << '\n';
    std::cout << '\n'; std::cout << "PLAYER:" << '\n'; std::cout << move_ascii(player) << '\n'; std::cout << "COMPUTER:" << '\n'; std::cout << move_ascii(computer) << '\n';
}
// Выводим правила игры и допустимые варианты ввода.
static void show_intro() {
    clear_screen(); std::cout << "========================================" << '\n'; std::cout << "         ROCK - SCISSORS - PAPER" << '\n';
    std::cout << "========================================" << '\n'; std::cout << '\n'; std::cout << "The first to win two rounds wins the game." << '\n'; std::cout << '\n';
    std::cout << "You can enter:" << '\n'; std::cout << "  rock / r" << '\n'; std::cout << "  scissors / s" << '\n'; std::cout << "  paper / p" << '\n'; std::cout << '\n';
}
// Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
static void play_game() {
    int player_score=0,computer_score=0;
    show_intro();
    while (player_score < 2 && computer_score < 2) {
        Move player;
        while (true) {
            std::optional<Move> move=read_player_move();
            if (move.has_value()) { player=move.value(); break; }
            std::cout << '\n'; std::cout << "Invalid move." << '\n'; std::cout << '\n';
        }
        Move computer=computer_move();
        animate_moves(); clear_screen(); show_moves(player,computer);
        RoundResult result=round_result(player,computer);
        if (result==RoundResult::Draw) std::cout << "This round is a draw." << '\n';
        else if (result==RoundResult::Player) { std::cout << "The player won this round." << '\n'; player_score += 1; }
        else { std::cout << "The computer won this round." << '\n'; computer_score += 1; }
        show_score(player_score,computer_score);
        if (player_score < 2 && computer_score < 2) pause(1200);
    }
    std::cout << '\n';
    if (player_score==2) std::cout << "THE PLAYER WINS THE GAME!" << '\n';
    else std::cout << "THE COMPUTER WINS THE GAME!" << '\n';
    std::cout << '\n';
}
// Читаем ответ: y или yes означает повторную игру.
static bool wants_to_play_again() {
    std::cout << "Play again? [y/n]: "; std::cout.flush();
    std::string input=read_line();
    return input == "y" || input == "yes";
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    while (true) {
        play_game();
        if (!wants_to_play_again()) break;
    }
    std::cout << '\n'; std::cout << "Thanks for playing." << '\n';
}
