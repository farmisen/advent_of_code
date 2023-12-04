package aockt.y2023

import io.github.jadarma.aockt.core.Solution

object Y2023D03 : Solution {
    class GridLocation(val x: Int, val y: Int) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (other == null || this::class != other::class) return false

            other as GridLocation

            if (x != other.x) return false
            if (y != other.y) return false

            return true
        }

        override fun hashCode(): Int {
            var result = x
            result = 31 * result + y
            return result
        }

        fun adjacentLocations(): List<GridLocation> {
            return listOf(
                GridLocation(x - 1, y - 1),
                GridLocation(x, y - 1),
                GridLocation(x + 1, y - 1),
                GridLocation(x - 1, y),
                GridLocation(x + 1, y),
                GridLocation(x - 1, y + 1),
                GridLocation(x, y + 1),
                GridLocation(x + 1, y + 1)
            )
        }
    }


    class Number {
        private val digitsLocations = mutableListOf<GridLocation>()
        private val digits = mutableListOf<Char>()
        fun addDigit(x: Int, y: Int, digit: Char) {
            digitsLocations.add(GridLocation(x, y))
            digits.add(digit)
        }

        fun getValue() = digits.joinToString("").toInt()

        fun adjacentLocations() = digitsLocations.flatMap { it.adjacentLocations() }.toSet()
    }

    private fun parseInput(input: String, partSymbols: List<Char>): Pair<List<Number>, List<GridLocation>> {
        return input.splitToSequence('\n').map { it.trim() }
            .foldIndexed(Pair(mutableListOf<Number>(), mutableListOf<GridLocation>())) { y, (num, sym), line ->
                line.toCharArray().foldIndexed<Triple<MutableList<Number>, MutableList<GridLocation>, Number?>>(
                    Triple(
                        num, sym, null
                    )
                ) { x, (numbers, symbols, number: Number?), c ->
                    var currentNumber = number
                    when {
                        c.isDigit() -> {
                            val n = currentNumber ?: Number()
                            n.addDigit(x, y, c)
                            currentNumber = n
                        }

                        c in partSymbols -> {
                            symbols.add(GridLocation(x, y))
                            currentNumber?.also { numbers.add(it) }
                            currentNumber = null
                        }

                        else -> {
                            currentNumber?.also { numbers.add(it) }
                            currentNumber = null
                        }

                    }
                    if (x == line.length - 1 && currentNumber != null) {
                        numbers.add(currentNumber)
                    }
                    Triple(numbers, symbols, currentNumber)
                }
                Pair(num, sym)
            }
    }

    override fun partOne(input: String): Int {
        val (numbers, symbols) = parseInput(
            input, listOf(
                '*', '$', '+', '-', '%', '#', '&', '=', '/', '@', ':'
            )
        )
        val symbolsSet = symbols.toSet()

        return numbers.sumOf { number ->
            if (number.adjacentLocations().any { it in symbolsSet }) number.getValue() else 0
        }
    }

    override fun partTwo(input: String): Int {
        val (numbers, stars) = parseInput(
            input, listOf(
                '*'
            )
        )

        return stars.sumOf { star ->
            numbers.asSequence().mapNotNull { number ->
                number.takeIf { star in it.adjacentLocations() }?.getValue()
            }.takeIf { it.count() == 2 }?.reduce(Int::times) ?: 0
        }
    }
}

