package aockt.y2023

import io.github.jadarma.aockt.core.Solution

object Y2023D01 : Solution {
    private fun parseInput(input: String): List<String> = input.splitToSequence('\n').map { it.trim() }.toList()

    private val numWords = mapOf(
        "one" to "1",
        "1" to "1",
        "two" to "2",
        "2" to "2",
        "three" to "3",
        "3" to "3",
        "four" to "4",
        "4" to "4",
        "five" to "5",
        "6" to "5",
        "six" to "6",
        "6" to "6",
        "seven" to "7",
        "7" to "7",
        "eight" to "8",
        "8" to "8",
        "nine" to "9",
        "9" to "9",
        "zero" to "0",
        "0" to "0"
    )

    private fun replaceNumWords(str: String): String {
        // Create a map with the index of the last occurrence of each number word in the string as the key and the number word as the value.
        return numWords.keys.fold(mutableMapOf<Int, String>()) { res, numWord ->
            str.lastIndexOf(numWord).takeIf { it != -1 }?.let { res[it] = numWord }.let { res }
        }.let {
            // Find the highest index in the map (the last number word in the string).
            // Replace this last occurrence of the number word with its numeric representation.
            it.keys.maxOrNull()?.let { lastIndex ->
                val numWord = it.getValue(lastIndex)
                str.substring(0, lastIndex) + numWords.getValue(numWord) + str.substring(lastIndex + numWord.length)
            } ?: str
        }.let { newStr ->
            // Create a map with the index of the first occurrence of each number word in the string as the key and the number word as the value.
            numWords.keys.fold(mutableMapOf<Int, String>()) { res, numWord ->
                str.indexOf(numWord).takeIf { it != -1 }?.let { res[it] = numWord }.let { res }
            }.let {
                // Find the smallest index in this map (the first number word in the modified string).
                // Replace this first occurrence of the number word with its numeric representation.
                it.keys.minOrNull()?.let { firstIndex ->
                    val numWord = it.getValue(firstIndex)
                    newStr.replaceFirst(numWord, numWords.getValue(numWord))
                } ?: newStr
            }
        }
    }

    override fun partOne(input: String) =
        parseInput(input)
            .asSequence()
            .map { it.replace(Regex("\\D"), "") }
            .sumOf { "${it.first()}${it.last()}".toInt() }

    override fun partTwo(input: String): Int =
        parseInput(input)
            .asSequence()
            .map { replaceNumWords(it).replace(Regex("\\D"), "") }
            .sumOf { "${it.first()}${it.last()}".toInt() }

}






