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

#include <iostream>
#include <climits>

// Номиналы положительны, расположены по убыванию и включают единицу.
// Возвращаем число монет после жадного выбора от крупных к мелким.
int pay_with_coins(const int coins[], int amount) {
    int remainder = amount, count = 0, i = 0;
    while (remainder > 0) {
        int coin_count = remainder / coins[i];
        count += coin_count;
        remainder -= coin_count * coins[i];
        i += 1;
    }
    return count;
}
// Перебираем количества каждого номинала начиная с index.
// Большое число обозначает невозможность собрать остаток суммы.
int optimal_coin_count(const int coins[], int length, int index, int amount) {
    if (amount == 0) return 0;
    if (index >= length) return INT_MAX / 2;
    int coin = coins[index], count = amount / coin, minimum = INT_MAX / 2;
    while (count >= 0) {
        int remainder = amount - count * coin;
        int remaining_count = optimal_coin_count(coins, length, index + 1, remainder);
        if (remaining_count < INT_MAX / 2) {
            int total = count + remaining_count;
            if (total < minimum) minimum = total;
        }
        count -= 1;
    }
    return minimum;
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    const int coins[] = {50, 20, 10, 5, 2, 1};
    std::cout << "PART A\n\n";
    std::cout << "The minimum number of coins needed to pay 97 CZK is " << pay_with_coins(coins, 97) << ".\n";
    std::cout << "The minimum number of coins needed to pay 88 CZK is " << pay_with_coins(coins, 88) << ".\n";
    std::cout << "The minimum number of coins needed to pay 26 CZK is " << pay_with_coins(coins, 26) << ".\n";
    // Для суммы 6 жадный выбор даёт 4+1+1, оптимальный — 3+3.
    const int strange_coins[] = {4, 3, 1};
    int strange_amount = 6;
    std::cout << "\nPART B\n\nCoin set: [4, 3, 1]\nAmount: " << strange_amount << '\n';
    std::cout << "Greedy algorithm: " << pay_with_coins(strange_coins, strange_amount) << " coins\n";
    std::cout << "Optimal solution: " << optimal_coin_count(strange_coins, 3, 0, strange_amount) << " coins\n";
}
