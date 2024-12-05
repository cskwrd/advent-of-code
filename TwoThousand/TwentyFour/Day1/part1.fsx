open System
open System.IO
open System.Text.RegularExpressions

let processProblem solveForInput (inputs: Map<string, string>) =
    fun formatter ->
        inputs
        |> Map.iter (fun label inputPath ->
            let result = solveForInput inputPath
            formatter label result
        )

let getInput (inputPath: string) =
    File.ReadAllLines(inputPath)
    |> Array.filter (fun line -> not (String.IsNullOrWhiteSpace(line)))

let solveForInput (inputPath: string) =
    getInput inputPath
    |> Array.map (fun line ->
        line.Split([| ' ' |], StringSplitOptions.RemoveEmptyEntries ||| StringSplitOptions.TrimEntries)
    )
    |> Array.map (fun locations ->
        let l = Int64.Parse(locations.[0])
        let r = Int64.Parse(locations.[1])
        
        (l, r)
    )
    |> Array.unzip
    |> (fun (first, second) ->
        Array.zip (first |> Array.sort) (second |> Array.sort)
    )
    |> Array.sumBy (fun (f, s) ->
        abs (f - s)
    )

let solve = processProblem solveForInput (Map.ofList [
    "[EXAMPLE]", "./Input/Example.input.txt" // ans: 11
    "[PUZZLE]", "./Input/Puzzle.input.txt" // ans: 2375403
])

solve (fun label result -> printfn "%s The answer is: %A" label result)
