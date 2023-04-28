type In = crate::pp::name::Out;
type Out = In;

pub fn pp_remove_blank(seq: &Vec<In>) -> Vec<Out> {
    let r = seq
        .iter()
        .filter(|p| match p {
            In::Symbol(' ') => false,
            _ => true
        })
        .map(|x| x.clone())
        .collect();

    if cfg!(feature = "pp_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "RmBlank");
        println!("{log}");
    }

    r
}
