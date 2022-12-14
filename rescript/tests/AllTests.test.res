open Zora

// TODO: Why test order matters?
zoraBlock("all tests", t => {
  t->equal(Day01.part02(Utils.loadInput(1)), 213089, "Fail")
  t->equal(Day01.part01(Utils.loadInput(1)), 72718, "Fail")

  t->equal(Day02.part01(Utils.loadLines(2)), 9651, "Fail")
  t->equal(Day02.part02(Utils.loadLines(2)), 10560, "Fail")

  t->equal(Day03.part01(Utils.loadLines(3)), 7917, "Fail")
  t->equal(Day03.part02(Utils.loadLines(3)), 2585, "Fail")

  t->equal(Day04.part01(Utils.loadLines(4)), 441, "Fail")
  t->equal(Day04.part02(Utils.loadLines(4)), 861, "Fail")

  t->equal(Day05.part01(Utils.loadLines(5)), ["H", "B", "T", "M", "T", "B", "S", "D", "C"], "Fail")
  t->equal(Day05.part02(Utils.loadLines(5)), ["P", "Q", "T", "J", "R", "S", "H", "W", "S"], "Fail")

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
  t->equal(Day06.part02(Utils.loadInput(6)), 1623, "Fail")
})
