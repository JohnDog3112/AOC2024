IN: AOC2024-day8-vocab

! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;

: default-of ( table key default -- val ) -rot ?of [ nip ] [ drop ] if ;

: 2d-enumerated ( 2darr -- flat-list ) <enumerated> [ first2 <enumerated> swap [ swap first2 swapd 3array ] curry map ] map-flat ;
: get-antennas ( inp -- antennas ) 2d-enumerated [ third CHAR: . = not ] filter dup H{ } clone [ third V{ } clone set-of ] reduce [ [ 1 head* ] [ third of ] 2bi push ] reduce ;

: gen-pairs ( antenna_group -- pairs ) dup cartesian-product <enumerated> [ first2 swap tail ] map-flat [ first2 = not ] filter ; 
: vec- ( vec1 vec2 -- res ) [ clone ] bi@ [ - ] 2map ;
: vec+ ( vec1 vec2 -- res ) [ clone ] bi@ [ + ] 2map ;

: get-antinodes ( pnt1 pnt2 -- anti1 anti2 ) 2dup vec- dup swapd [ vec+ ] [ vec- ] 2bi* ;

: get-all-antinodes ( antennas -- antinodes ) >alist [ second gen-pairs [ first2 get-antinodes 2array ] map-flat ] map-flat ;
: get-input-size ( inp -- size ) [ first length ] [ length ] bi 2array ;

: part1 ( inp -- sol ) get-file [ get-input-size ] [ get-antennas get-all-antinodes ] bi unique >alist [ first ] map [ [ -1 > ] all? ] filter swap [ [ < ] 2map first2 and ] curry filter length ;

! ---------- Part2 ----------
: get-antinode-pairs ( pnt1 pnt2 -- pnt1&dir1 pnt2&dir2 ) 2dup vec- dup swapd [  ] [ { 0 0 } swap vec- ] 2bi* [ 2array ] 2bi@ ;
: run-antinode-path ( bounds pnt dir -- anti ) V{ } clone -roll [ 2over [ [ > ] 2map first2 and ] [ nip [ -1 > ] all? ] 2bi and ] [ 4dup drop nip swap push swap over vec+ swap ] while 3drop ;
: get-antinodes2 ( bounds pnt1 pnt2 -- anti ) get-antinode-pairs swapd over [ swap first2 run-antinode-path ] 2bi@ append ;

: get-all-antinodes2 ( bounds antennas -- antinodes ) >alist swap [ swap second gen-pairs swap [ swap first2 get-antinodes2 ] curry map-flat ] curry map-flat ;

: part2 ( inp -- sol ) get-file [ get-input-size ] [ get-antennas ] bi get-all-antinodes2 unique >alist [ first ] map length ;

! ---------- "Main" ----------
input-file part1 .
input-file part2 .