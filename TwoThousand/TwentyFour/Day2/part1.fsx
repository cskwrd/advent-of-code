open System
open System.IO

let solveForInput (inputPath: string) = 
    // Read all lines from the file and filter out blank or whitespace-only lines
    let reports = 
        File.ReadAllLines(inputPath)
        |> Array.filter (fun line -> not (String.IsNullOrWhiteSpace(line)))
        
    (0, reports)
    ||> Array.fold (fun acc report ->
        let (direction, inBounds) =
            report.Split([| ' ' |], StringSplitOptions.RemoveEmptyEntries ||| StringSplitOptions.TrimEntries)
            |> Seq.map Int64.Parse
            |> Seq.pairwise
            |> Seq.map (fun (left, right) ->
                left - right
            )
            |> Seq.map (fun diff ->
                match diff with
                | 3L | 2L | 1L -> (Some("↓"), true)
                | -3L | -2L | -1L -> (Some("↑"), true)
                | _ -> (None, false)
            )
            |> Seq.reduce (fun (aDirection, aInBounds) (bDirection, bInBounds) ->
                let dir = if aDirection = bDirection then aDirection else None
                let bounds = aInBounds && bInBounds
                (dir, bounds)
            )
        if direction.IsSome && inBounds then
            acc + 1
        else
            acc
    )

printfn "[EXAMPLE] Num safe reports: %i" (solveForInput "./Input/Example.input.txt")

printfn "[PUZZLE] Num safe reports: %i" (solveForInput "./Input/Puzzle.input.txt")
