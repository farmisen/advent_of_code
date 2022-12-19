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
})
