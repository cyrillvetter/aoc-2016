import Data.List (sort, sortBy, group, intercalate)
import Data.List.Split (splitOn)
import Data.Ord (comparing, Down (Down))

type Room = ([String], Int, String)

alphabet = ['a'..'z']

main = do
    input <- parse . lines <$> readFile "inputs/4.txt"
    print $ validRoomIdSum input
    print $ snd $ head $ filter (\(s, _) -> s == "northpole object storage") $ rotateRoomNames input

parse :: [String] -> [Room]
parse [] = []
parse (x:xs) = (names, id, checksum) : parse xs
    where [left, right] = splitOn "[" x
          checksum = init right
          dashSplit = splitOn "-" left
          id = read $ last dashSplit
          names = init dashSplit

validRoomIdSum :: [Room] -> Int
validRoomIdSum [] = 0
validRoomIdSum ((names, id, checksum):xs)
    | mostCommon == checksum = id + validRoomIdSum xs
    | otherwise = validRoomIdSum xs
    where mostCommon = take 5 $ map head $ sortBy (comparing (Data.Ord.Down . length)) $ group $ sort $ concat names

rotateRoomNames :: [Room] -> [(String, Int)]
rotateRoomNames [] = []
rotateRoomNames ((names, id, _):xs) = (unwords (map rotateLetters names), id) : rotateRoomNames xs
    where
        rest = id `mod` 26
        rotateLetters :: String -> String
        rotateLetters [] = []
        rotateLetters (l:ls) = ([l..'z'] ++ alphabet) !! rest : rotateLetters ls
