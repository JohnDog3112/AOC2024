USING: arrays assocs io.encodings.ascii io.files kernel math
math.functions math.parser prettyprint ranges sequences sets
splitting ;

IN: AOC2024-day4-vocab
! *suffering*
! how it's solved:
!   part1:
!       * the rules are converted to a hashtable where a page is a key and the value is a list of every page that has to be after it
!        * then, gen-windows will take a list like { 1 2 3 } and split it up like this: { { } { 1 } { 1 2 } { 1 2 3 } }
!        * So, for each update I generate a window and 2map it with itself so each iteration is { everything-before current-val rules }
!        * then I grab the rules for current-val and check if any value in everything-before is supposed to be after the current-val
!        * then I just propogate that down to get the valid updates
!    
!    part2:
!        * inverts part1 for filtering to get improper reports
!        * then it generates an inverse rules where the key is the after and the value is every before value
!        * from there, it maps every value in the update with itself and the values supposed to be before it intersected with the report itself
!            * this gets every value in the report that is supposed to be before the mapped value
!        * after that, it keeps iterating over the values that haven't been finalized yet doing this:
!            1. seperates list into a list of values that have nothing before (finished values) and everything else (unfinished values)
!            2. every finished value is removed from the unfinished values before lists
!            3. every finished value is pushed to the corrected report
!        * once every value has been pushed, it returns and carries on to collecting the fixed reports


! ---------- Part1 ----------
: input-file ( -- str ) "./input.txt" ;
: example-file ( -- str ) "./example.txt" ; 
: get-file ( str -- file_content ) ascii file-contents ;

: parse-ordering ( ordering_str -- ordering ) "\n" split [ "|" split [ string>number ] map ] map ;
: parse-updates ( updates_str -- updates ) "\n" split [ "," split [ string>number ] map ] map ;
: parse-file ( file_str -- ordering, updates ) get-file "\n\n" split-subseq first2 [ parse-ordering ] [ parse-updates ] bi* ; 

: default-of ( table key default -- val ) -rot ?of [ nip ] [ drop ] if ;

: gen-hash-table ( key_pair_seq -- table ) H{ } clone [  [ [ first ] [ second ] bi ] [ first ] 2bi HS{ } clone default-of swap over adjoin set-of ] reduce ;

: gen-windows ( seq -- windows ) dup length [0..b] swap [ 0 -rot subseq ] curry map ;

: valid-report? ( report ordering -- bool ) [ rot HS{ } default-of [ in? ] curry any? ] curry swap dup gen-windows 1 head* rot 2map [ ] any? not ;

: get-center ( seq -- val ) dup length 2 / floor swap nth ;

: part1 ( file_str -- sol ) parse-file swap gen-hash-table [ valid-report? ] curry filter [ get-center ] map sum ; 
! ---------- Part2 ----------

: seperate-ready ( pending_list -- ready waiting ) [ [ second cardinality 0 = ] filter [ first ] map ] [ [ second cardinality 0 > ] filter ] bi ;
: remove-ready-from-waiting ( waiting ready -- new_waiting ) [ swap first2 rot [ over delete ] each 2array ] curry map ;

: correct-report-helper ( rules -- out ) V{ } clone swap [ dup cardinality 0 > ] [ seperate-ready over remove-ready-from-waiting -rot [ over push ] each swap ] while drop ;
: correct-report ( report reverse_ordering -- c_report ) dupd dupd [ rot HS{ } clone default-of intersect ] curry curry map zip correct-report-helper ; 
: part2 ( file_str -- sol ) parse-file over gen-hash-table [ valid-report? not ] curry filter swap [ reverse ] map gen-hash-table [ correct-report get-center ] curry map sum ;

! ---------- "Main" ----------

input-file part1 . 
input-file part2 .