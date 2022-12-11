open NodeJs

@send external padStart: (string, int, string) => string = "padStart"
@send external toString: int => string = "toString"

exception WentSouth

let loadInput = (day: int) => {
  let path = ("../puzzles/day_" ++ day->toString->padStart(2, "0") ++ "/input.txt")
  Fs.readFileSyncWith(path, Fs.readFileOptions(~encoding="UTF-8", ()))->Buffer.toString
}

let sortNumbers = Js.Array.sortInPlaceWith((a, b) => a - b)

let unwrapOrRaise = (exp, a) => {
  switch a {
  | Some(thing) => thing
  | None => raise(exp)
  }
}


let loadLines = (day: int) => day |> loadInput |> Js.String.split(Os.eol)