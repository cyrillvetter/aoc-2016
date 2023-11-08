import Data.Char (intToDigit)
import Debug.Trace (trace)

type Point = (Int, Int)

keyPad1 = [[' ', ' ', ' ', ' ', ' '],
           [' ', '1', '2', '3', ' '],
           [' ', '4', '5', '6', ' '],
           [' ', '7', '8', '9', ' '],
           [' ', ' ', ' ', ' ', ' ']]

keyPad2 = [[' ', ' ', '1', ' ', ' '],
           [' ', '2', '3', '4', ' '],
           ['5', '6', '7', '8', '9'],
           [' ', 'A', 'B', 'C', ' '],
           [' ', ' ', 'D', ' ', ' ']]

main = do
    input <- lines <$> readFile "inputs/2.txt"
    print $ map (inputCode keyPad1 (2, 2)) input
    print $ map (inputCode keyPad2 (2, 0)) input

inputCode :: [String] -> Point -> String -> Char
inputCode keyPad (x, y) [] = keyPad !! y !! x
inputCode keyPad p (d:ds)
    | isPointInBoundary (x, y) && keyPad !! y !! x /= ' ' = inputCode keyPad (x, y) ds
    | otherwise = inputCode keyPad p ds
    where (x, y) = move p d

move :: Point -> Char -> Point
move (x, y) 'U' = (x, y - 1)
move (x, y) 'D' = (x, y + 1)
move (x, y) 'L' = (x - 1, y)
move (x, y) 'R' = (x + 1, y)

isPointInBoundary :: Point -> Bool
isPointInBoundary (x, y) = x >= 0 && x <= 4 && y >= 0 && y <= 4