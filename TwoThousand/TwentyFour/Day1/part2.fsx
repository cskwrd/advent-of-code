open System
open System.IO
open System.Collections.Generic

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
    |> Array.fold (fun ((list:int64 list), (dict: Dictionary<int64,int64>)) line ->
        let parts = line.Split([| ' ' |], StringSplitOptions.RemoveEmptyEntries)
        let r = Int64.Parse(parts.[1])
        if dict.ContainsKey(r) then
            dict.[r] <- dict.[r] + 1L
        else
            dict.[r] <- 1L
        Int64.Parse(parts.[0])::list, dict
    ) ([], Dictionary<int64, int64>())
    |> (fun (locations, occurrences) ->
        seq {
            for location in locations do
                if occurrences.ContainsKey(location) then
                    location * occurrences.[location]
                else
                    0L
        }
    )
    |> Seq.sum

let solve = processProblem solveForInput (Map.ofList [
    "[EXAMPLE]", "./Input/Example.input.txt" // ans: 31
    "[PUZZLE]", "./Input/Puzzle.input.txt" // ans: 23082277
])

solve (fun label result -> printfn "%s The answer is: %A" label result)
