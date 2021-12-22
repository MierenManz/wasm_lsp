(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func))

  (import "console" "log" (func (param i32)))
  (import "console" "log2" (func $log (param i32)))

  (table $my_tab 2 funcref)
  (elem (i32.const 0) $my_func 2)

  (global i32 (i32.const 4))
  (global $my_global i32 (i32.const 5))
  (global (mut i32) (i32.const 6))
  (global $mut_glob (mut i32) (i32.const 7))

  (func
    $my_func
    (export "myFunction")
    (type $t0)
    (param $first i32)
    (result i32)
    (local $local_param i32)

    i32.const 5
    i32.const 3
    i32.mul
  )

  (func
    (type $t1)
    return
  )
  (export "secondFunction" (func 1))

  (func 
    call 3
  )

  (func
  
    i32.const 2
    call_indirect
  )

  (func 
    return_call 3
  )

  (func
  
    i32.const 2
    return_call_indirect
  )

  (memory $my_memory (export "myMemory") 1)
  (data (i32.const 0) "this is data")
  (export "secondMemoryRef" (memory $my_memory))
  (export "thirdMemRef" (memory 0))

)