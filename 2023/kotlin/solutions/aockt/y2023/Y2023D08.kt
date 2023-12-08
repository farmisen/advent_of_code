package aockt.y2023

import kotlin.math.*

import io.github.jadarma.aockt.core.Solution

object Y2023D08 : Solution {

    data class Node(val id: String, val left: String, val right: String) {
        companion object {
            fun of(input: String): Node {
                val id = input.substringBefore("=").trim()
                val left = input.substringAfter("(").substringBefore(",").trim()
                val right = input.substringAfter(",").substringBefore(")").trim()
                return Node(id, left, right)
            }
        }
    }

    data class State(
        val instructions: List<Char>,
        val nodes: Map<String, Node>,
        var currentNodeId: String,
        var totalSteps: Long = 0,
        var nextInstructionIdx: Int = 0
    ) {
        fun step() {
            val instruction = instructions.elementAt(nextInstructionIdx)
            val currentNode = nodes[currentNodeId]
            require(currentNode != null)
            currentNodeId = if (instruction == 'L') currentNode.left else currentNode.right
            nextInstructionIdx = (nextInstructionIdx + 1) % instructions.size
            totalSteps += 1
        }
    }

    private fun parseInput(input: String): Pair<List<Char>, Map<String, Node>> {
        val instructions = input.substringBefore("\n").trim().toCharArray().toList()
        val nodes = input.substringAfter("\n\n").split("\n").map { Node.of(it) }.groupBy { it.id }
            .mapValues { (_, list) -> list.single() }
        return Pair(instructions, nodes)
    }


    private fun gcd(a: Long, b: Long): Long {
        if (b == 0L) return a
        return gcd(b, a % b)
    }

    private fun lcm(a: Long, b: Long): Long {
        return a * (b / gcd(a, b))
    }

    override fun partOne(input: String): Long {
        val (instructions, nodes) = parseInput(input)

        val state = State(instructions, nodes, "AAA")
        while (state.currentNodeId != "ZZZ") {
            state.step()
        }
        return state.totalSteps
    }

    override fun partTwo(input: String): Long {
        val (instructions, nodes) = parseInput(input)

        val startingNodeIds = nodes.keys.filter { it.endsWith('A') }
        val states = startingNodeIds.map { State(instructions, nodes, it) }

        val distances = states.map { state ->
            while (!state.currentNodeId.endsWith('Z')) {
                state.step()
            }
            state.totalSteps
        }

//        distance from each **A to **Z is == to distance from each **Z loop
//        val loops =  states.zip(states.map {it.currentNodeId}).map( ) {
//                (state, endingNode) ->
//            do {
//                state.step()
//            } while (!state.currentNodeId.endsWith(endingNode))
//            state.totalSteps
//        }
//        require(loops.zip(distances).map {(l,d) -> l - d} == distances)

        return distances.reduce { a, b -> lcm(a, b) }
    }
}

