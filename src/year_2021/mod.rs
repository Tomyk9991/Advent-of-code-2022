use seq_macro::seq;
use crate::amount_2021;

macro_rules! pub_mod {
($limit:literal) => {
        seq!(N in 1..=$limit {
            pub mod day~N;
        });
    }
}

amount_2021!(pub_mod);