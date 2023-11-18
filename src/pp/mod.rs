mod merge_blank;
mod rm_comment;

pub use merge_blank::pp_merge_blank as merge_blank;
pub use rm_comment::pp_rm_comment as rm_comment;

pub fn preprocess(seq: &str) -> String {
    let r = rm_comment(seq);
    merge_blank(&r)
}
