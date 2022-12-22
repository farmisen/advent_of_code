open Zora

// TODO: Why test order matters?
zoraBlock("all tests", t => {
  //
  // Day 01
  //
  t->equal(Day01.part02(Utils.loadInput(1)), 213089, "Fail")
  t->equal(Day01.part01(Utils.loadInput(1)), 72718, "Fail")

  //
  // Day 02
  //
  t->equal(Day02.part01(Utils.loadLines(2)), 9651, "Fail")
  t->equal(Day02.part02(Utils.loadLines(2)), 10560, "Fail")

  //
  // Day 03
  //
  t->equal(Day03.part01(Utils.loadLines(3)), 7917, "Fail")
  t->equal(Day03.part02(Utils.loadLines(3)), 2585, "Fail")

  //
  // Day 04
  //
  t->equal(Day04.part01(Utils.loadLines(4)), 441, "Fail")
  t->equal(Day04.part02(Utils.loadLines(4)), 861, "Fail")

  //
  // Day 05
  //
  t->equal(Day05.part01(Utils.loadLines(5)), ["H", "B", "T", "M", "T", "B", "S", "D", "C"], "Fail")
  t->equal(Day05.part02(Utils.loadLines(5)), ["P", "Q", "T", "J", "R", "S", "H", "W", "S"], "Fail")

  //
  // Day 06
  //
  t->equal(Day06.part01("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5, "Fail")
  t->equal(Day06.part01("nppdvjthqldpwncqszvftbrmjlhg"), 6, "Fail")
  t->equal(Day06.part01("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10, "Fail")
  t->equal(Day06.part01("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11, "Fail")
  t->equal(Day06.part01(Utils.loadInput(6)), 1623, "Fail")
  t->equal(Day06.part02("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19, "Fail")
  t->equal(Day06.part02("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23, "Fail")
  t->equal(Day06.part02("nppdvjthqldpwncqszvftbrmjlhg"), 23, "Fail")
  t->equal(Day06.part02("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29, "Fail")
  t->equal(Day06.part02("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26, "Fail")
  t->equal(Day06.part02(Utils.loadInput(6)), 3774, "Fail")

  //
  // Day 07
  //
  let fs = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`
  t->equal(Day07.part01(fs), 95437, "Fail")
  t->equal(Day07.part01(Utils.loadInput(7)), 1749646, "Fail")
  t->equal(Day07.part02(fs), 24933642, "Fail")
  t->equal(Day07.part02(Utils.loadInput(7)), 1498966, "Fail")

  //
  // Day 08
  //
  let trees = `30373
25512
65332
33549
35390`
  t->equal(Day08.part01(trees), 21, "Fail")
  t->equal(Day08.part01(Utils.loadInput(8)), 1703, "Fail")
  let trees = `30373
25512
65332
33549
35390`
  t->equal(Day08.part02(trees), 8, "Fail")
  t->equal(Day08.part02(Utils.loadInput(8)), 496650, "Fail")

  t->equal(Day08.countUntilBiggerOrEqual(3, [2, 3, 3]), 2, "Fail")
  t->equal(Day08.countUntilBiggerOrEqual(3, [2, 1, 2]), 3, "Fail")
  t->equal(Day08.countUntilBiggerOrEqual(3, [5, 1, 2]), 1, "Fail")

  //
  // Day 09
  //
  let moves = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`
  t->equal(Day09.part01(moves), 13, "Fail")
  t->equal(Day09.part01(Utils.loadInput(9)), 5513, "Fail")
  t->equal(Day09.part02(Utils.loadInput(9)), 2427, "Fail")

  //
  // Day 10
  //

  let program = `addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
`

  t->equal(Day10.part01(program), 13140, "Fail")
  t->equal(Day10.part01(Utils.loadInput(10)), 13180, "Fail")

  let crt = `##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....`

  t->equal(Day10.part02(program), crt, "Fail")

  let crt = `####.####.####..##..#..#...##..##..###..
#.......#.#....#..#.#..#....#.#..#.#..#.
###....#..###..#....####....#.#..#.###..
#.....#...#....#....#..#....#.####.#..#.
#....#....#....#..#.#..#.#..#.#..#.#..#.
####.####.#.....##..#..#..##..#..#.###..`

  t->equal(Day10.part02(Utils.loadInput(10)), crt, "Fail")

  //
  // Day 11
  //

  t->equal(
    Day11.collectNumbersFromString("  Starting items: 86, 67, 61, 96, 52, 63, 73"),
    [86, 67, 61, 96, 52, 63, 73],
    "Fail",
  )

  let monkeys = `Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
    `

  // t->equal(Day11.part01(monkeys), 10605, "Fail")
  // t->equal(Day11.part01(Utils.loadInput(11)), 100345, "Fail")

  t->equal(Day11.part02(monkeys), 2713310158, "Fail")



})
