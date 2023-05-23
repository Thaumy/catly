use crate::pp::comment::pp_comment;
use crate::pp::merge_blank::pp_merge_blank;

mod comment;
mod merge_blank;

pub fn preprocess(seq: &str) -> String {
    let r = pp_comment(seq);
    pp_merge_blank(&r)
}
