

IN: AOC2024-day14-vocab


! ----------- Part1 -----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-lines ;
: parse-file ( str -- inp ) get-file [ " " split [ "=" split second "," split [ string>number ] map ] map ] map ;


: get-positions ( dims inp steps -- poses ) 
    [ swap first2 rot [ * ] curry map [ + ] 2map ] curry map 
    over [ [ mod ] 2map ] curry map 
    swap [ swap [ dup 0 < [ + ] [ nip ] if ] 2map ] curry map
;

: add-nth ( seq add nth -- seq ) 
    swap -rot swap 2dup ! add nth seq nth seq
    nth roll + ! nth seq new_val
    -rot dup -roll set-nth
;

: part1 ( dims str -- sol ) 
    dupd parse-file 100 get-positions ! dims poses
    swap [ 1 - 2 / ] map ! poses mid_lines
    [ 0 0 0 0 0 ] clone >vector swap ! poses quadrant_counts mid_lines
    [ ! quadrants pos mid_lines
        [ - ] 2map first2 ! quardrants x y 
        {
            { [ 2dup [ 0 < ] [ 0 < ] bi* and ] [ 0 ] } ! top_left quadrant
            { [ 2dup [ 0 > ] [ 0 < ] bi* and ] [ 1 ] } ! top_right quadrant
            { [ 2dup [ 0 > ] [ 0 > ] bi* and ] [ 2 ] } ! bottom_right quadrant
            { [ 2dup [ 0 < ] [ 0 > ] bi* and ] [ 3 ] } ! bottom_left quadrant
            [ 4 ]
        }
        cond
        2nip ! quadrants quadrant
        1 swap add-nth ! quadrants
    ] 
    curry reduce
    1 head*
    product
;

! ----------- Part2 -----------



! ----------- "Main" -----------
{ 101 103 } input-file part1 .