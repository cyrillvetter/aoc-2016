import Data.Ord (comparing)
import Data.List (sort, sortBy, transpose, group)

main = do
    input <- transpose . lines <$> readFile "inputs/6.txt"
    let ex = map extremes input
    print $ map fst ex
    print $ map snd ex

extremes :: String -> (Char, Char)
extremes s = (head (last sorted), head (head sorted))
    where sorted = sortBy (comparing length) $ group $ sort s