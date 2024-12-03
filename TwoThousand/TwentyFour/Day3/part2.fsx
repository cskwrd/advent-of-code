open System
open System.IO
open System.Text.RegularExpressions

let getInput (inputPath: string) =
    File.ReadAllLines(inputPath)
    |> Array.filter (fun line -> not (String.IsNullOrWhiteSpace(line)))

let solveForInput (inputPath: string) =
    let mulEnabled = ref true
    
    getInput inputPath
    |> Seq.collect (fun memoryDump ->
        // this seems like a lot of work just to get the matches out...
        seq {
            for m in Regex.Matches(memoryDump, @"do\(\)|don't\(\)|mul\((?<left>\d+),(?<right>\d+)\)", RegexOptions.Compiled ||| RegexOptions.IgnoreCase) do
                let matchValue = m.Value

                if matchValue.StartsWith("do") then
                    if matchValue = "don't()" then
                        mulEnabled.Value <- false
                    else if matchValue = "do()" then
                        mulEnabled.Value <- true
                else if mulEnabled.Value then
                    yield m
        }
    )
    |> Seq.map (fun parsedOp ->
        // printfn "%A" parsedOp
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
    "[EXAMPLE]", "./Input/Example2.input.txt" // ans: 48
    "[PUZZLE]", "./Input/Puzzle.input.txt" // ans: 97836217, too high
])

solve (fun label result -> printfn "%s The answer is: %A" label result)