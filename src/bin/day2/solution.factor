USING: io.encodings.ascii io.files kernel math math.parser
prettyprint ranges sequences splitting ;

IN: AOC-day2-vocab

! ---------- Part1 -----------
: input-file ( -- x ) "./input.txt" ;

: get-input ( -- x ) input-file ascii file-lines ;

: parse-input ( -- x ) get-input [ " " split [ string>number ] map ] map ; 

: all-eq? ( seq -- bool ) dup first [ = ] curry all? ; 

: get-bound-dir ( report -- direction_list ) [ 1 head* ] [ 1 tail ] bi [ - dup abs dup 4 < [ ] [ nip 0 swap ] if dup 0 = [ drop ] [ / ] if ] 2map ;

: check-report ( report -- valid_bool ) get-bound-dir dup first 0 = [ drop f ] [ all-eq? ] if ;

: part1 ( -- sol ) parse-input [ check-report ] map [ ] count ;


! ---------- Part2 ----------

: gen-one-removed-lists ( seq -- seq_list ) dup length [0..b] swap [ swap 1 - dup -1 = [ drop ] [ swap remove-nth ] if ] curry map ;

: part2 ( -- sol ) parse-input [ gen-one-removed-lists [ check-report ] map [ ] any? ] map [ ] count ;



! ---------- "Main" -----------
part1 .
part2 .
