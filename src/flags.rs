pub mod flag {
    use std::collections::HashMap;

    use lazy_static::lazy_static;

    use crate::colors::color;

    lazy_static! {
        pub static ref FLAGS: HashMap<&'static str, Vec<color::RGBColor>> = {
            let mut h = HashMap::new();
            h.insert(
                "pride",
                vec![
                    color::PRIDE_RED,
                    color::PRIDE_ORANGE,
                    color::PRIDE_YELLOW,
                    color::PRIDE_GREEN,
                    color::PRIDE_INDIGO,
                    color::PRIDE_VIOLET,
                ],
            );
            h.insert(
                "lesbian",
                vec![
                    color::LESBIAN_SINOPIA,
                    color::LESBIAN_ATOMIC_TANGERINE,
                    color::WHITE,
                    color::LESBIAN_SUPER_PINK,
                    color::LESBIAN_FLIRT,
                ],
            );
            h.insert(
                "gay",
                vec![
                    color::GAY_TEAL,
                    color::GAY_TURQUOISE,
                    color::GAY_MINT,
                    color::WHITE,
                    color::GAY_COBALT_BLUE,
                    color::GAY_MAJORELLE_BLUE,
                    color::GAY_PURPLE,
                ],
            );
            h.insert(
                "bisexual",
                vec![
                    color::BISEXUAL_RUBY,
                    color::BISEXUAL_RUBY,
                    color::BISEXUAL_MAUVE,
                    color::BISEXUAL_BLUE,
                    color::BISEXUAL_BLUE,
                ],
            );
            h.insert(
                "transgender",
                vec![
                    color::TRANS_MAYA_BLUE,
                    color::TRANS_AMARANTH_PINK,
                    color::WHITE,
                    color::TRANS_AMARANTH_PINK,
                    color::TRANS_MAYA_BLUE,
                ],
            );
            h.insert(
                "non-binary",
                vec![
                    color::ENBY_YELLOW,
                    color::WHITE,
                    color::ENBY_MAUVE,
                    color::ENBY_BLACK,
                ],
            );
            h.insert(
                "asexual",
                vec![color::BLACK, color::ACE_GREY, color::WHITE, color::PURPLE],
            );
            h
        };
        pub static ref NAME_ALIASES: HashMap<Vec<&'static str>, &'static str> = {
            let mut h = HashMap::new();
            h.insert(vec!["p"], "pride");
            h.insert(vec!["l"], "lesbian");
            h.insert(vec!["g"], "gay");
            h.insert(vec!["b", "bi"], "bisexual");
            h.insert(vec!["t", "trans"], "transgender");
            h.insert(vec!["nb", "enby", "nonbinary"], "non-binary");
            h.insert(vec!["a", "ace"], "asexual");
            h
        };
    }
}
