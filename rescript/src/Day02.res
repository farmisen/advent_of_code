open Js.Array
open Utils

let rock = 1
let paper = 2
let scissor = 3
let lost = 0
let draw = 3
let won = 6

let part01 = () => {
  // A: rock
  // B: paper
  // C: scissor
  // X: rock
  // Y: paper
  // Z: scissor

  let scoring = Js.Dict.fromArray([
    ("A X", rock + draw),
    ("A Y", paper + won),
    ("A Z", scissor + lost),
    ("B X", rock + lost),
    ("B Y", paper + draw),
    ("B Z", scissor + won),
    ("C X", rock + won),
    ("C Y", paper + lost),
    ("C Z", scissor + draw),
  ])

  loadLines(2) |> reduce((accu, val) => accu + Js.Dict.unsafeGet(scoring, val), 0)
}

let part02 = () => {
  // A: rock
  // B: paper
  // C: scissor
  // X: lost
  // Y: draw
  // Z: win

  let scoring = Js.Dict.fromArray([
    ("A X", scissor + lost),
    ("A Y", rock + draw),
    ("A Z", paper + won),
    ("B X", rock + lost),
    ("B Y", paper + draw),
    ("B Z", scissor + won),
    ("C X", paper + lost),
    ("C Y", scissor + draw),
    ("C Z", rock + won),
  ])

  loadLines(2) |> reduce((accu, val) => accu + Js.Dict.unsafeGet(scoring, val), 0)
}
