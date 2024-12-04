USING: arrays compiler.utilities io.encodings.ascii io.files
kernel math prettyprint ranges sequences
sequences.generalizations ;


IN: AOC2024-day4-vocab

! ---------- Part1 -----------

: input-file ( -- str ) "./input.txt" ;
: get-input ( -- str ) input-file ascii file-lines ;

: gen-seq ( len off -- seq ) [ 2dup [ first * ] [ second * ] 2bi* 2array ] curry swap 1 - [0..b] swap map ;
: up-off ( -- up_seq ) 4 { 0 -1 } gen-seq ;
: up-right-off ( -- up_right_seq ) 4 { 1 -1 } gen-seq ;
: right-off ( -- right_seq ) 4 { 1 0 } gen-seq ;
: down-right-off ( -- down_right_seq ) 4 { 1 1 } gen-seq ;
: down-off ( -- down_seq ) 4 { 0 1 } gen-seq ;
: down-left-off ( -- down_left_seq ) 4 { -1 1 } gen-seq ;
: left-off ( -- down_seq ) 4 { -1 0 } gen-seq ;
: up-left-off ( -- down_seq ) 4 { -1 -1 } gen-seq ;

: directions ( -- dir_seq ) up-off up-right-off right-off down-right-off down-off down-left-off left-off up-left-off 8 narray ;

: apply-seq ( offset_seq base_pnt -- pnt_seq ) [ 2dup [ [ first ] bi@ + ] [ [ second ] bi@ + ] 2bi* 2array ] curry map ;

: default-nth ( arr n default -- val ) -rot swap ?nth dup [ nip ] [ drop ] if ;
: default-2dnth ( arr x y default -- val ) -roll swap -rot { } default-nth swap rot default-nth ; 

: get-seq ( map pnt_seq -- seq ) swap [ swap first2 CHAR: . default-2dnth ] curry map ;

: get-dim ( str_seq -- x_size y_size ) [ first length ] [ length ] bi ;
: gen-index-map ( x_size y_size -- map ) swap [0..b] 1 head* [ swap [ 2array ] curry map ] curry swap [0..b] 1 head* swap map-flat ;


: part1 ( input -- sol ) dup get-dim gen-index-map swap [ [ -rot apply-seq get-seq ] curry curry directions swap map ] curry map-flat [ "XMAS" >array = ] filter length ;

! ---------- Part2 -----------
: top-left-diag ( -- seq ) { { -1 -1 } { 0 0 } { 1 1 } } ;
: bottom-right-diag ( -- seq ) { { 1 1 } { 0 0 } { -1 -1 } } ;
: diag1 ( -- seq ) top-left-diag bottom-right-diag 2array ;

: top-right-diag ( -- seq ) { { 1 -1 } { 0 0 } { -1 1 } } ;
: bottom-left-diag ( -- seq ) { { -1 1 } { 0 0 } { 1 -1 } } ;
: diag2 ( -- seq ) top-right-diag bottom-left-diag 2array ;

: xmas? ( loc map -- bool ) [ -rot apply-seq get-seq ] curry curry [ diag1 ] [ diag2 ] bi [ swap map [ "MAS" >array = ] any? ] 2bi@ and ;

: part2 ( input -- sol ) dup get-dim gen-index-map swap [ xmas? ] curry count ;

! ---------- "Main" ----------
get-input part1 .
get-input part2 .