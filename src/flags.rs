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
                    color::RED,
                    color::ORANGE,
                    color::YELLOW,
                    color::GREEN,
                    color::INDIGO,
                    color::VIOLET,
                ],
            );
            h.insert(
                "lesbian",
                vec![
                    color::SINOPIA,
                    color::ATOMIC_TANGERINE,
                    color::WHITE,
                    color::SUPER_PINK,
                    color::FLIRT,
                ],
            );
            h.insert(
                "gay",
                vec![
                    color::TEAL,
                    color::TURQUOISE,
                    color::MINT,
                    color::WHITE,
                    color::COBALT_BLUE,
                    color::MAJORELLE_BLUE,
                    color::PURPLE,
                ],
            );
            h.insert(
                "bisexual",
                vec![
                    color::RUBY,
                    color::RUBY,
                    color::MAUVE,
                    color::BLUE,
                    color::BLUE,
                ],
            );
            h.insert(
                "transgender",
                vec![
                    color::MAYA_BLUE,
                    color::AMARANTH_PINK,
                    color::WHITE,
                    color::AMARANTH_PINK,
                    color::MAYA_BLUE,
                ],
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
            h
        };
    }
}
