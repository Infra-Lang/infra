module.exports = grammar({
  name: 'infra',

  extras: $ => [
    /\s/,
    $.comment
  ],

  rules: {
    source_file: $ => repeat($._item),

    comment: $ => token(seq('#', /.*/)),

    _item: $ => choice(
      $.function_declaration,
      $.variable_declaration,
      $.expression_statement
    ),

    variable_declaration: $ => seq(
      'let',
      $.identifier,
      optional(seq(':', $.type)),
      '=',
      $.expression
    ),

    type: $ => choice(
      'number',
      'string',
      'boolean',
      'array',
      'object'
    ),

    function_declaration: $ => prec.right(seq(
      'function',
      $.identifier,
      '(',
      optional($.parameters),
      ')',
      optional(seq('->', $.type)),
      ':',
      repeat1($.statement)
    )),

    parameters: $ => sep1($.parameter, ','),

    parameter: $ => seq(
      $.identifier,
      optional(seq(':', $.type))
    ),



    statement: $ => choice(
      $.variable_declaration,
      $.if_statement,
      $.return_statement,
      $.expression_statement
    ),

    if_statement: $ => prec.right(seq(
      'if',
      $.expression,
      ':',
      repeat1($.statement),
      optional(seq('else', ':', repeat1($.statement)))
    )),

    return_statement: $ => prec.right(seq(
      'return',
      optional($.expression)
    )),

    expression_statement: $ => $.expression,

    expression: $ => choice(
      $.binary_expression,
      $.call_expression,
      $.member_expression,
      $.primary_expression
    ),

    binary_expression: $ => choice(
      prec.left(1, seq($.expression, choice('+', '-'), $.expression)),
      prec.left(2, seq($.expression, choice('*', '/'), $.expression)),
      prec.left(3, seq($.expression, choice('==', '!='), $.expression)),
      prec.left(4, seq($.expression, choice('<', '>'), $.expression))
    ),

    call_expression: $ => prec(10, seq(
      $.primary_expression,
      '(',
      optional($.arguments),
      ')'
    )),

    member_expression: $ => prec(10, seq(
      $.primary_expression,
      '.',
      $.identifier
    )),

    arguments: $ => sep1($.expression, ','),

    primary_expression: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.boolean,
      $.array,
      $.object,
      seq('(', $.expression, ')')
    ),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    number: $ => /\d+(\.\d+)?/,

    string: $ => choice(
      seq('"', repeat(/[^"]/), '"'),
      seq("'", repeat(/[^']/), "'")
    ),

    boolean: $ => choice('true', 'false'),

    array: $ => seq(
      '[',
      optional(sep1($.expression, ',')),
      ']'
    ),

    object: $ => seq(
      '{',
      optional(sep1($.object_pair, ',')),
      '}'
    ),

    object_pair: $ => seq(
      choice($.identifier, $.string),
      ':',
      $.expression
    )
  }
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}
