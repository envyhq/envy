; Vars

(var_declaration
  [
    (var_keyword)
    (var_identifier)
    ":"
    (type_identifier)
  ]
)

; Identifiers

(var_identifier) @variable
(module_identifier) @constant
(type_identifier) @type

; Literals

[
  (var_keyword)
  (module_keyword)
] @keyword

[
  (str)
] @string

[
  (int)
  (float)
] @number

[
  (nowt)
  (bool)
] @constant.builtin
