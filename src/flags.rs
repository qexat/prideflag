pub mod flag {
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use maplit::hashmap;

    use crate::colors::color;

    lazy_static! {
        pub static ref FLAGS: HashMap<&'static str, Vec<color::RGBColor>> = hashmap! {
        "pride" => vec![
            color::PRIDE_RED,
            color::PRIDE_ORANGE,
            color::PRIDE_YELLOW,
            color::PRIDE_GREEN,
            color::PRIDE_INDIGO,
            color::PRIDE_VIOLET,
        ],
        "lesbian" => vec![
            color::LESBIAN_SINOPIA,
            color::LESBIAN_ATOMIC_TANGERINE,
            color::WHITE,
            color::LESBIAN_SUPER_PINK,
            color::LESBIAN_FLIRT,
        ],
        "gay" => vec![
            color::GAY_TEAL,
            color::GAY_TURQUOISE,
            color::GAY_MINT,
            color::WHITE,
            color::GAY_COBALT_BLUE,
            color::GAY_MAJORELLE_BLUE,
            color::GAY_PURPLE,
        ],
        "bisexual" => vec![
            color::BISEXUAL_RUBY,
            color::BISEXUAL_RUBY,
            color::BISEXUAL_MAUVE,
            color::BISEXUAL_BLUE,
            color::BISEXUAL_BLUE,
        ],
        "transgender" => vec![
            color::TRANS_MAYA_BLUE,
            color::TRANS_AMARANTH_PINK,
            color::WHITE,
            color::TRANS_AMARANTH_PINK,
            color::TRANS_MAYA_BLUE,
        ],
        "non-binary" => vec![
            color::ENBY_YELLOW,
            color::WHITE,
            color::ENBY_MAUVE,
            color::ENBY_BLACK,
        ],
        "asexual" => vec![color::BLACK, color::ACE_GREY, color::WHITE, color::PURPLE],
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
