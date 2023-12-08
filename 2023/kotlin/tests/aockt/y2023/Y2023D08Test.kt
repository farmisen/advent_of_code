package aockt.y2023

import io.github.jadarma.aockt.test.AdventDay
import io.github.jadarma.aockt.test.AdventSpec

@AdventDay(2023, 8, "Haunted Wasteland")
class Y2023D08Test : AdventSpec<Y2023D08>({

    val input1 = """RL                    

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)"""

    val input2 = """LLR
    
                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"""

    partOne {
        input1 shouldOutput 2
        input2 shouldOutput 6
    }


    val input3 = """LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)"""

    partTwo {
        input3 shouldOutput 6
    }
    test("debug") {
        solution.partTwo(input3)
    }

})

