(module $my_module
    (func $cool_func
        (param $first_param i32)
        (param $second_param i32)
        (result i32)
        local.get $first_param
        local.get $second_param
        i32.mul
    )
)