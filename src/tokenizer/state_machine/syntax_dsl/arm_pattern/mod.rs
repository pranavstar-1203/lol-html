#[macro_use]
mod ch_sequence;

macro_rules! arm_pattern {
    ( | $cb_args:tt |>
         alpha => $actions:tt
    ) => {
        state_body!(@callback | $cb_args |> Some(b'a'...b'z') | Some(b'A'...b'Z') => $actions);
    };

    ( | $cb_args:tt |>
        whitespace => $actions:tt
    ) => {
        state_body!(@callback | $cb_args |>
            Some(b' ') | Some(b'\n') | Some(b'\r') | Some(b'\t') | Some(b'\x0C') => $actions
        );
    };

    ( | [ [$self:tt, $input:ident, $ch:ident ], $($rest_cb_args:tt)+ ] |>
        eoc => ( $($actions:tt)* )
    ) => {
        state_body!(@callback | [ [$self, $input, $ch], $($rest_cb_args)+ ] |>
            None if !$input.is_last() => ({
                action_list!(|$self, $input, $ch|> $($actions)* );

                return Ok(ParsingLoopDirective::Break);
            })
        );
    };

    // NOTE: this arm is always enforced by the compiler to make match exhaustive,
    // so it's safe to break parsing loop here, since we don't have any input left
    // to parse. We execute EOF actions only if it's a last input, otherwise we just
    // break the parsing loop if it hasn't been done by the explicit EOC arm.
    ( | [ [$self:tt, $input:ident, $ch:ident ], $($rest_cb_args:tt)+ ] |>
        eof => ( $($actions:tt)* )
    ) => {
        state_body!(@callback | [ [$self, $input, $ch], $($rest_cb_args)+ ] |>
            None => ({
                if $input.is_last() {
                    action_list!(|$self, $input, $ch|> $($actions)* );
                }

                return Ok(ParsingLoopDirective::Break);
            })
        );
    };

    ( | [ $scope_vars:tt, $($rest_cb_args:tt)+ ] |>
        [ $seq_pat:tt $(; $case_mod:ident)* ] => $actions:tt
    ) => {
        // NOTE: character sequence arm should be expanded in
        // place before we hit the character match block.
        ch_sequence_arm_pattern!(|$scope_vars|> $seq_pat, $actions, $($case_mod)* );
        state_body!(@callback | [ $scope_vars, $($rest_cb_args)+ ] |>);
    };

    ( | $cb_args:tt |> $pat:pat => $actions:tt ) => {
        state_body!(@callback | $cb_args |> Some($pat) => $actions);
    };
}