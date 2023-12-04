package aockt.y2023

import io.github.jadarma.aockt.core.Solution

object Y2023D04 : Solution {
    data class Card(val id: Int, val winningNumbers: List<Int>, val cardNumbers: List<Int>, var count: Int = 1) {

        companion object {
            fun of(input: String): Card {
                val id = input.substringAfter("Card").substringBefore(":").trim().toInt()
                val (winningNumbers, cardNumbers) = input.substringAfter(":").split("|", limit = 2)
                    .map { numbers -> numbers.trim().split(Regex("\\s+")).map { it.toInt() } }
                return Card(id, winningNumbers, cardNumbers)
            }
        }

        private fun winnings() = winningNumbers.intersect(cardNumbers.toSet()).size

        fun wonCards() = (id + 1..id + winnings()).toList()

        fun wonPoints(): Int {
            return 1.shl(winnings() - 1)
        }
    }

    private fun parseInput(input: String) = input.splitToSequence('\n').map { Card.of(it) }

    override fun partOne(input: String) = parseInput(input).sumOf { it.wonPoints() }

    override fun partTwo(input: String): Int {
        val cards = parseInput(input).toMutableList()

        cards.forEach { card ->
            val winnings = card.wonCards()
            winnings.forEach { winning ->
                cards.elementAt(winning - 1).count += card.count
            }
        }
        return cards.sumOf { it.count }
    }
}

