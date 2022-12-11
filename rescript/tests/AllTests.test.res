open Zora

// TODO: Why test order matters?
zoraBlock("all tests", t => {
  t->equal(Day01.part02(), 213089, "Fail")
  t->equal(Day01.part01(), 72718, "Fail")
  t->equal(Day02.part01(), 9651, "Fail")
  t->equal(Day02.part02(), 10560, "Fail")

})

