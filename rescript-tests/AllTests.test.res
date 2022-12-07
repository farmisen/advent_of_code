open Zora

zoraBlock("testing_works", t => {
  t->equal(Day01Part01.run(), 72718, "Fail")
})

zoraBlock("testing_works", t => {
  t->equal(Day01Part02.run(), 213089, "Fail")
})

