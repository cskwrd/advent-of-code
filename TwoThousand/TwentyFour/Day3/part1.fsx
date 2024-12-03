open System
open System.IO
open System.Text.RegularExpressions

let getInput (inputPath: string) =
    File.ReadAllLines(inputPath)
    |> Array.filter (fun line -> not (String.IsNullOrWhiteSpace(line)))

let solveForInput (inputPath: string) =
    getInput inputPath
    |> Seq.collect (fun memoryDump ->
        // this seems like a lot of work just to get the matches out...
        seq {
            for m in Regex.Matches(memoryDump, @"mul\((?<left>\d+),(?<right>\d+)\)", RegexOptions.Compiled ||| RegexOptions.IgnoreCase) do
                yield m
        }
    )
    |> Seq.map (fun parsedOp ->
        let left = new bigint(Int64.Parse(parsedOp.Groups["left"].Value))
        let right = new bigint(Int64.Parse(parsedOp.Groups["right"].Value))

        left * right
    )
    |> Seq.sum

// let processProblem solveForInput (inputs: Map<string, string>) =
//     inputs
//     |> Map.iter (fun label inputPath ->
//         let result = solveForInput inputPath
//         printfn "%s The answer is: %A" label result
//     )
let processProblem solveForInput (inputs: Map<string, string>) =
    fun formatter ->
        inputs
        |> Map.iter (fun label inputPath ->
            let result = solveForInput inputPath
            formatter label result
        )

let solve = processProblem solveForInput (Map.ofList [
    "[EXAMPLE]", "./Input/Example.input.txt" // ans: 161
    "[PUZZLE]", "./Input/Puzzle.input.txt"
])

solve (fun label result -> printfn "%s The answer is: %A" label result)