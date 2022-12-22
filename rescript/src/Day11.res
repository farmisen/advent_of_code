open Js.Array
open Utils

type operation =
  | Square
  | Add(int)
  | Mult(int)

type test = {
  divisability: int,
  onTrue: int,
  onFalse: int,
}

type monkey = {
  id: int,
  items: array<int>,
  op: operation,
  test: test,
  inspections: int,
}

type monkeysMap = Belt.Map.Int.t<monkey>

let collectNumbersFromString = (input: string): array<int> => {
  input
  |> Js.String.splitByRe(%re("/\D/"))
  |> map(token =>
    switch token {
    | None => None
    | Some(t) if t === "" => None
    | Some(t) => t |> Belt.Int.fromString
    }
  )
  |> filter(Belt.Option.isSome)
  |> map(Belt.Option.getExn)
}

let intFromString = (str: string): int => str |> Belt.Int.fromString |> Belt.Option.getExn

let toMonkey = (input: string): monkey => {
  let lines = input |> Js.String.split(NodeJs.Os.eol)
  {
    id: lines[0] |> collectNumbersFromString |> first,
    items: lines[1] |> collectNumbersFromString,
    op: switch lines[2] |> Js.String.split(" ") |> sliceFrom(-2) {
    | [o, v] if o === "+" => Add(v |> intFromString)
    | [o, v] if o === "*" && v === "old" => Square
    | [o, v] if o === "*" => Mult(v |> intFromString)
    | _ => raise(WentSouth)
    },
    test: {
      divisability: lines[3] |> collectNumbersFromString |> first,
      onTrue: lines[4] |> collectNumbersFromString |> first,
      onFalse: lines[5] |> collectNumbersFromString |> first,
    },
    inspections: 0,
  }
}

let toMonkeys = (input: string): monkeysMap => {
  input
  |> Js.String.split(NodeJs.Os.eol ++ NodeJs.Os.eol)
  |> map(toMonkey)
  |> reducei((monkeys, monkey, idx) => {
    monkeys |> Belt.Map.Int.set(_, idx, monkey)
  }, Belt.Map.Int.fromArray([]))
}

let print = (pre: 'a, monkeys: monkeysMap) => {
  Js.log2(
    pre,
    monkeys
    |> Belt.Map.Int.keysToArray
    |> map(id => monkeys |> Belt.Map.Int.getExn(_, id))
    |> map(m => m.inspections),
  )
}



let reduceOverItems = (part: int, modulo: int, monkeyId: int, items: array<int>, monkeys: monkeysMap): monkeysMap => {
  items |> reduce((monkeysAccu, item) => {
      let currentMonkey = monkeysAccu |> Belt.Map.Int.getExn(_, monkeyId)

      let tmpNewItem = switch currentMonkey.op {
      | Square => item * item
      | Add(val) => item + val
      | Mult(val) => item * val
      }

      let newItem = switch part {
      | 1 => tmpNewItem / 3
      | _ => mod(tmpNewItem, modulo)
      }

      let otherMonkeyId = switch mod(newItem, currentMonkey.test.divisability) {
      | 0 => currentMonkey.test.onTrue
      | _ => currentMonkey.test.onFalse
      }

      let otherMonkey = monkeysAccu |> Belt.Map.Int.getExn(_, otherMonkeyId)

      monkeysAccu
      |> Belt.Map.Int.set(
        _,
        monkeyId,
        {
          ...currentMonkey,
          items: currentMonkey.items |> filter(i => i !== item),
          inspections: currentMonkey.inspections + 1,
        },
      )
      |> Belt.Map.Int.set(
        _,
        otherMonkeyId,
        {...otherMonkey, items: newItem |> pushTo(otherMonkey.items)},
      )
    }, monkeys)
}


let reduceOverMonkeyIds = (part: int, monkeyIds:array<int>, monkeys: monkeysMap): monkeysMap => {
    let modulo =
    monkeyIds
    |> map(id => monkeys |> Belt.Map.Int.getExn(_, id))
    |> map(m => m.test.divisability)
    |> reduce(\"*", 1)

  monkeyIds |> reduce((monkeysAccu, id) => {
    let monkey = monkeysAccu |> Belt.Map.Int.getExn(_, id)
    // Reduce over items
    monkey.items |> reduceOverItems(part, modulo, id, monkeysAccu)
  }, monkeys)
}


let newRound = (currentRound, part: int, monkeys: monkeysMap): monkeysMap => {
  let monkeyIds = monkeys |> Belt.Map.Int.keysToArray

  // Reduce over monkeys
  // let t = monkeyIds |> reduce((monkeysAccu, id) => {
  //   let monkey = monkeysAccu |> Belt.Map.Int.getExn(_, id)
  //   // Reduce over items
  //   monkey.items |> reduceOverItems(monkeysAccu)
  // }, monkeys)

  let t = monkeyIds |> reduceOverMonkeyIds(part, monkeys)
  if mod(currentRound, 1000) === 0 || currentRound == 1 || currentRound == 20{
    Js.log(currentRound)
    t |> print(">")
  }

  t
}

let simulate = (part: int, rounds: int, input: string): int => {
  let monkeys = input |> toMonkeys

  let rec reduceRounds = (monkeys: monkeysMap, part: int, roundsLeft: int): monkeysMap => {
    switch roundsLeft {
    | 0 => monkeys
    | _ =>
      reduceRounds(monkeys |> newRound(rounds - roundsLeft + 1, part), part, roundsLeft - 1)
    }
  }

  let newMonkeys = reduceRounds(monkeys, part, rounds)
  let t =
    newMonkeys
    |> Belt.Map.Int.keysToArray
    |> map(id => newMonkeys |> Belt.Map.Int.getExn(_, id))
    |> sortArrayWith((m1, m2) => {
      m1.inspections - m2.inspections
    })
    |> map(m => m.inspections)
    |> sliceFrom(-2)

  Js.log(t)
  t |> reduce(\"*", 1)
}

let part01 = (input: string): int => {
  input |> simulate(1, 20)
}

let part02 = (input: string): int => {
  input |> simulate(2, 10000)
}
