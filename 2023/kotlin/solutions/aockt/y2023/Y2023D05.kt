package aockt.y2023

import io.github.jadarma.aockt.core.Solution

object Y2023D05 : Solution {


    private fun intersectRanges(range1: LongRange, range2: LongRange): LongRange? {
        val start = maxOf(range1.first, range2.first)
        val end = minOf(range1.last, range2.last)

        return if (start <= end) {
            start..end
        } else {
            null // No intersection
        }
    }

    data class PlantingMap(val name: String, val ranges: Map<LongRange, LongRange>, var next: PlantingMap? = null) {
        companion object {
            fun of(block: String): PlantingMap {
                var name = block.trim().substringBefore(" ")
                var ranges = block.substringAfter('\n').split('\n').map {
                    var (dst, src, len) = it.split(" ", limit = 3).map { it.toLong() }
                    src..src + len - 1 to dst..dst + len - 1
                }.toMap()
                return PlantingMap(name, ranges)
            }
        }

        override fun toString(): String {
            return "$name:$ranges"
        }

        private fun toNext(value: Long): Long {
//            println("toNext: $value")
            return next?.pipe(value) ?: value
        }

        private fun toNext(range: LongRange): Long {
            return next?.pipe(range) ?: range.minOf { pipe(it) }
        }

        fun pipe(range: LongRange): Long {
            println("[$name] piped: $range")
            // get the intersections between the range and each src ranges
            val commonRanges = ranges.keys.mapNotNull { intersectRanges(it, range) }
            println("common:$commonRanges")

            return if (commonRanges.isEmpty()) {
                toNext(range)
            } else commonRanges.minOf { toNext(it) }
        }

        fun pipe(value: Long): Long {
            println("[$name] piped: $value")
            ranges.keys.forEach { srcRange ->
                if (value in srcRange) {
                    val dstRange = ranges.get(srcRange)!!
                    println("src:$srcRange dst:$dstRange")
                    return toNext(value + dstRange.start - srcRange.start)
                }
            }
            return toNext(value)
        }

    }


    private fun parseInput(input: String): Pair<List<Long>, List<PlantingMap>> {
        var seeds = input.substringBefore('\n').substringAfter(": ").split(" ").mapNotNull { it.toLongOrNull() }
        var maps = input.substringAfter("\n\n").split("\n\n").map { PlantingMap.of(it) }
        return Pair(seeds, maps)
    }

    override fun partOne(input: String): Long {
        return 0
        val (seeds, maps) = parseInput(input)
        val grouped = maps.groupBy { it.name }.mapValues { (_, list) -> list.single() }.toMutableMap()

        grouped["seed-to-soil"]!!.next = grouped["soil-to-fertilizer"]
        grouped["soil-to-fertilizer"]!!.next = grouped["fertilizer-to-water"]
        grouped["fertilizer-to-water"]!!.next = grouped["water-to-light"]
        grouped["water-to-light"]!!.next = grouped["light-to-temperature"]
        grouped["light-to-temperature"]!!.next = grouped["temperature-to-humidity"]
        grouped["temperature-to-humidity"]!!.next = grouped["humidity-to-location"]

        var first = grouped["seed-to-soil"]!!

        return seeds.map { first.pipe(it) }.sorted()[0]

    }

    override fun partTwo(input: String): Long {
        val (seeds, maps) = parseInput(input)

        if (seeds.size > 10) {
            return 0
        }
        val grouped = maps.groupBy { it.name }.mapValues { (_, list) -> list.single() }.toMutableMap()

        grouped["seed-to-soil"]!!.next = grouped["soil-to-fertilizer"]
        grouped["soil-to-fertilizer"]!!.next = grouped["fertilizer-to-water"]
        grouped["fertilizer-to-water"]!!.next = grouped["water-to-light"]
        grouped["water-to-light"]!!.next = grouped["light-to-temperature"]
        grouped["light-to-temperature"]!!.next = grouped["temperature-to-humidity"]
        grouped["temperature-to-humidity"]!!.next = grouped["humidity-to-location"]

        var first = grouped["seed-to-soil"]!!


        val seedRanges = seeds.chunked(2) { (start, len) -> start..start + len - 1 }

        return seedRanges.minOf { first.pipe(it) }

//        var min = Long.MAX_VALUE
//
//        for (r in seedRanges) {
//            for (seed in r) {
//                val res = first.pipe(seed)
//                min = minOf(min, res)
//            }
//        }

//        return seedRanges.map {range -> range.map { first.pipe(it) }.sorted()[0]}.sorted()[0]

//        return min
//        return seeds.map { first.pipe(it) }.sorted()[0]

    }
}

