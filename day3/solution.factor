USING: ascii io.encodings.ascii io.files kernel math math.parser
prettyprint sequences splitting ;

IN: AOC2024-day2-VOCAB

! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;

: get-input ( -- input ) input-file ascii file-contents ;

: part1 ( input -- sol ) "mul(" split-subseq 1 tail [ ")" split ] map [ length 1 > ] filter [ first "," split ] map [ [ [ digit? ] all? ] all? ] filter [ [ string>number ] map ] map [ length 2 = ] filter 0 [ first2 * + ] reduce ;

! --------- Part2 ----------
: part2 ( input -- sol ) "don't()" split-subseq [ first part1 ] [ 1 tail ] bi [ "do()" split-subseq 1 tail [ part1 ] map ] map [ sum ] map sum + ;

! ---------- "Main" ----------
get-input part1 .
get-input part2 .
