open System
open System.IO

// Optimized function to check safety with potential one-element removal
let isSafeReport (levels: int list) =
    let rec checkSafety (lst: int list) =
        match lst with
        | [] | [_] -> true
        | x::y::rest ->
            let diff = abs(y - x)
            if diff > 3 then 
                false
            else 
                checkSafety (y::rest)
    
    // Check both increasing and decreasing sequences
    let isIncreasingOrDecreasing lst =
        let ascCheck = checkSafety (List.sort lst)
        let descCheck = checkSafety (List.sortDescending lst)
        ascCheck || descCheck

    // Attempt to remove one element
    let rec tryRemovingElements lst =
        match lst with
        | [] -> false
        | _ when isIncreasingOrDecreasing lst -> true
        | _::rest ->
            let removeFirst = rest
            let removeOthers = 
                [for i in 1..List.length lst - 1 do
                    yield List.take i lst @ List.skip (i + 1) lst]
            
            List.exists isIncreasingOrDecreasing (removeFirst::removeOthers)

    tryRemovingElements levels

// Optimized function to read and process reports
let processReports (inputPath: string) =
    File.ReadAllLines(inputPath)
    |> Array.choose (fun line ->
        if String.IsNullOrWhiteSpace(line) then None
        else 
            let levels = 
                line.Split(' ') 
                |> Array.map int 
                |> Array.toList
            Some levels
    )
    |> Array.filter isSafeReport
    |> Array.length

// Example file path (modify this to your actual file path)
let inputPath = "./Input/Puzzle.input.txt"

// Output the number of safe reports
let safeReportCount = processReports inputPath
printfn "Number of safe reports: %d" safeReportCount // 465 - cheated on this one i wrote simething in C# and translated it to F# using Claude.ai
