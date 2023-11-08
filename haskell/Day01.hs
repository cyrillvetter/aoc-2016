import Data.List.Split (splitOn)
import Data.Maybe (fromJust, isJust)
import qualified Data.Set as S

data Direction = North | East | South | West deriving (Enum, Eq)
type Point = (Int, Int)

main = do
    input <- splitOn ", " <$> readFile "inputs/1.txt"
    print $ walkPath input North (0, 0)
    print $ walkPath' input S.empty North (0, 0)

walkPath :: [String] -> Direction -> Point -> Int
walkPath [] _ p = zeroManhattanDistance p
walkPath ((d:num):xs) dir p = walkPath xs nextDir $ move p nextDir $ read num 
          where nextDir = changeDirection dir d

walkPath' :: [String] -> S.Set Point -> Direction -> Point -> Int
walkPath' ((d:num):xs) visited dir p
    | isJust found = fromJust found
    | otherwise = walkPath' xs nextVisited nextDirection nextPoint
    where nextDirection = changeDirection dir d
          (nextPoint, nextVisited, found) = move' p visited nextDirection (read num)

move :: Point -> Direction -> Int -> Point
move (x, y) dir steps
    | dir == North = (x, y + steps)
    | dir == East = (x + steps, y)
    | dir == South = (x, y - steps)
    | otherwise = (x - steps, y)

move' :: Point -> S.Set Point -> Direction -> Int -> (Point, S.Set Point, Maybe Int)
move' p visited _ 0 = (p, visited, Nothing)
move' p visited dir steps
    | nextPoint `S.member` visited = (nextPoint, nextVisited, Just (zeroManhattanDistance nextPoint))
    | otherwise = move' nextPoint nextVisited dir (steps - 1)
    where nextPoint = move p dir 1
          nextVisited = nextPoint `S.insert` visited

changeDirection :: Direction -> Char -> Direction
changeDirection West 'R' = North
changeDirection North 'L' = West
changeDirection dir n
    | n == 'R' = succ dir
    | otherwise = pred dir

zeroManhattanDistance :: Point -> Int
zeroManhattanDistance (x, y) = abs x + abs y
