type In = crate::lexer::name::Out;
type Out = In;

pub fn lexer_remove_blank<S>(seq: S) -> Vec<Out>
where
    S: Iterator<Item = In>
{
    let r = seq
        .filter(|p| !matches!(p, In::Symbol(' ')))
        .collect();

    if cfg!(feature = "pp_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "RmBlank");
        println!("{log}");
    }

    r
}
