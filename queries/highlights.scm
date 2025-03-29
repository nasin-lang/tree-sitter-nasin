(let_stmt (let) @keyword)

(if [
 "if"
 "else"
 "then"
] @keyword.conditional)

[
 (not)
] @keyword.operator

[
 (plus)
 (minus)
 (star)
 (double_star)
 (slash)
 (percent)
 (double_gt)
 (double_lt)
 (pipe)
 (ampersand)
 (double_eq)
 (not_eq)
 (gt)
 (lt)
 (gt_eq)
 (lt_eq)
 (colon)
 "="
] @operator
 
[
 ";"
 ","
 (dot)
] @punctuation.delimiter

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
] @punctuation.bracket

(expr (ident) @variable)

(global_decl name: (ident) @variable)

(let_stmt pat: (ident) @variable)

((expr (ident) @constant)
 (#match? @constant "^[A-Z][A-Z_0-9]*$"))

((global_decl name: (ident) @constant)
 (#match? @constant "^[A-Z][A-Z_0-9]*$"))

((let_stmt pat: (ident) @constant)
 (#match? @constant "^[A-Z][A-Z_0-9]*$"))

[
 (false)
 (true)
] @boolean

(record_lit_field name: (ident) @property)

(record_type_field name: (ident) @property)

(get_prop prop_name: (ident) @property)

(func_decl name: (ident) @function)

(method name: (ident) @function.method)

(macro name: (ident) @function.macro.call)

(directive name: (ident) @attribute)

(macro "@" @keyword.directive)

(call callee: (expr [
 (ident) @function.call
 (get_prop prop_name: (ident) @function.method.call)
]))

(func_param pat: (ident) @variable.parameter)

(number) @number

(string_lit) @string

(type_expr [
 (ident) @type
 (generic_type name: (ident) @type)
])

(type_decl
  "type" @keyword.type
  name: (ident) @type.declaration
)
