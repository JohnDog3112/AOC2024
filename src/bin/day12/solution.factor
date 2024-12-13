IN: AOC2024-day12-vocab

! ----------- Part1 -----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;

: default-of ( table key default -- val ) -rot ?of [ nip ] [ drop ] if ;
: add-of ( assoc key add_val -- assoc ) [ drop ] [ -rot 0 default-of + ] 3bi set-of ;

: default-nth ( seq n def -- val ) -rot swap ?nth dup [ nip ] [ drop ] if ; 
: default-2dnth ( seq x y def -- val ) -roll swapd { } default-nth -rot swap default-nth ;

: directions ( -- dirs ) { { 1 0 } { 0 1 } { -1 0 } { 0 -1 } } clone [ clone ] map ;
: vec+ ( vec1 vec2 -- n_vec ) [ + ] 2map ;

: get-area-perim ( map explored pnt -- area perim ) 2dup swap in? 
    [ 3drop 0 0 ]
    [ 
        2dup swap adjoin ! adds point to explored list
        3dup nip first2 CHAR: ! default-2dnth ! map explored pnt char
        swap [ vec+ ] curry directions swap map ! map explored char pnt_seq
        -roll ! pnt_seq map explored char
        [ ! pnt map explored char
            swap -roll swapd ! explored map pnt char
            2over first2 CHAR: ! default-2dnth = ! explored map pnt same?
            [ ! explored map pnt 
                swapd get-area-perim ! area perim
            ]
            [ ! explored map pnt
                3drop 0 1
            ]
            if ! area perim
            2array
        ] 3curry map ! { { area perim } { area perim } ... }
        { 1 0 } [ vec+ ] reduce first2
    ]
    if
; recursive

: 2d-enumerated ( 2darr -- flat-list ) <enumerated> [ first2 <enumerated> swap [ swap first swap 2array ] curry map ] map-flat ;

: part1 ( str -- sol ) get-file dup 2d-enumerated swap HS{ } clone [ rot get-area-perim * ] 2curry map sum ;

! ----------- Part2 -----------

: vec- ( vec1 vec2 -- n_vec ) [ - ] 2map ;

! sides is a hashset of { x y dir }
: get-area-sides-helper ( map explored&sides pnt -- area sides ) 2dup swap first in? 
    [ 3drop 0 0 ]
    [ 
        2dup swap first adjoin ! adds point to explored&sides list
        3dup nip first2 CHAR: ! default-2dnth ! map explored&sides pnt char
        over [ vec+ ] curry directions swap map ! map explored pnt char pnt_seq
        -rot 2array swap ! map explored pnt&char pnt_seq
        -roll ! pnt_seq map explored pnt&char
        [ ! n_pnt map explored&sides pnt&char
            swap -roll swapd ! explored&sides map n_pnt pnt&char
            [ ]
            [   ! map n_pnt pnt&char
                first vec- nip ! dir
            ]
            [   ! map n_pnt pnt&char
                second -rot first2 CHAR: ! default-2dnth = ! same?
            ]
            3tri ! explored&sides map n_pnt pnt&char dir same?
            rot drop ! explored&sides map n_pnt dir same?
            [ ! explored&sides map n_pnt dir 
                drop swapd get-area-sides-helper ! area sides
            ]
            [ ! explored&sides map n_pnt dir
                2dup vec- -rot nip ! explored&sides map pnt dir
                [ first2 ] [ ] bi* 3array nip ! explored&sides side
                over second adjoin second ! sides
                0 swap ! area sides
            ]
            if ! area sides
            2array
        ] 3curry map ! { { area sides } { area sides } ... }
        unzip ! { area1 area2 ... } { sides sides sides ... }
        first ! { area1 area2 ... } sides
        swap sum 1 + swap ! total_area sides
    ]
    if
; recursive

: turn-right ( offset -- turned ) first2 neg swap 2array ;
: turn-left ( offset -- turned ) first2 swap neg 2array ;

: count-sides ( orig_dir sides point dir -- orig_dir side_tail ) 
    [
        4dup drop first2 roll 3array swap in?
    ]
    [
        4dup drop first2 roll 3array swap delete
        [ vec+ ] [ nip ] 2bi ! acc sides n_point dir

    ]
    while
    2drop ! orig_dir sides
;

: get-area-sides ( map explored pnt -- area sides ) 
    swap HS{ } clone 2array ! map pnt explored&sides
    swap get-area-sides-helper ! area sides
    0 -rot ! acc area sides
    [ dup cardinality 0 > ] ! while sides.len > 0
    [ ! acc area sides
        members [ 1 tail >hash-set ] [ first ] bi ! acc area sides_tail side
        first3 -rot 2array swap ! acc area sides_tail pnt dir
        dup -roll ! acc area orig_dir sides_tail pnt dir
        [
            turn-right dup rot vec+ swap
        ]
        [
            turn-left dup rot vec+ swap
        ]
        2bi ! acc area orig_dir sides_tail clockwise_point dir1 counter_point dir2 
        2array -roll ! acc area orig_dir counter&dir2 side_tail clockwise_point dir1
        3array rot swap first3 ! acc area counter&dir2 orig_dir side_tail clockwise_point dir1
        count-sides ! acc area counter&dir2 orig_dir side_tail
        rot first2 ! acc area orig_dir side_tail counter_point dir2
        count-sides ! acc area orig_dir side_tail
        nip ! acc area side_tail
        rot 1 + -rot
    ] 
    while
    drop swap ! area acc
;

: get-area-sides-entry ( map explored pnt -- area sides ) 2dup swap in? [ 3drop 0 0 ] [ get-area-sides ] if ;

: part2 ( str -- sol ) get-file dup 2d-enumerated swap HS{ } clone [ rot get-area-sides-entry * ] 2curry map sum ;


! ----------- "Main" -----------
input-file part1 .
input-file part2 .