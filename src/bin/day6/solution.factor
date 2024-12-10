USING: arrays assocs compiler.utilities io.encodings.ascii
io.files kernel math prettyprint sequences sets ;
IN: AOC2024-day6-vocab

! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;

: get-start ( inp -- x y ) <enumerated> [ first2 <enumerated> [ second CHAR: ^ = ] filter 2array ] map [ second length 0 > ] filter first first2 first first swap ;
: set-2dnth ( inp x y val -- inp ) swapd -roll -roll over nth rot roll -rot swap swapd set-nth ;
: ?2dnth ( inp x y -- val ) swap -rot swap ?nth dup [ ?nth ] [ nip ] if ;

: over2 ( x y z -- x y z x y ) 3dup drop ;

: add-points ( pnt1 pnt2 -- pnt ) [ + ] 2map ; 
: turn-right ( offset -- turned ) first2 neg swap 2array ;

: main-loop ( inp loc dir -- new-inp )  [ over2 first2 ?2dnth ] [ over2 first2 CHAR: X set-2dnth over2 add-points swap over first2 ?2dnth CHAR: # = [ drop turn-right ] [ rot drop swap ] if ] while 2drop ;
: part1-helper ( inp -- new-inp ) dup get-start 2array { 0 -1 } clone main-loop ;
: part1 ( file -- sol ) get-file part1-helper [ [ CHAR: X = ] count ] map sum ; 
! ---------- Part2 ----------

: part2-inner-loop ( inp loc dir -- inp loc1 dir1 ) 3dup add-points swap over first2 ?2dnth CHAR: # = [ drop turn-right ] [ rot drop swap ] if ;
: part2-loops? ( inp loc dir -- loops? ) HS{ } clone -roll [ 2array [ first2 ] [ nip swap in? not ] [ first first2 ?2dnth nip ] 3tri and ] [ 2array 3dup nip swap adjoin first2 part2-inner-loop ] while 2array nip swap in? ;

: get-possible-points ( inp -- pnts ) dup get-start 2array swap part1-helper <enumerated> [ first2 <enumerated> swap [ swap first2 swapd 3array ] curry map ] map-flat [ third CHAR: X = ] filter [ first2 2array ] map swap [ = not ] curry filter ;
: test-possiblities ( poss_pnts inp start -- poss ) [ -rot swap 2dup first2 CHAR: # set-2dnth roll first2 [ clone ] bi@ part2-loops? -rot first2 CHAR: . set-2dnth drop ] curry curry map ;

: part2 ( file -- sol ) get-file dup clone [ clone ] map get-possible-points over get-start 2array { 0 -1 } 2array swapd test-possiblities [ ] count ;
! ---------- "Main" ----------
input-file part1 .
input-file part2 .