USING: arrays assocs io.encodings.ascii io.files kernel math
math.parser prettyprint sequences splitting ;

IN: AOC2024-day12-vocab


! manually solved the following equations:
!   x = (a_x)A + (b_x)B
!   y = (a_y)A + (b_y)B 
! to get:
!   B = ((a_x)y - (a_y)x)/((a_x)(b_y) - (a_y)(b_x))
!   A = (x - (b_x)B)/(a_x)
! I solved A and B for every game and then filtered by the ones that gave whole integers for A and B
! For part2, I simply added 10000000000000 to x and y and then ran it through part1

! Solved in desmos: https://www.desmos.com/calculator/5b5ve9wvqt

! ----------- Part1 -----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;
: parse-file ( str -- inp ) get-file { "" } split 
    [
        first3
        "=" split 1 tail
        -rot
        [
            "+" split 1 tail 
        ]
        bi@

        [
            [ "," split first string>number ] 
            map
        ]
        tri@
        rot
        3array
    ]
    map
;

! B = ( ax*y - ay*x )/(ax*by - ay*bx)
: get-pushes ( AXY BXY XY -- A B )
    ! { ax ay } { bx by } { x y }
    [
        nip reverse [ * ] 2map first2 - ! (ax*y - ay*x)
    ]
    [
        drop reverse [ * ] 2map first2 - ! (ax*by - ay*bx)
    ]
    [
        first -rot zip first ! x { ax bx }
        first2 3array ! { x ax bx }
    ]
    3tri ! (ax*y - ay*x) (ax*by - ay*bx) {x  ax bx }
    -rot / ! { x ax bx } B
    swap dupd first3                    ! B B x ax bx
    ! target: B ax x bx B * - swap /
    swap -roll                          ! B ax B x bx
    rot                                 ! B ax x bx B
    * - swap / ! B A 
    swap   
;

: part1 ( inp -- sol ) [ first3 get-pushes 2array ] map [ [ dup >integer = ] all? ] filter [ first2 [ 3 * ] [ ] bi* + ] map sum ;

! ----------- Part2 -----------

: part2 ( inp -- sol ) [ first3 [ 10000000000000 + ] map 3array ] map part1 ;


! ----------- "Main" -----------
input-file parse-file part1 .
input-file parse-file part2 .