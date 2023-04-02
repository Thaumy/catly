use crate::parser::ast::test::f;

#[test]
fn test_parse_ast_part3() {
    let seq = "\
def fib: Int -> Int =
n ->
  match n with
  | 0 -> 0
  | 1 -> 1
  | _ -> add (fib (sub n 1)) (fib (sub n 2))

type True = Unit
def true = 1: True
type False = Unit
def false = 0: False
type Bool = True | False

type EmptyList = Unit
def emptyList = (): EmptyList
type IntCons = { head: Int, tail: IntCons }
def intCons = h -> t -> { head = h, tail = t }
type IntList = IntCons | EmptyList

def map: (Int -> Int) -> IntList -> IntList =
f -> list ->
  match list with
  | ({ head = _, tail = _ }: IntCons) ->
      ({ head = f head, tail = map f tail }: IntCons)
  | (emptyList: EmptyList) -> emptyList

def find: Int -> IntList -> Bool =
n -> list ->
  match list with
  | ({ head = _, tail = _ }: IntCons) ->
      if eq head n then
        true
      else
        find n tail
  | (emptyList: EmptyList) -> false

def filter: (Int -> Bool) -> IntList -> IntList =
p -> list ->
  match list with
  | ({ head = _, tail = _ }: IntCons) ->
      if p head then
        { head = head, tail = filter p tail }
      else
        filter p tail
  | (emptyList: EmptyList) -> emptyList\
";

    assert!(f(seq).is_some());
}
