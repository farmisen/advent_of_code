package aockt.y2023

import kotlin.math.*

import io.github.jadarma.aockt.core.Solution

object Y2023D06 : Solution {

    private fun roundUp(n: Double) = if (n % 1.0 != 0.0) ceil(n).toLong() else n.toLong() + 1
    private fun roundDown(n: Double) = if (n % 1.0 != 0.0) floor(n).toLong() else n.toLong() - 1

    private fun quadraticDiff(a: Double, b: Double) =
        sqrt(a * a - 4 * b).let { s -> roundDown(0.5 * (a + s)) - roundUp(0.5 * (a - s)) }

    private fun parseInput(input: String) = Pair(
        input.substringBefore('\n').substringAfter(':').trim(), input.substringAfter('\n').substringAfter(':').trim()
    )

    override fun partOne(input: String): Long {
        val (times, distances) = parseInput(input)

        return times.split(" ").mapNotNull { it.toDoubleOrNull() }
            .zip(distances.split(" ").mapNotNull { it.toDoubleOrNull() }).map { (a, b) ->
                quadraticDiff(a, b) + 1
            }.reduce(Long::times)
    }

    override fun partTwo(input: String) =
        parseInput(input).toList().map { it.replace(" ", "").toDouble() }.let { (a, b) -> quadraticDiff(a, b) + 1}
}

