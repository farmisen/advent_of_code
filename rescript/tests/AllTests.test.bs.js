// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Zora from "zora";
import * as Day01 from "../src/Day01.bs.js";
import * as Day02 from "../src/Day02.bs.js";
import * as Day03 from "../src/Day03.bs.js";
import * as Day04 from "../src/Day04.bs.js";
import * as Day05 from "../src/Day05.bs.js";
import * as Day06 from "../src/Day06.bs.js";
import * as Day07 from "../src/Day07.bs.js";
import * as Day08 from "../src/Day08.bs.js";
import * as Day09 from "../src/Day09.bs.js";
import * as Day10 from "../src/Day10.bs.js";
import * as Day11 from "../src/Day11.bs.js";
import * as Utils from "../src/Utils.bs.js";

Zora.test("all tests", (function (t) {
        t.equal(Day01.part02(Utils.loadInput(1)), 213089, "Fail");
        t.equal(Day01.part01(Utils.loadInput(1)), 72718, "Fail");
        t.equal(Day02.part01(Utils.loadLines(2)), 9651, "Fail");
        t.equal(Day02.part02(Utils.loadLines(2)), 10560, "Fail");
        t.equal(Day03.part01(Utils.loadLines(3)), 7917, "Fail");
        t.equal(Day03.part02(Utils.loadLines(3)), 2585, "Fail");
        t.equal(Day04.part01(Utils.loadLines(4)), 441, "Fail");
        t.equal(Day04.part02(Utils.loadLines(4)), 861, "Fail");
        t.equal(Day05.part01(Utils.loadLines(5)), [
              "H",
              "B",
              "T",
              "M",
              "T",
              "B",
              "S",
              "D",
              "C"
            ], "Fail");
        t.equal(Day05.part02(Utils.loadLines(5)), [
              "P",
              "Q",
              "T",
              "J",
              "R",
              "S",
              "H",
              "W",
              "S"
            ], "Fail");
        t.equal(Day06.part01("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5, "Fail");
        t.equal(Day06.part01("nppdvjthqldpwncqszvftbrmjlhg"), 6, "Fail");
        t.equal(Day06.part01("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10, "Fail");
        t.equal(Day06.part01("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11, "Fail");
        t.equal(Day06.part01(Utils.loadInput(6)), 1623, "Fail");
        t.equal(Day06.part02("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19, "Fail");
        t.equal(Day06.part02("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23, "Fail");
        t.equal(Day06.part02("nppdvjthqldpwncqszvftbrmjlhg"), 23, "Fail");
        t.equal(Day06.part02("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29, "Fail");
        t.equal(Day06.part02("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26, "Fail");
        t.equal(Day06.part02(Utils.loadInput(6)), 3774, "Fail");
        var fs = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        t.equal(Day07.part01(fs), 95437, "Fail");
        t.equal(Day07.part01(Utils.loadInput(7)), 1749646, "Fail");
        t.equal(Day07.part02(fs), 24933642, "Fail");
        t.equal(Day07.part02(Utils.loadInput(7)), 1498966, "Fail");
        t.equal(Day08.part01("30373\n25512\n65332\n33549\n35390"), 21, "Fail");
        t.equal(Day08.part01(Utils.loadInput(8)), 1703, "Fail");
        t.equal(Day08.part02("30373\n25512\n65332\n33549\n35390"), 8, "Fail");
        t.equal(Day08.part02(Utils.loadInput(8)), 496650, "Fail");
        t.equal(Day08.countUntilBiggerOrEqual(3, [
                  2,
                  3,
                  3
                ]), 2, "Fail");
        t.equal(Day08.countUntilBiggerOrEqual(3, [
                  2,
                  1,
                  2
                ]), 3, "Fail");
        t.equal(Day08.countUntilBiggerOrEqual(3, [
                  5,
                  1,
                  2
                ]), 1, "Fail");
        t.equal(Day09.part01("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13, "Fail");
        t.equal(Day09.part01(Utils.loadInput(9)), 5513, "Fail");
        t.equal(Day09.part02(Utils.loadInput(9)), 2427, "Fail");
        var program = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n";
        t.equal(Day10.part01(program), 13140, "Fail");
        t.equal(Day10.part01(Utils.loadInput(10)), 13180, "Fail");
        t.equal(Day10.part02(program), "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....", "Fail");
        t.equal(Day10.part02(Utils.loadInput(10)), "####.####.####..##..#..#...##..##..###..\n#.......#.#....#..#.#..#....#.#..#.#..#.\n###....#..###..#....####....#.#..#.###..\n#.....#...#....#....#..#....#.####.#..#.\n#....#....#....#..#.#..#.#..#.#..#.#..#.\n####.####.#.....##..#..#..##..#..#.###..", "Fail");
        t.equal(Day11.collectNumbersFromString("  Starting items: 86, 67, 61, 96, 52, 63, 73"), [
              86,
              67,
              61,
              96,
              52,
              63,
              73
            ], "Fail");
        t.equal(Day11.part02("Monkey 0:\n    Starting items: 79, 98\n    Operation: new = old * 19\n    Test: divisible by 23\n      If true: throw to monkey 2\n      If false: throw to monkey 3\n\n  Monkey 1:\n    Starting items: 54, 65, 75, 74\n    Operation: new = old + 6\n    Test: divisible by 19\n      If true: throw to monkey 2\n      If false: throw to monkey 0\n\n  Monkey 2:\n    Starting items: 79, 60, 97\n    Operation: new = old * old\n    Test: divisible by 13\n      If true: throw to monkey 1\n      If false: throw to monkey 3\n\n  Monkey 3:\n    Starting items: 74\n    Operation: new = old + 3\n    Test: divisible by 17\n      If true: throw to monkey 0\n      If false: throw to monkey 1\n    "), -1581657138, "Fail");
      }));

export {
  
}
/*  Not a pure module */
