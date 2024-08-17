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
(var_identifier) @function.method

; Modules

(module_declaration
  [
    (module_keyword)
    (module_identifier)
    (module_block)
  ]
)
(module_identifier) @function.builtin

; Providers

(provider_declaration
  [
    (provider_keyword)
    (provider_identifier)
    ":"
    (provider_type_identifier)
    (provider_block)
  ]
)
(provider_identifier) @function.builtin
(provider_type_identifier) @type.builtin

; Identifiers

(attribute_identifier) @property
(type_identifier) @type.builtin

; Literals

[
  (var_keyword)
  (module_keyword)
  (provider_keyword)
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
