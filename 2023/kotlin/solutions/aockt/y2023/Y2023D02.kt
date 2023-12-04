package aockt.y2023

import io.github.jadarma.aockt.core.Solution
import com.kotlinspirit.core.Rules.char
import com.kotlinspirit.core.Rules.int
object Y2023D02 : Solution {
    // Each pair consists of an integer for the game id and a list of mutable maps for the game sets.
    private fun parseInput(input: String): Sequence<Pair<Int, List<MutableMap<String, Int>>>> =
        // Split the input string by newline characters and map each line.
        input.splitToSequence('\n').map { line ->
            // Trim each line and split it by the colon character.
            line.trim().split(":").let { (name, sets) ->
                // Get the game id and map it to the sets
                name.split(" ")[1].toInt() to sets.split(";").map { set ->
                    // For each set (split by semicolons), create a mutable map.
                    set.trim().split(",").associate {
                        // Split each entry by space, where the first part is a number
                        // and the second part is a color. This forms key-value pairs in the map.
                        val (num, color) = it.trim().split(" ")
                        color to num.toInt()
                    }.toMutableMap()
                }
            }
        }

    private val bag = mapOf("red" to 12, "green" to 13, "blue" to 14)

    override fun partOne(input: String) = parseInput(input).sumOf { (id, sets) ->
        // Return the game id if no set contains more of a color than the same color count in 'bag', otherwise return 0.
        if (sets.any { set -> set.any { (color, count) -> bag[color]!! < count } }) 0 else id
    }

    override fun partTwo(input: String) = parseInput(input).sumOf { (_, sets) ->
        sets.fold(mutableMapOf<String, Int>()) { res, set ->
            // For each color in the set, update the count in 'res' to be the maximum
            // of the current count in 'res' and the count in the set.
            set.forEach { (color, count) -> res[color] = maxOf(res[color] ?: 0, count) }
            res
            // Reduce the map values (counts) by multiplying them together.
        }.values.reduce(Int::times)
    }
}
