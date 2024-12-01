USING: arrays assocs io.encodings.ascii io.files kernel math
math.parser prettyprint sequences sorting splitting ;

IN: AOC-day1-vocab


! ---------- PART1 -----------
: input-file ( -- x ) "./input.txt" ;

: get-input ( -- x ) input-file ascii file-lines ;

: parse-input ( -- inp ) get-input [ " " split [ first string>number ] [ last string>number ] bi 2array ] map ;

: part1 ( -- sol ) parse-input unzip [ sort ] dup bi* zip [ first2 - abs ] map sum ;


! ---------- PART2 ----------
: get-count-table ( list -- count-table ) H{ } clone [ 2dup ?of [ 1 + ] [ drop 1 ] if set-of ] reduce ;

: get-similarity ( count-alist, count-table -- sim-list ) [ over first ?of [ ] [ drop 0 ] if swap first2 * * ] curry map ;

: part2 ( -- sol ) parse-input unzip [ get-count-table ] dup bi* >alist swap get-similarity sum ;


! ---------- "Main" ----------
part1 .
part2 .

