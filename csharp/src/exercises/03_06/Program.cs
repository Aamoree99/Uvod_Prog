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

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // Возможные ходы и результаты раунда представлены перечислениями.
    enum Move { Rock, Scissors, Paper }
    enum RoundResult { Draw, Player, Computer }
    private static readonly Random RNG=new Random();
    // Возвращаем случайное число от 0 до 2 для выбора хода.
    static int RandomMove() { return RNG.Next(0,3); }
    // Читаем строку и убираем окружающие пробелы.
    static string ReadLine() { return (Console.ReadLine() ?? throw new InvalidOperationException("End of input")).Trim().ToLowerInvariant(); }
    // Приостанавливаем выполнение на заданное число миллисекунд.
    static void Pause(int ms) { System.Threading.Thread.Sleep(ms); }
    // ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
    static void ClearScreen() { Console.Write("\u001b[2J\u001b[1;1H"); Console.Out.Flush(); }
    // Возвращаем английское название выбранного хода.
    static string MoveText(Move move) {
        if (move==Move.Rock) return "Rock";
        if (move==Move.Scissors) return "Scissors";
        return "Paper";
    }
    // Возвращаем символ для выбранного хода.
    static string MoveEmoji(Move move) {
        if (move==Move.Rock) return "🪨";
        if (move==Move.Scissors) return "✂️";
        return "📄";
    }
    // Многострочные рисунки соответствуют исходной Rust-версии.
    static string MoveAscii(Move move) {
        if (move==Move.Rock) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
        if (move==Move.Scissors) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
        return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
    }
    // Три кадра с той же паузой 600 мс, что в Rust.
    static void AnimateMoves() {
        ClearScreen(); Console.WriteLine(""); Console.WriteLine("             ROCK!"); Console.WriteLine(""); Console.WriteLine("{0}", MoveAscii(Move.Rock)); Pause(600);
        ClearScreen(); Console.WriteLine(""); Console.WriteLine("             SCISSORS!"); Console.WriteLine(""); Console.WriteLine("{0}", MoveAscii(Move.Scissors)); Pause(600);
        ClearScreen(); Console.WriteLine(""); Console.WriteLine("             PAPER!"); Console.WriteLine(""); Console.WriteLine("{0}", MoveAscii(Move.Paper)); Pause(600);
    }
    // Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
    static Move? ReadPlayerMove() {
        Console.Write("Enter your move [rock / scissors / paper]: "); Console.Out.Flush();
        string input=ReadLine();
        if (input == "rock" || input == "r") return Move.Rock;
        if (input == "scissors" || input == "s") return Move.Scissors;
        if (input == "paper" || input == "p") return Move.Paper;
        return null;
    }
    // Случайное целое 0, 1 или 2 определяет ход компьютера.
    static Move ComputerMove() {
        int number=RandomMove();
        if (number==0) return Move.Rock;
        if (number==1) return Move.Scissors;
        return Move.Paper;
    }
    // Возвращаем исход раунда по ходам игрока и компьютера.
    static RoundResult GetRoundResult(Move player,Move computer) {
        if (player==computer) return RoundResult.Draw;
        if ((player==Move.Rock && computer==Move.Scissors) || (player==Move.Scissors && computer==Move.Paper) || (player==Move.Paper && computer==Move.Rock)) return RoundResult.Player;
        return RoundResult.Computer;
    }
    // Выводим текущие очки игрока и компьютера.
    static void ShowScore(int playerScore,int computerScore) {
        Console.WriteLine(""); Console.WriteLine("========================================");
        Console.WriteLine("        PLAYER {0} : {1} COMPUTER", playerScore, computerScore);
        Console.WriteLine("========================================"); Console.WriteLine("");
    }
    // Выводим названия, символы и рисунки обоих ходов.
    static void ShowMoves(Move player,Move computer) {
        Console.WriteLine(""); Console.WriteLine("PLAYER                         COMPUTER");
        Console.WriteLine("{0} {1}                     {2} {3}", MoveEmoji(player), MoveText(player), MoveText(computer), MoveEmoji(computer));
        Console.WriteLine(""); Console.WriteLine("PLAYER:"); Console.WriteLine("{0}", MoveAscii(player)); Console.WriteLine("COMPUTER:"); Console.WriteLine("{0}", MoveAscii(computer));
    }
    // Выводим правила игры и допустимые варианты ввода.
    static void ShowIntro() {
        ClearScreen(); Console.WriteLine("========================================"); Console.WriteLine("         ROCK - SCISSORS - PAPER");
        Console.WriteLine("========================================"); Console.WriteLine(""); Console.WriteLine("The first to win two rounds wins the game."); Console.WriteLine("");
        Console.WriteLine("You can enter:"); Console.WriteLine("  rock / r"); Console.WriteLine("  scissors / s"); Console.WriteLine("  paper / p"); Console.WriteLine("");
    }
    // Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
    static void PlayGame() {
        int playerScore=0,computerScore=0;
        ShowIntro();
        while (playerScore < 2 && computerScore < 2) {
            Move player;
            while (true) {
                Move? move=ReadPlayerMove();
                if (move.HasValue) { player=move.Value; break; }
                Console.WriteLine(""); Console.WriteLine("Invalid move."); Console.WriteLine("");
            }
            Move computer=ComputerMove();
            AnimateMoves(); ClearScreen(); ShowMoves(player,computer);
            RoundResult result=GetRoundResult(player,computer);
            if (result==RoundResult.Draw) Console.WriteLine("This round is a draw.");
            else if (result==RoundResult.Player) { Console.WriteLine("The player won this round."); playerScore += 1; }
            else { Console.WriteLine("The computer won this round."); computerScore += 1; }
            ShowScore(playerScore,computerScore);
            if (playerScore < 2 && computerScore < 2) Pause(1200);
        }
        Console.WriteLine("");
        if (playerScore==2) Console.WriteLine("THE PLAYER WINS THE GAME!");
        else Console.WriteLine("THE COMPUTER WINS THE GAME!");
        Console.WriteLine("");
    }
    // Читаем ответ: y или yes означает повторную игру.
    static bool WantsToPlayAgain() {
        Console.Write("Play again? [y/n]: "); Console.Out.Flush();
        string input=ReadLine();
        return input == "y" || input == "yes";
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        while (true) {
            PlayGame();
            if (!WantsToPlayAgain()) break;
        }
        Console.WriteLine(""); Console.WriteLine("Thanks for playing.");
    }
}
