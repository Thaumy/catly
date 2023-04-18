type In = crate::parser::preprocess::name::Out;

pub fn pp_remove_blank(seq: &Vec<In>) -> Vec<In> {
    let r = seq
        .iter()
        .filter(|p| match p {
            In::Symbol(' ') => false,
            _ => true
        })
        .map(|x| x.clone())
        .collect();
    println!("{:8}{:>10} │ {r:?}", "[pp]", "RmBlank");
    r
}
