; Vars

(var_declaration
  [
    (var_modifier)
    (var_keyword)
    (var_identifier)
    ":"
    (type_identifier)
    (var_block)
  ]
)

; Modules

(module_declaration
  [
    (module_keyword)
    (module_identifier)
    (module_block)
  ]
)

; Identifiers

(var_identifier) @function.method
(module_identifier) @function.builtin
(attribute_identifier) @property
(type_identifier) @type.builtin

; Literals

[
  (var_keyword)
  (module_keyword)
  (var_modifier)
] @keyword

[
  (str)
] @string

[
  (escape)
] @constant

[
  (int)
  (float)
] @number

[
  (nowt)
  (bool)
] @constant.builtin
