import Data.List (sort, transpose)
import Data.List.Split (chunksOf)

main = do
    input <- map (map read . words) . lines <$> readFile "inputs/3.txt"
    print $ trianglesAmount input
    print $ trianglesAmount $ chunksOf 3 $ concat $ transpose input

trianglesAmount :: [[Int]] -> Int
trianglesAmount = length . filter ((\[x, y, z] -> x + y > z) . sort)