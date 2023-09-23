// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        name -> Nullable<Text>,
        username -> Text,
        body -> Nullable<Text>,
        exercise_type -> Nullable<Text>,
        photo -> Nullable<Binary>,
    }
}
