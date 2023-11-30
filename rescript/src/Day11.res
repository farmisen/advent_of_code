open Js.Array
open Utils

type operation =
  | Square
  | Add(int)
  | Mult(int)

type monkey = {
  items: ref<array<float>>,
  divider: int,
  op: operation,
  onTrue: int,
  onFalse: int,
  inspections: ref<float>,
}

let collectFloatsFromString = (input: string): array<float> => {
  input
  |> Js.String.splitByRe(%re("/\D/"))
  |> map(token =>
    switch token {
    | None => None
    | Some(t) if t === "" => None
    | Some(t) => t |> Belt.Float.fromString
    }
  )
  |> filter(Belt.Option.isSome)
  |> map(Belt.Option.getExn)
}

let collectIntsFromString = (input: string): array<int> => {
  input |> collectFloatsFromString |> map(f => f |> Belt.Int.fromFloat)
}

let intFromString = (str: string): int => str |> Belt.Int.fromString |> Belt.Option.getExn

let toMonkey = (input: string): monkey => {
  let lines = input |> Js.String.split(NodeJs.Os.eol)
  {
    items: ref(lines[1] |> collectFloatsFromString),
    op: switch lines[2] |> Js.String.split(" ") |> sliceFrom(-2) {
    | [o, v] if o === "+" => Add(v |> intFromString)
    | [o, v] if o === "*" && v === "old" => Square
    | [o, v] if o === "*" => Mult(v |> intFromString)
    | _ => raise(WentSouth)
    },
    divider: lines[3] |> collectIntsFromString |> first,
    onTrue: lines[4] |> collectIntsFromString |> first,
    onFalse: lines[5] |> collectIntsFromString |> first,
    inspections: ref(0.0),
  }
}

let toMonkeys = (input: string): array<monkey> => {
  input |> Js.String.split(NodeJs.Os.eol ++ NodeJs.Os.eol) |> map(toMonkey)
}

let print = (monkeys: array<monkey>) => {
  Js.log(monkeys |> map(m => m.items.contents))
}

let simulate = (worryReducer: float => float, rounds: int, monkeys: ref<array<monkey>>): float => {
  Belt.Range.forEach(1, rounds, _ => {
    monkeys :=
      monkeys.contents |> map(monkey => {
        monkey.items.contents |> forEach(
          item => {
            // Js.log(item)

            let newItem = switch monkey.op {
            | Square => item *. item
            | Add(val) => item +. (val |> Belt.Float.fromInt)
            | Mult(val) => item *. (val |> Belt.Float.fromInt)
            } |> worryReducer

            let receiverIdx = switch mod(newItem |> Belt.Float.toInt, monkey.divider) {
            | 0 => monkey.onTrue
            | _ => monkey.onFalse
            }

            monkey.inspections := monkey.inspections.contents +. 1.0
            let _ = monkeys.contents[receiverIdx].items.contents |> push(newItem)
          },
        )
        monkey.items := []
        monkey
      })
  })

  monkeys.contents
  |> map(monkey => monkey.inspections.contents)
  |> Belt.SortArray.stableSortBy(_, (a, b) => a -. b |> Belt.Int.fromFloat)
  |> Belt.Array.sliceToEnd(_, -2)
  |> reduce(\"*.", 1.)
}

let part01 = (input: string): float => {
  ref(input |> toMonkeys) |> simulate(x => x /. 3.0 |> Js.Math.round, 20)
  // Js.log(monkeys)
  // input |> simulate(1, 20)
  // 0.0
}

// let part02 = (input: string): int => {
//   input |> simulate(2, 10000)
// }
