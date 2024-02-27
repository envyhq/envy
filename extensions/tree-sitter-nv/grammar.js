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
      optional($.var_modifier),
      $.var_keyword,
      $.var_identifier,
      ":",
      $.type_identifier,
      optional($.var_block),
    ),

  var_block: ($) => seq("{", repeat($.var_attribute), "}"),

  var_modifier: ($) => /pub/,

  var_attribute: ($) => seq($.attribute_identifier, "=", $._expression),
};

const declarations = {
  _declaration: ($) => choice($.module_declaration, $.var_declaration),
};

const expressions = {
  _expression: ($) => $._literal,
};

const identifiers = {
  module_identifier: (_$) => /[A-Z][a-zA-Z]+/,
  var_identifier: (_$) => /[a-z_]+/,
  attribute_identifier: (_$) => /[a-z_]+/,
  type_identifier: (_$) => /(str|int|float|bool|url|nowt)/,
};

const strings = {
  unescaped_string_fragment: (_) => token.immediate(prec(1, /[^"\\\r\n]+/)),

  str: ($) =>
    seq('"', repeat(choice($.unescaped_string_fragment, $.escape)), '"'),
};

const literals = {
  _literal: ($) => choice($.bool, $.str, $.float, $.int, $.nowt),

  bool: (_$) => /\w(true|false)\w/,
  nowt: (_$) => /\w(nowt)\w/,
  float: (_$) => /\d+\.\d+/,
  int: (_$) => /\d+/,

  escape: (_) =>
    token.immediate(
      seq(
        "\\",
        choice(
          /[^xu0-7]/,
          /[0-7]{1,3}/,
          /x[0-9a-fA-F]{2}/,
          /u[0-9a-fA-F]{4}/,
          /u{[0-9a-fA-F]+}/,
          /[\r?][\n\u2028\u2029]/,
        ),
      ),
    ),

  ...strings,
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
