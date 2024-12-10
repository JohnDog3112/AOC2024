USING: arrays assocs compiler.utilities io.encodings.ascii
io.files kernel math prettyprint sequences sets ;
IN: AOC2024-day6-vocab

! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;
: parse-file ( str -- inp ) get-file [ ":" split first2 swap string>number swap " " split [ string>number ] map [ ] filter 2array ] map ;

: gen-possibilities ( length -- poss ) 1 - { { 0 } { 1 } } [ swap 1 - dup swapd 0 > ] [ { 0 1 } [ [ >vector ] [ ] bi* over push ] cartesian-map [ ] map-flat ] while nip ;
: calculate-possibilities ( numbers -- calc ) dup length gen-possibilities swap [ 1 tail ] [ first ] bi [ swapd [ 0 = [ + ] [ * ] if ] 2reduce ] curry curry map ;

: part1 ( inp -- sol ) parse-file [ first2 calculate-possibilities swap [ = ] curry filter ] map [ length 0 > ] filter [ first ] map sum ;

! ---------- Part2 ----------

! : concat ( a b -- a||b ) dup log10 ceiling 10 swap fpow >integer swapd * + ;
: concat ( a b -- a||b ) [ number>string ] bi@ append string>number ;
: gen-possibilities2 ( length -- poss ) 1 - { { 0 } { 1 } { 2 } } [ swap 1 - dup swapd 0 > ] [ { 0 1 2 } [ [ >vector ] [ ] bi* over push ] cartesian-map [ ] map-flat ] while nip ;
: calculate-possibilities2 ( numbers -- calc ) dup length gen-possibilities2 swap [ 1 tail ] [ first ] bi [ swapd [ dup 0 = [ drop + ] [ 1 = [ * ] [ concat  ] if ] if ] 2reduce ] curry curry map ;

: part2 ( inp -- sol ) parse-file [ first2 calculate-possibilities2 swap [ = ] curry filter ] map [ length 0 > ] filter [ first ] map sum ;


! ---------- Optimized -----------
: run-operation ( a b op -- res ) dup 0 = [ drop + ] [ 1 = [ * ] [ swap concat ] if ] if ;


DEFER: has-solution-part1?

: has-solution-part2? ( ops target nums acc op -- bool ) [ clone [ 1 tail ] [ first ] bi ] [ ] dup tri* run-operation rot has-solution-part1? ; recursive
: has-solution-part1? ( ops nums acc target -- bool ) 2dup > [ 4drop f ] [ 3dup = swap length 0 = [ 4nip ] [ drop -rot 3array dupd [ rot [ first3 ] [ ] bi* has-solution-part2? ] curry curry any? ] if ] if ; recursive
: has-solution? ( ops target nums -- bool ) clone swap [ [ 1 tail ] [ first ] bi ] [ ] bi* has-solution-part1? ;

: optimized ( ops inp -- sol ) parse-file swap [ swap first2 3array ] curry map [ first3 has-solution? ] filter [ second ] map sum ;

! ---------- Optimized2 ------------
: check-division ( target val -- res ) / dup integer? [ ] [ drop f ] if ;
: check-concat ( target val -- res ) [ number>string ] bi@
: rev-operation ( target val op -- res ) dup 0 = [ drop - ] [ 1 = [ check-division ] [ ] if ] if ;
! ---------- "Main" ----------

{ 0 1 } input-file optimized .
{ 0 1 2 } input-file optimized .
