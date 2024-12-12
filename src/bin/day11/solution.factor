USING: arrays assocs combinators io.encodings.ascii io.files
kernel math math.parser prettyprint ranges sequences splitting ;

IN: AOC2024-day8-vocab

! it's the polymer problem!
! simply use a hash-map to store the stones
!   The key is simply the stone number, and the value is the number of stones with that val
!   That way, the algorithm only needs to deal with *unique* numbers
! For instance, after 75 blinks for the example numbers, 
!   there are *only* 54 unique stones to run on rather than 65601038650482

! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- str ) ascii file-lines ;
: parse-file ( str -- stones ) get-file first " " split [ string>number ] map ;

: handle-stone ( stone -- new_stone(s) ) {
    { [ dup 0 = ] [ 1 + 1array ] }
    { [ dup number>string length 2 mod 0 = ] [ number>string dup length 2 / [ head ] [ tail ] 2bi [ string>number ] bi@ 2array ] }
    [ 2024 * 1array ]
} cond ;

: default-of ( table key default -- val ) -rot ?of [ nip ] [ drop ] if ;

: add-of ( assoc key add_val -- assoc ) [ drop ] [ -rot 0 default-of + ] 3bi set-of ;
: make-hash ( stone_seq -- stone_hash ) H{ } clone [ 1 add-of ] reduce ;

: gen-next-stones ( stones -- new_stones_alist ) >alist H{ } clone [ first2 [ handle-stone ] [ [ add-of ] curry ] bi* swapd reduce ] reduce ;

: run-stones ( init_stones iters -- stones ) [1..b] [ drop gen-next-stones ] each ;

: part1 ( str -- sol ) parse-file make-hash 25 run-stones >alist [ second ] map sum ;

! ---------- Part2 ----------
: part2 ( str -- sol ) parse-file make-hash 75 run-stones >alist [ second ] map sum ;

! ---------- "Main" ----------
input-file part1 .
input-file part2 .