module.exports = grammar({
    name: "maple",
    word: ($) => $.identifier,
    extras: ($) => [" ", "\t", $.comment],

    rules: {
        // TODO: add the actual grammar rules
        source_file: ($) => repeat($._statement),
        comment: (_) => seq("//", /[^\n]*/),
        _definition: ($) =>
            choice($.function_definition, $.variable_definition),
        fn: ($) => "fn",
        function_definition: ($) =>
            seq(
                $.fn,
                field("function_name", $.identifier),
                $.parameter_list,
                $.block,
            ),
        anon_function: ($) => seq($.fn, $.parameter_list, $.block),
        variable_definition: ($) =>
            seq(
                choice("var", "const"),
                field("name", $.identifier),
                optional(seq("=", field("right", $.expression))),
            ),

        parameter_list: ($) =>
            seq(
                "(",
                optional(
                    choice(
                        seq(
                            repeat(seq(field("parameter", $.identifier), ",")),
                            field("parameter", $.identifier),
                        ),
                    ),
                ),
                ")",
            ),
        block: ($) => seq("{", $.EOS, repeat($._statement), "}"),
        _statement: ($) =>
            seq(
                optional(
                    choice(
                        $._definition,
                        $.expression,
                        $.return_statement,
                        $.while_loop,
                        $.if_statement,
                        $.break_statement,
                        $.continue_statement,
                    ),
                ),
                $.EOS,
            ),
        break_statement: ($) => seq("break"),
        continue_statement: ($) => seq("continue"),
        while_loop: ($) => seq("while", $.expression, $.block),
        if_statement: ($) =>
            seq(
                "if",
                $.expression,
                $.block,
                repeat(seq("elseif", $.block)),
                optional(seq("else", $.block)),
            ),
        return_statement: ($) => seq("return", $.expression),
        EOS: (_) => "\n",
        identifier: (_) => /[a-zA-Z_][a-zA-Z0-9_]*/,
        number: (_) => /[0-9]+((\.[0-9]*)|)/,
        expression: ($) =>
            choice(
                $.number,
                $.identifier,
                $.binary_operator,
                $.unary_operator,
                $.object_access,
                $.function_call,
                $.object_literal,
                $.anon_function,
                $.array_access,
                $.array_literal,
                $.string_literal,
                $.import_expression,
                $.character_literal,
                $.boolean_literal,
            ),
        boolean_literal: (_) => choice("true", "false"),
        character_literal: ($) =>
            seq("'", choice(/[^']/, $.escape_sequence), "'"),
        escape_sequence: (_) => /\\./,
        import_path: (_) => /[\.\/a-zA-Z0-9_]+/,
        import_expression: ($) => seq("import", $.import_path),
        string_literal: ($) =>
            seq('"', repeat(choice(/[^"]/, $.escape_sequence)), '"'),
        op_eq: (_) => "=",
        comma: (_) => ",",
        object_key: ($) =>
            seq(
                choice($.identifier, $.number),
                $.op_eq,
                $.expression,
                optional(","),
                repeat1($.EOS),
            ),
        object_literal: ($) =>
            seq("{", repeat($.EOS), repeat($.object_key), "}"),
        array_literal: ($) =>
            seq(
                "[",
                repeat($.EOS),
                repeat(seq($.expression, optional(","), optional($.EOS))),
                "]",
            ),
        argument_list: ($) =>
            seq(
                "(",
                optional(
                    choice(
                        $.expression,
                        seq(repeat(seq($.expression, ",")), $.expression),
                    ),
                ),
                ")",
            ),
        function_call: ($) =>
            seq(field("caller", choice($.assignable)), $.argument_list),
        object_access: ($) =>
            prec(1, seq($.identifier, repeat1(seq(".", $.identifier)))),
        array_access: ($) =>
            prec(1, seq($.expression, repeat1(seq("[", $.expression, "]")))),
        unary_operator: ($) =>
            choice(
                prec.left(3, seq("-", $.expression)),
                prec.left(3, seq("!", $.expression)),
            ),
        assignable: ($) =>
            choice($.identifier, $.object_access, $.array_access),
        binary_operator: ($) =>
            choice(
                prec.left(
                    5,
                    seq(
                        field("left", $.expression),
                        field("op", "*"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    5,
                    seq(
                        field("left", $.expression),
                        field("op", "/"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    6,
                    seq(
                        field("left", $.expression),
                        field("op", "+"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    6,
                    seq(
                        field("left", $.expression),
                        field("op", "-"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    9,
                    seq(
                        field("left", $.expression),
                        field("op", "<"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    9,
                    seq(
                        field("left", $.expression),
                        field("op", ">"),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    9,
                    seq(
                        field("left", $.expression),
                        field("op", ">="),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    9,
                    seq(
                        field("left", $.expression),
                        field("op", "<="),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    16,
                    seq(
                        field("left", $.assignable),
                        field("op", "="),
                        field("right", $.expression),
                    ),
                ),
                prec.left(
                    16,
                    seq(
                        field("left", $.assignable),
                        field("op", "+="),
                        field("right", $.expression),
                    ),
                ),
            ),
    },
});
