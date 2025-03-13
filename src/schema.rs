diesel::table! {
    test (id) {
        id -> Int4,
        bool_value -> Nullable<crate::array_type::ArrayType<Bool>>,
        int_value -> Nullable<crate::array_type::ArrayType<Int4>>,
        float_value -> Nullable<crate::array_type::ArrayType<Float4>>,
        string_value -> Nullable<crate::array_type::ArrayType<Text>>,
    }
}
