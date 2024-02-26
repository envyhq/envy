const modules = {
  module_keyword: ($) => "module",

  module_declaration: ($) =>
    seq($.module_keyword, $.module_identifier, $.module_block),

  module_block: ($) => seq("{", repeat($.var_declaration), "}"),
};

const vars = {
  var_keyword: ($) => "var",

  var_declaration: ($) =>
    seq(
      $.var_keyword,
      $.var_identifier,
      ":",
      $.type_identifier
      // optional($.var_block),
    ),

  // var_block: $ => seq(
  //     '{',
  //             $._var_attributes,
  //     '}'
  // ),

  // _var_attributes: $ => repeat($.var_attribute),

  var_attribute: ($) => seq($.attribute_identifier, "=", $._expression),
};

const declarations = {
  _declaration: ($) => choice($.module_declaration, $.var_declaration),
};

const expressions = {
  _expression: ($) => $._literal,
};

const identifiers = {
  var_identifier: (_$) => /[a-z_]+/,
  module_identifier: (_$) => /[a-z_]+/,
  attribute_identifier: (_$) => /[a-z_]+/,
  type_identifier: (_$) => /(str|int|float|bool|url|nowt)/,
};

const literals = {
  _literal: ($) => choice($.bool, $.str, $.float, $.int, $.nowt),

  bool: (_$) => /\w(true|false)\w/,
  nowt: (_$) => /\w(nowt)\w/,
  str: (_$) => /"[^"]*"/,
  float: (_$) => /\d+\.\d+/,
  int: (_$) => /\d+/,
};

module.exports = grammar({
  name: "nv",

  rules: {
    source_file: ($) => repeat($._declaration),

    ...modules,
    ...vars,
    ...declarations,
    ...expressions,
    ...identifiers,
    ...literals,
  },
});
