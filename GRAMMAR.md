== MUTATING OPS ==
"+" - increment
"-" - decrement
">" - move right
"<" - move left

== NON MUTATING OPS ==
"?<val>,<dest>" - jump if equals (if current cell is equal to <val>, jump to instruction <dest>*)
"!<val>,<dest>" - jump if not equals (if current cell is _not_ equal to <val>, jump to instruction <dest>*)
"." - output current cell
"🦖<amt>" - rollback memory by <amt> mutating operations
"💥" - halt

\* _dest is 0 indexed_
