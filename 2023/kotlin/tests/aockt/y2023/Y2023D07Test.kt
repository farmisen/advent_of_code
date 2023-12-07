package aockt.y2023

import io.github.jadarma.aockt.test.AdventDay
import io.github.jadarma.aockt.test.AdventSpec

@AdventDay(2023, 7, "Wait For It")
class Y2023D07Test : AdventSpec<Y2023D07>({

    val input =  """32T3K 765
                    T55J5 684
                    KK677 28
                    KTJJT 220
                    QQQJA 483"""

    test("debug") {
        solution.partOne(input)
    }


    partOne {
        input shouldOutput 6440
    }

    partTwo {

        input shouldOutput 5905
    }

})

// 246519197 too low
// 248105065

// 249239176 too low
// 249515436