open NodeJs

@send external padStart: (string, int, string) => string = "padStart"
@send external toString: int => string = "toString"

exception WentSouth

let loadInput = (day: int) => {
  let path = "../puzzles/day_" ++ day->toString->padStart(2, "0") ++ "/input.txt"
  Fs.readFileSyncWith(path, Fs.readFileOptions(~encoding="UTF-8", ()))->Buffer.toString
}

let sortNumbers = Js.Array.sortInPlaceWith((a, b) => a - b)

let unwrapOrRaise = (exp, a) => {
  switch a {
  | Some(thing) => thing
  | None => raise(exp)
  }
}

let first = array => array[0]

let loadLines = (day: int) => day |> loadInput |> Js.String.split(Os.eol)

let toChunks = (size, array) => {
  array |> Js.Array.reducei((accumulator, item, index) => {
    let chunk = if mod(index, size) === 0 {
      []
    } else {
      accumulator |> Js.Array.pop(_) |> Belt.Option.getWithDefault(_, [])
    }

    Js.Array.push(item, chunk)->ignore
    Js.Array.push(chunk, accumulator)->ignore
    accumulator
  }, [])
}
