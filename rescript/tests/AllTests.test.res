open Zora

// TODO: Why test order matters?
zoraBlock("all tests", t => {
  t->equal(Day01.part02(), 213089, "Fail")
  t->equal(Day01.part01(), 72718, "Fail")
  t->equal(Day02.part01(), 9651, "Fail")
  t->equal(Day02.part02(), 10560, "Fail")
  t->equal(Day03.part01(), 7917, "Fail")
  t->equal(Day03.part02(), 2585, "Fail")
})
