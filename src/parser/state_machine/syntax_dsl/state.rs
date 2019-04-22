macro_rules! state {
    (
        $name:ident $(<-- ( $($enter_actions:tt)* ))* {
            $($arms:tt)*
        }

        $($rest:tt)*
    ) => {
        fn $name(&mut self, input: &Chunk) -> StateResult {
            // NOTE: clippy complains about some states that break the loop in each match arm
            #[allow(clippy::never_loop)]
            loop {
                let ch = self.input_cursor().consume_ch(input);

                state_body!(|[self, input, ch]|> [$($arms)*], [$($($enter_actions)*)*]);
            }
        }

        state!($($rest)*);
    };

    // NOTE: end of the state list
    () => ();
}
