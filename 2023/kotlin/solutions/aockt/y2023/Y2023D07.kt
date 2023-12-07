package aockt.y2023

import kotlin.math.*

import io.github.jadarma.aockt.core.Solution

object Y2023D07 : Solution {

    enum class HandType(val value: Int) {
        HighCard(1), OnePair(2), TwoPairs(3), ThreeOfAKind(4), FullHouse(5), FourOfAKind(6), FiveOfAKind(7)
    }

    data class Hand(val cards: List<Char>, val bid: Long) {
        companion object {
            fun of(input: String): Hand {
                val cards = input.trim().substringBefore(" ").toCharArray().toList()
                val bid = input.trim().substringAfter(" ").toLong()
                return Hand(cards, bid)
            }
        }

        private fun calcType(c: List<Char>): HandType {
            val cardsCount = c.groupingBy { it }.eachCount().values.sorted()

            return when (cardsCount) {
                listOf(5) -> HandType.FiveOfAKind
                listOf(1, 4) -> HandType.FourOfAKind
                listOf(2, 3) -> HandType.FullHouse
                listOf(1, 1, 3) -> HandType.ThreeOfAKind
                listOf(1, 2, 2) -> HandType.TwoPairs
                listOf(1, 1, 1, 2) -> HandType.OnePair
                else -> HandType.HighCard
            }
        }

        fun type(withJokers: Boolean = false): HandType {
            cards.toCharArray().let { cardsArray ->
                return if (withJokers) {
                    "23456789TQKA".asSequence().map { cardType ->
                        cardsArray.map { if (it == 'J') cardType else it }
                    }.map { cardList ->
                        calcType(cardList)
                    }.maxBy { it.value }
                } else calcType(cardsArray.toList())
            }
        }
    }

    private fun parseInput(input: String) = input.split("\n").map { Hand.of(it) }

    private fun makeCombinedComparator(order: Map<Char, Int>, withJoker: Boolean = false) =
        compareBy<Hand> { it.type(withJoker) }.thenComparator { h1, h2 ->
            h1.cards.zip(h2.cards).firstNotNullOfOrNull { (c1, c2) ->
                val order1 = order[c1] ?: 0
                val order2 = order[c2] ?: 0
                if (order1 != order2) order1.compareTo(order2) else null
            } ?: 0
        }

    private fun getAnswer(input: String, withJoker: Boolean): Long {
        val cardRankOrder = mapOf(
            '2' to 2,
            '3' to 3,
            '4' to 4,
            '5' to 5,
            '6' to 6,
            '7' to 7,
            '8' to 8,
            '9' to 9,
            'T' to 10,
            'J' to if (withJoker) 1 else 11,
            'Q' to 12,
            'K' to 13,
            'A' to 14,
        )
        val combinedComparator = makeCombinedComparator(cardRankOrder, withJoker)
        return parseInput(input).sortedWith(combinedComparator).mapIndexed { idx, hand -> (idx + 1) * hand.bid }.sum()
    }

    override fun partOne(input: String) = getAnswer(input, false)

    override fun partTwo(input: String) = getAnswer(input, true)
}
