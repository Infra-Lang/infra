; Keywords
"let" @keyword
"function" @keyword
"if" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"try" @keyword
"catch" @keyword
"return" @keyword
"import" @keyword
"export" @keyword
"from" @keyword
"in" @keyword
"and" @keyword
"or" @keyword
"not" @keyword

; Types
"number" @type.builtin
"string" @type.builtin
"boolean" @type.builtin
"array" @type.builtin
"object" @type.builtin

; Literals
(number_literal) @number
(string_literal) @string
(boolean_literal) @boolean
(null_literal) @constant.builtin

; Comments
(comment) @comment

; Identifiers
(identifier) @variable

; Function names
(function_declaration name: (identifier) @function)
(postfix_expression (identifier) @function.call
  (#has-parent? postfix_expression argument_list))

; Parameters
(parameter name: (identifier) @parameter)

; Types in annotations
(type_annotation) @type

; Operators
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
":" @punctuation.delimiter
";" @punctuation.delimiter
"." @punctuation.delimiter
"->" @punctuation.special

; Property access
(postfix_expression "." (identifier) @property)

; Object keys
(object_pair key: (identifier) @property)
(object_pair key: (string_literal) @property)
