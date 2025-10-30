module.exports = grammar({
  name: 'infra',

  rules: {
    // Source file
    source_file: $ => repeat(choice($._statement, $._expression)),

    // Statements
    _statement: $ => choice(
      $.function_declaration,
      $.class_declaration,
      $.variable_declaration,
      $.return_statement,
      $.if_statement,
      $.for_statement,
      $.while_statement,
      $.break_statement,
      $.continue_statement,
      $.expression_statement,
      $.import_statement,
      $.export_statement,
      $.empty_statement
    ),

    // Expressions
    _expression: $ => choice(
      $.assignment_expression,
      $.binary_expression,
      $.unary_expression,
      $.call_expression,
      $.member_expression,
      $.identifier,
      $.literal,
      $.parenthesized_expression,
      $.if_expression,
      $.array_expression,
      $.object_expression,
      $.await_expression
    ),

    // Function declaration
    function_declaration: $ => seq(
      'function',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      optional(':'),
      optional(field('return_type', $.type_annotation)),
      field('body', $.block_statement)
    ),

    // Class declaration
    class_declaration: $ => seq(
      'class',
      field('name', $.identifier),
      optional($.class_inheritance),
      field('body', $.block_statement)
    ),

    class_inheritance: $ => seq(':', $.identifier),

    // Variable declaration
    variable_declaration: $ => seq(
      optional('let'),
      field('pattern', $.variable_pattern),
      optional(':'),
      optional(field('type', $.type_annotation)),
      '=',
      field('value', $._expression)
    ),

    variable_pattern: $ => choice(
      $.identifier,
      $.object_pattern,
      $.array_pattern
    ),

    // Return statement
    return_statement: $ => seq('return', optional($._expression)),

    // If statement
    if_statement: $ => seq(
      'if',
      field('condition', $.parenthesized_expression),
      field('consequence', $.block_statement),
      optional(seq('else', field('alternative', choice($.block_statement, $.if_statement))))
    ),

    // If expression
    if_expression: $ => seq(
      'if',
      field('condition', $.parenthesized_expression),
      '?',
      field('consequence', $._expression),
      ':',
      field('alternative', $._expression)
    ),

    // For statement
    for_statement: $ => seq(
      'for',
      field('variable', $.identifier),
      'in',
      field('iterable', $._expression),
      field('body', $.block_statement)
    ),

    // While statement
    while_statement: $ => seq(
      'while',
      field('condition', $.parenthesized_expression),
      field('body', $.block_statement)
    ),

    // Block statement
    block_statement: $ => seq(
      '{',
      repeat($._statement),
      '}'
    ),

    // Expression statement
    expression_statement: $ => seq($._expression, ';'),

    // Import statement
    import_statement: $ => seq(
      'import',
      choice(
        $.string_literal,
        seq('{', repeat1($.identifier), '}', 'from', $.string_literal)
      )
    ),

    // Export statement
    export_statement: $ => seq(
      'export',
      choice(
        seq('function', $.identifier, $.parameter_list, optional($.block_statement)),
        seq('class', $.identifier, optional($.class_inheritance), optional($.block_statement)),
        seq('let', $.identifier, optional(seq('=', $._expression))),
        seq('{', repeat1($.identifier), '}')
      )
    ),

    // Empty statement
    empty_statement: $ => ';',

    // Assignment expression
    assignment_expression: $ => prec.right(
      1,
      seq(
        field('left', choice($.identifier, $.member_expression, $.subscript_expression)),
        '=',
        field('right', $._expression)
      )
    ),

    // Binary expressions
    binary_expression: $ => {
      const table = [
        ['or', 1],
        ['and', 2],
        ['==', 3], ['!=', 3], ['<', 3], ['<=', 3], ['>', 3], ['>=', 3],
        ['+', 4], ['-', 4],
        ['*', 5], ['/', 5], ['%', 5],
        ['**', 6]
      ];

      return choice(...table.map(([operator, precedence]) => {
        return prec.left(precedence, seq(
          field('left', $._expression),
          operator,
          field('right', $._expression)
        ));
      }));
    },

    // Unary expressions
    unary_expression: $ => choice(
      prec.left(9, seq('!', field('argument', $._expression))),
      prec.left(9, seq('-', field('argument', $._expression))),
      prec.left(9, seq('+', field('argument', $._expression)))
    ),

    // Call expression
    call_expression: $ => prec.left(8, seq(
      field('function', $._expression),
      field('arguments', $.argument_list)
    )),

    // Member expression
    member_expression: $ => prec.left(7, seq(
      field('object', $._expression),
      '.',
      field('property', $.identifier)
    )),

    // Subscript expression
    subscript_expression: $ => prec.left(7, seq(
      field('object', $._expression),
      '[',
      field('index', $._expression),
      ']'
    )),

    // Parenthesized expression
    parenthesized_expression: $ => seq('(', $._expression, ')'),

    // Argument list
    argument_list: $ => seq(
      '(',
      commaSep($._expression),
      optional(','),
      ')'
    ),

    // Parameter list
    parameter_list: $ => seq(
      '(',
      commaSep($.parameter),
      optional(','),
      ')'
    ),

    parameter: $ => seq(
      $.identifier,
      optional(':'),
      optional($.type_annotation)
    ),

    // Array expression
    array_expression: $ => seq(
      '[',
      commaSep($._expression),
      optional(','),
      ']'
    ),

    // Object expression
    object_expression: $ => seq(
      '{',
      commaSep($.pair),
      optional(','),
      '}'
    ),

    pair: $ => seq(
      field('key', choice($.identifier, $.string_literal)),
      ':',
      field('value', $._expression)
    ),

    // Object pattern
    object_pattern: $ => seq(
      '{',
      commaSep(choice($.identifier, seq($.identifier, ':', $.identifier))),
      optional(','),
      '}'
    ),

    // Array pattern
    array_pattern: $ => seq(
      '[',
      commaSep($.identifier),
      optional(','),
      ']'
    ),

    // Await expression
    await_expression: $ => seq('await', $._expression),

    // Literals
    literal: $ => choice(
      $.number_literal,
      $.string_literal,
      $.boolean_literal,
      $.nil_literal,
      $.regex_literal
    ),

    number_literal: $ => /\d+(\.\d+)?([eE][+-]?\d+)?/,

    string_literal: $ => choice(
      seq('"', repeat(/[^"\\]/), '"'),
      seq("'", repeat(/[^'\\]/), "'"),
      seq('f"', repeat(/[^"\\]/), '"'),
      seq("f'", repeat(/[^'\\]/), "'"),
      seq('r"', repeat(/[^"\\]/), '"'),
      seq("r'", repeat(/[^'\\]/), "'")
    ),

    boolean_literal: $ => choice('true', 'false'),

    nil_literal: $ => 'nil',

    regex_literal: $ => seq('/', repeat(/[^\//], '/'),

    // Identifier
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    // Type annotation
    type_annotation: $ => choice(
      'number',
      'string',
      'boolean',
      'nil',
      'array',
      'object',
      'function',
      $.function_type,
      $.generic_type
    ),

    function_type: $ => seq(
      '(',
      commaSep($.type_annotation),
      ')',
      '=>',
      $.type_annotation
    ),

    generic_type: $ => seq($.identifier, '<', commaSep($.type_annotation), '>'),

    // Keywords
    _keyword: $ => choice(
      'function', 'class', 'let', 'return', 'if', 'else', 'for', 'while',
      'break', 'continue', 'import', 'export', 'and', 'or', 'not', 'in',
      'await', 'async', 'true', 'false', 'nil'
    )
  }
});

function commaSep(rule) {
  return optional(commaSep1(rule));
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}
