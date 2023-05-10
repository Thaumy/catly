#[allow(unused_macros)]
macro_rules! std_fraction {
    () => {
        "
type Fraction = { n: Int, d: Int }

def fraction =
    n -> d ->
        let rec gcd =
            a -> b ->
                if eq b 0 then
                    a
                else
                    gcd b (rem a b)
        in
            if or (gt n 1000) (gt d 1000) then
                let
                    g = gcd n d
                in
                    { n = div n g, d = div d g }: Fraction
            else
                { n = n, d = d }: Fraction

def int2F = i -> fraction i 1
"
    };
}
#[allow(unused_imports)]
pub(super) use std_fraction;
