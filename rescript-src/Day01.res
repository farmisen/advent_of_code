open NodeJs
open Js.Array
open Js.String
open Utils

let reduceLine = line =>
  line
  |> split(Os.eol)
  |> map(Belt.Int.fromString)
  |> map(unwrapOrRaise(WentSouth))
  |> reduce(\"+", 0)

let sortedCalCounts =
  loadInput(1)
  |> Js.String.concat(Os.eol ++ Os.eol)
  |> split(Os.eol ++ Os.eol)
  |> filter(s => s != "" && s != Os.eol)
  |> map(reduceLine)
  |> sortNumbers

let part01 = () => {
  sortedCalCounts |> pop |> unwrapOrRaise(WentSouth)
}

let part02 = () => {
  sortedCalCounts |> sliceFrom(-3) |> reduce(\"+", 0)
}
