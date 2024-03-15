Functions are defined as in rust with the 'fn' keyword. These functions have no stable abi.
Interupts following x86 interupt calling convention can be declared with 'interupt' keyword. These cannot be called.
Integers are defined by literals. Only positive decimal literals at the moment.
All integers wrap.
Basic binary operations: '+', '-', '*', '/', '>>', '<<', '&', '|', '=', '!='
Basic unary operators: '!', '-', '*', '&'
These are to eventually be defined by traits.
'if <cond> { <then> } (else { <else })?'
'loop'
'break'
'struct'

builtin function:
'intrinsic' keyword is used to note that a function is builtin. It is a compile error if this is not actually the case.
