#[macro_export]
macro_rules! int_type {
    () => {
        Type::TypeEnvRef("Int".to_string())
    };
}

#[macro_export]
macro_rules! unit_type {
    () => {
        Type::TypeEnvRef("Unit".to_string())
    };
}

#[macro_export]
macro_rules! discard_type {
    () => {
        Type::TypeEnvRef("Discard".to_string())
    };
}

#[macro_export]
macro_rules! true_type {
    () => {
        Type::TypeEnvRef("Int".to_string())
    };
}

#[macro_export]
macro_rules! false_type {
    () => {
        Type::TypeEnvRef("Int".to_string())
    };
}

#[macro_export]
macro_rules! bool_type {
    () => {
        Type::SumType(btree_set![true_type!(), false_type!(),])
    };
}
