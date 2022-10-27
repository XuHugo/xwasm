(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32 i32)))
  (type (;8;) (func (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32) (result i64)))
  (type (;11;) (func (param i32 i32 i32 i32)))
  (type (;12;) (func (param i64 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (func $std::rt::lang_start::hec3e200a8398bde9 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 3
    i32.const 16
    local.set 4
    local.get 3
    local.get 4
    i32.sub
    local.set 5
    local.get 5
    global.set 0
    i32.const 1048576
    local.set 6
    local.get 6
    local.set 7
    i32.const 12
    local.set 8
    local.get 5
    local.get 8
    i32.add
    local.set 9
    local.get 9
    local.set 10
    local.get 5
    local.get 0
    i32.store
    local.get 5
    local.get 1
    i32.store offset=4
    local.get 5
    local.get 2
    i32.store offset=8
    local.get 5
    i32.load
    local.set 11
    local.get 5
    local.get 11
    i32.store offset=12
    local.get 5
    i32.load offset=4
    local.set 12
    local.get 5
    i32.load offset=8
    local.set 13
    local.get 10
    local.get 7
    local.get 12
    local.get 13
    call $std::rt::lang_start_internal::h7924b428fced5051
    local.set 14
    i32.const 16
    local.set 15
    local.get 5
    local.get 15
    i32.add
    local.set 16
    local.get 16
    global.set 0
    local.get 14
    return)
  (func $std::rt::lang_start::__closure__::h1c0e9359e1a4f911 (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=12
    local.get 3
    i32.load offset=12
    local.set 4
    local.get 4
    i32.load
    local.set 5
    local.get 5
    call_indirect (type 0)
    call $<___as_std::process::Termination>::report::h224137e2ed084612
    local.set 6
    i32.const 16
    local.set 7
    local.get 3
    local.get 7
    i32.add
    local.set 8
    local.get 8
    global.set 0
    local.get 6
    return)
  (func $core::ops::function::FnOnce::call_once__vtable.shim__::hf8647b93951115d6 (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set 0
    local.get 3
    local.get 0
    i32.store offset=4
    local.get 3
    i32.load offset=4
    local.set 4
    local.get 4
    i32.load
    local.set 5
    local.get 5
    call $core::ops::function::FnOnce::call_once::h8a6c770298a7533e
    local.set 6
    i32.const 16
    local.set 7
    local.get 3
    local.get 7
    i32.add
    local.set 8
    local.get 8
    global.set 0
    local.get 6
    return)
  (func $core::ops::function::FnOnce::call_once::h8a6c770298a7533e (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set 0
    i32.const 4
    local.set 4
    local.get 3
    local.get 4
    i32.add
    local.set 5
    local.get 5
    local.set 6
    local.get 3
    local.get 0
    i32.store offset=4
    local.get 6
    call $std::rt::lang_start::__closure__::h1c0e9359e1a4f911
    local.set 7
    i32.const 16
    local.set 8
    local.get 3
    local.get 8
    i32.add
    local.set 9
    local.get 9
    global.set 0
    local.get 7
    return)
  (func $core::ptr::real_drop_in_place::h3bc6cfbc64ffdde6 (type 1) (param i32)
    (local i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    local.get 0
    i32.store offset=12
    return)
  (func $core::fmt::Arguments::new_v1::h459eb86d15d4762a (type 7) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 5
    i32.const 32
    local.set 6
    local.get 5
    local.get 6
    i32.sub
    local.set 7
    i32.const 0
    local.set 8
    local.get 7
    local.get 1
    i32.store offset=8
    local.get 7
    local.get 2
    i32.store offset=12
    local.get 7
    local.get 3
    i32.store offset=16
    local.get 7
    local.get 4
    i32.store offset=20
    local.get 7
    i32.load offset=8
    local.set 9
    local.get 7
    i32.load offset=12
    local.set 10
    local.get 7
    local.get 8
    i32.store offset=24
    local.get 7
    i32.load offset=16
    local.set 11
    local.get 7
    i32.load offset=20
    local.set 12
    local.get 0
    local.get 9
    i32.store
    local.get 0
    local.get 10
    i32.store offset=4
    local.get 7
    i32.load offset=24
    local.set 13
    local.get 7
    i32.load offset=28
    local.set 14
    local.get 0
    local.get 13
    i32.store offset=8
    local.get 0
    local.get 14
    i32.store offset=12
    local.get 0
    local.get 11
    i32.store offset=16
    local.get 0
    local.get 12
    i32.store offset=20
    return)
  (func $hw_rust::main::ha1617da2eac3802e (type 0)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 0
    i32.const 32
    local.set 1
    local.get 0
    local.get 1
    i32.sub
    local.set 2
    local.get 2
    global.set 0
    i32.const 8
    local.set 3
    local.get 2
    local.get 3
    i32.add
    local.set 4
    local.get 4
    local.set 5
    i32.const 1048616
    local.set 6
    local.get 6
    local.set 7
    i32.const 1
    local.set 8
    i32.const 4
    local.set 9
    i32.const 0
    local.set 10
    local.get 5
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $core::fmt::Arguments::new_v1::h459eb86d15d4762a
    i32.const 8
    local.set 11
    local.get 2
    local.get 11
    i32.add
    local.set 12
    local.get 12
    local.set 13
    local.get 13
    call $std::io::stdio::_print::hc753675f57d09bbc
    i32.const 32
    local.set 14
    local.get 2
    local.get 14
    i32.add
    local.set 15
    local.get 15
    global.set 0
    return)
  (func $main (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 4
    local.set 2
    i32.const 0
    local.set 3
    local.get 3
    i32.load8_u offset=1056576
    drop
    local.get 2
    local.get 0
    local.get 1
    call $std::rt::lang_start::hec3e200a8398bde9
    local.set 4
    local.get 4
    return)
  (func $<___as_std::process::Termination>::report::h224137e2ed084612 (type 8) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 0
    i32.const 16
    local.set 1
    local.get 0
    local.get 1
    i32.sub
    local.set 2
    local.get 2
    global.set 0
    i32.const 0
    local.set 3
    i32.const 1
    local.set 4
    local.get 3
    local.get 4
    i32.and
    local.set 5
    local.get 5
    call $<std::process::ExitCode_as_std::process::Termination>::report::hdb1e76ab3304298b
    local.set 6
    i32.const 16
    local.set 7
    local.get 2
    local.get 7
    i32.add
    local.set 8
    local.get 8
    global.set 0
    local.get 6
    return)
  (func $<std::process::ExitCode_as_std::process::Termination>::report::hdb1e76ab3304298b (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set 0
    i32.const 15
    local.set 4
    local.get 3
    local.get 4
    i32.add
    local.set 5
    local.get 5
    local.set 6
    local.get 0
    local.set 7
    local.get 3
    local.get 7
    i32.store8 offset=15
    local.get 6
    call $std::sys::wasm::process::ExitCode::as_i32::h4c9c3e9590aabf51
    local.set 8
    i32.const 16
    local.set 9
    local.get 3
    local.get 9
    i32.add
    local.set 10
    local.get 10
    global.set 0
    local.get 8
    return)
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc
    local.set 2
    local.get 2
    return)
  (func $__rust_dealloc (type 3) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rdl_dealloc
    return)
  (func $__rust_realloc (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__rdl_realloc
    local.set 4
    local.get 4
    return)
  (func $<T_as_core::any::Any>::type_id::h047c16fec401b221 (type 10) (param i32) (result i64)
    i64.const 6308721582299515157)
  (func $<T_as_core::any::Any>::type_id::h2d4d17f20cb15612 (type 10) (param i32) (result i64)
    i64.const -2918786428776706287)
  (func $<T_as_core::any::Any>::type_id::h8b736fbe99f2e981 (type 10) (param i32) (result i64)
    i64.const 1229646359891580772)
  (func $<T_as_core::any::Any>::type_id::had50064618b8bd18 (type 10) (param i32) (result i64)
    i64.const 7549865886324542212)
  (func $<&T_as_core::fmt::Debug>::fmt::h090d81485cc71665 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      call $core::fmt::Formatter::debug_lower_hex::hf149757ee45f6e30
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        call $core::fmt::Formatter::debug_upper_hex::h0919786d4217197b
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $core::fmt::num::imp::<impl_core::fmt::Display_for_u32>::fmt::h3518dbff2fc7fe22
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl_core::fmt::UpperHex_for_i32>::fmt::h288c0aa06d70df28
      return
    end
    local.get 0
    local.get 1
    call $core::fmt::num::<impl_core::fmt::LowerHex_for_i32>::fmt::h09b955a98f459559)
  (func $<&T_as_core::fmt::Debug>::fmt::h1874024e098cc5f9 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      call $core::fmt::Formatter::debug_lower_hex::hf149757ee45f6e30
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        call $core::fmt::Formatter::debug_upper_hex::h0919786d4217197b
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $core::fmt::num::imp::<impl_core::fmt::Display_for_u8>::fmt::he44c0f717551868b
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl_core::fmt::UpperHex_for_i8>::fmt::hd760e3f648143fcc
      return
    end
    local.get 0
    local.get 1
    call $core::fmt::num::<impl_core::fmt::LowerHex_for_i8>::fmt::he8f810381047cecd)
  (func $<&T_as_core::fmt::Debug>::fmt::hdf6f72ae8da72177 (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.tee 0
    i32.load offset=8
    local.set 3
    local.get 0
    i32.load
    local.set 0
    local.get 2
    local.get 1
    call $core::fmt::Formatter::debug_list::h310f1e1cc9ab1b89
    block  ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 1048672
        call $core::fmt::builders::DebugSet::entry::h201353a235e5dda9
        drop
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const -1
        i32.add
        local.tee 3
        br_if 0 (;@2;)
      end
    end
    local.get 2
    call $core::fmt::builders::DebugList::finish::h5e29443e3f085b64
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func $<&T_as_core::fmt::Display>::fmt::h251750df0dd9a432 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $<str_as_core::fmt::Display>::fmt::h38ffb460df0bf4b9)
  (func $core::fmt::Write::write_char::h2ef624cf48b9cdf4 (type 2) (param i32 i32) (result i32)
    (local i32 i64 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 224
            i32.or
            i32.store8 offset=4
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=7
          local.get 2
          local.get 1
          i32.const 18
          i32.shr_u
          i32.const 240
          i32.or
          i32.store8 offset=4
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=6
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=5
          i32.const 4
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=4
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=5
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 31
      i32.and
      i32.const 192
      i32.or
      i32.store8 offset=4
      i32.const 2
      local.set 1
    end
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.load
    local.get 2
    i32.const 4
    i32.add
    local.get 1
    call $std::io::Write::write_all::h24f1b9deb3916b44
    i32.const 0
    local.set 1
    block  ;; label = @1
      local.get 2
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 2
      i64.load offset=8
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 1
        i32.load
        local.get 1
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 4
          i32.load offset=4
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.get 5
          local.get 4
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      local.get 3
      i64.store offset=4 align=4
      i32.const 1
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $std::io::Write::write_all::h24f1b9deb3916b44 (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 5
                    i32.or
                    local.set 5
                    loop  ;; label = @9
                      local.get 1
                      i32.load
                      local.tee 6
                      i32.load offset=4
                      br_if 4 (;@5;)
                      local.get 6
                      i32.const -1
                      i32.store offset=4
                      local.get 4
                      i32.const 8
                      i32.add
                      local.get 6
                      i32.const 8
                      i32.add
                      local.get 2
                      local.get 3
                      call $<std::io::buffered::LineWriter<W>_as_std::io::Write>::write::h447b0ce154db372f
                      local.get 6
                      local.get 6
                      i32.load offset=4
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.load offset=8
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 4
                            i32.load offset=12
                            local.tee 6
                            br_if 0 (;@12;)
                            i32.const 28
                            i32.const 1
                            call $__rust_alloc
                            local.tee 6
                            i32.eqz
                            br_if 8 (;@4;)
                            local.get 6
                            i32.const 24
                            i32.add
                            i32.const 0
                            i32.load offset=1049970 align=1
                            i32.store align=1
                            local.get 6
                            i32.const 16
                            i32.add
                            i32.const 0
                            i64.load offset=1049962 align=1
                            i64.store align=1
                            local.get 6
                            i32.const 8
                            i32.add
                            i32.const 0
                            i64.load offset=1049954 align=1
                            i64.store align=1
                            local.get 6
                            i32.const 0
                            i64.load offset=1049946 align=1
                            i64.store align=1
                            i32.const 12
                            i32.const 4
                            call $__rust_alloc
                            local.tee 3
                            i32.eqz
                            br_if 9 (;@3;)
                            local.get 3
                            i64.const 120259084316
                            i64.store offset=4 align=4
                            local.get 3
                            local.get 6
                            i32.store
                            i32.const 12
                            i32.const 4
                            call $__rust_alloc
                            local.tee 6
                            br_if 6 (;@6;)
                            i32.const 12
                            i32.const 4
                            call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
                            unreachable
                          end
                          local.get 3
                          local.get 6
                          i32.lt_u
                          br_if 9 (;@2;)
                          local.get 2
                          local.get 6
                          i32.add
                          local.set 2
                          local.get 3
                          local.get 6
                          i32.sub
                          local.set 3
                          br 1 (;@10;)
                        end
                        local.get 5
                        local.set 6
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 4
                            i32.load8_u offset=12
                            local.tee 7
                            br_table 5 (;@7;) 1 (;@11;) 0 (;@12;) 5 (;@7;)
                          end
                          local.get 4
                          i32.load offset=16
                          i32.const 8
                          i32.add
                          local.set 6
                        end
                        local.get 6
                        i32.load8_u
                        i32.const 15
                        i32.ne
                        br_if 3 (;@7;)
                        local.get 7
                        i32.const 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.load offset=16
                        local.tee 6
                        i32.load
                        local.get 6
                        i32.load offset=4
                        i32.load
                        call_indirect (type 1)
                        block  ;; label = @11
                          local.get 6
                          i32.load offset=4
                          local.tee 7
                          i32.load offset=4
                          local.tee 8
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 6
                          i32.load
                          local.get 8
                          local.get 7
                          i32.load offset=8
                          call $__rust_dealloc
                        end
                        local.get 6
                        i32.const 12
                        i32.const 4
                        call $__rust_dealloc
                      end
                      local.get 3
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.const 3
                  i32.store8
                  br 6 (;@1;)
                end
                local.get 0
                local.get 4
                i64.load offset=12 align=4
                i64.store align=4
                br 5 (;@1;)
              end
              local.get 6
              i32.const 14
              i32.store8 offset=8
              local.get 6
              i32.const 1049440
              i32.store offset=4
              local.get 6
              local.get 3
              i32.store
              local.get 6
              local.get 4
              i32.load16_u offset=24 align=1
              i32.store16 offset=9 align=1
              local.get 6
              i32.const 11
              i32.add
              local.get 4
              i32.const 24
              i32.add
              i32.const 2
              i32.add
              i32.load8_u
              i32.store8
              local.get 0
              i32.const 4
              i32.add
              local.get 6
              i32.store
              local.get 0
              i32.const 2
              i32.store
              br 4 (;@1;)
            end
            i32.const 1048688
            i32.const 16
            local.get 4
            i32.const 24
            i32.add
            i32.const 1048976
            call $core::result::unwrap_failed::hd11b409f5ba7889e
            unreachable
          end
          i32.const 28
          i32.const 1
          call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
          unreachable
        end
        i32.const 12
        i32.const 4
        call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
        unreachable
      end
      local.get 6
      local.get 3
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 4
    i32.const 32
    i32.add
    global.set 0)
  (func $core::fmt::Write::write_fmt::h87ff8cf34eea5f27 (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048648
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $std::panicking::begin_panic::h837c5ed1e256db31 (type 3) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=12
    local.get 3
    local.get 0
    i32.store offset=8
    local.get 3
    i32.const 8
    i32.add
    i32.const 1050552
    i32.const 0
    local.get 2
    call $std::panicking::rust_panic_with_hook::h5e7c2dc110ae79d4
    unreachable)
  (func $core::ops::function::FnOnce::call_once__vtable.shim__::h567968cd02331d13 (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $std::sync::once::Once::call_once::__closure__::h0dd633692ce85f20
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $std::sync::once::Once::call_once::__closure__::h0dd633692ce85f20 (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load
    local.tee 0
    i32.load8_u
    local.set 2
    local.get 0
    i32.const 0
    i32.store8
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1057145
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1056632
                local.set 4
                i32.const 0
                local.get 3
                i32.const 10
                i32.eq
                i32.store offset=1056632
                i32.const 0
                i32.const 0
                i32.store8 offset=1057145
                block  ;; label = @7
                  local.get 4
                  i32.const 1
                  i32.gt_u
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 4
                    br_table 5 (;@3;) 0 (;@8;) 5 (;@3;)
                  end
                  i32.const 1050352
                  i32.const 31
                  i32.const 1050336
                  call $std::panicking::begin_panic::h837c5ed1e256db31
                  unreachable
                end
                local.get 4
                i32.load
                local.tee 5
                local.get 4
                i32.load offset=8
                local.tee 2
                i32.const 3
                i32.shl
                i32.add
                local.set 6
                local.get 4
                i32.load offset=4
                local.set 7
                local.get 5
                local.set 0
                local.get 2
                i32.eqz
                br_if 1 (;@5;)
                local.get 5
                local.set 0
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load
                    local.tee 2
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 0
                    br 3 (;@5;)
                  end
                  local.get 2
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  call $<alloc::boxed::Box<F>_as_core::ops::function::FnOnce<A>>::call_once::h6b65f798dfc4041d
                  local.get 0
                  i32.const 8
                  i32.add
                  local.tee 0
                  local.get 6
                  i32.ne
                  br_if 0 (;@7;)
                  br 3 (;@4;)
                end
              end
              i32.const 1050772
              i32.const 32
              i32.const 1050756
              call $std::panicking::begin_panic::h837c5ed1e256db31
              unreachable
            end
            local.get 0
            local.get 6
            i32.eq
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 0
              i32.load
              local.tee 2
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 8
              i32.load
              call_indirect (type 1)
              block  ;; label = @6
                local.get 8
                i32.load offset=4
                local.tee 9
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                local.get 9
                local.get 8
                i32.load offset=8
                call $__rust_dealloc
              end
              local.get 0
              i32.const 8
              i32.add
              local.tee 0
              local.get 6
              i32.ne
              br_if 0 (;@5;)
            end
          end
          block  ;; label = @4
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 7
            i32.const 3
            i32.shl
            i32.const 4
            call $__rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 3
        local.get 3
        i32.const 10
        i32.lt_u
        local.tee 0
        i32.add
        local.set 3
        local.get 0
        br_if 0 (;@2;)
      end
      return
    end
    i32.const 1048920
    call $core::panicking::panic::h0142ee7f4c64bd08
    unreachable)
  (func $core::ops::function::FnOnce::call_once__vtable.shim__::hb430745993719cf4 (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store8 offset=4
      local.get 0
      i32.load
      local.set 1
      local.get 0
      i32.const 1
      i32.store
      local.get 1
      i32.load
      local.tee 0
      local.get 0
      i32.load
      local.tee 0
      i32.const -1
      i32.add
      i32.store
      block  ;; label = @2
        local.get 0
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        call $alloc::sync::Arc<T>::drop_slow::hf5b61bcc786904c0
      end
      local.get 1
      i32.const 4
      i32.const 4
      call $__rust_dealloc
      return
    end
    i32.const 1050772
    i32.const 32
    i32.const 1050756
    call $std::panicking::begin_panic::h837c5ed1e256db31
    unreachable)
  (func $alloc::sync::Arc<T>::drop_slow::hf5b61bcc786904c0 (type 1) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 28
      i32.add
      i32.load8_u
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      i32.const 29
      i32.add
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 24
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store offset=24
      local.get 1
      i32.const 0
      i32.store8 offset=29
    end
    block  ;; label = @1
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=16
      local.get 2
      i32.const 1
      call $__rust_dealloc
    end
    local.get 0
    i32.load
    local.tee 1
    local.get 1
    i32.load offset=4
    local.tee 1
    i32.const -1
    i32.add
    i32.store offset=4
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load
      i32.const 40
      i32.const 4
      call $__rust_dealloc
    end)
  (func $core::ptr::real_drop_in_place::h08b326c460981070 (type 1) (param i32))
  (func $core::ptr::real_drop_in_place::h1c5271a22a4bbc56 (type 1) (param i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u offset=4
        i32.const 2
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.get 3
        local.get 2
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 0
      i32.load offset=8
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end)
  (func $core::ptr::real_drop_in_place::h2621533cc62ca9fc (type 1) (param i32))
  (func $core::ptr::real_drop_in_place::h481a15a182dcb798 (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end)
  (func $core::ptr::real_drop_in_place::h6f9e84b48a4387b2 (type 1) (param i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 0
        i32.load offset=1057136
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        i32.const 0
        i64.const 1
        i64.store offset=1057136
        br 1 (;@1;)
      end
      i32.const 0
      i32.load offset=1057140
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      i32.const 1
      i32.store8 offset=4
    end
    local.get 0
    i32.load
    i32.load
    i32.const 0
    i32.store8)
  (func $core::ptr::real_drop_in_place::hed899f15376b0cdc (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end)
  (func $core::ptr::real_drop_in_place::hff6df1afa53ab3b9 (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $core::option::Option<T>::unwrap::h684599df4939e5f6 (type 5) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048920
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    local.get 0)
  (func $core::option::Option<T>::unwrap::hc5bf9494982dd003 (type 5) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048920
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    local.get 0)
  (func $<&mut_W_as_core::fmt::Write>::write_char::h29fafe67e786b5e9 (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 224
              i32.or
              i32.store8 offset=12
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load offset=4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
            local.get 0
            i32.load offset=8
            local.set 3
          end
          local.get 0
          i32.load
          local.get 3
          i32.add
          local.get 1
          i32.store8
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 31
        i32.and
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      local.get 0
      local.get 1
      call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
      local.get 0
      local.get 0
      i32.load offset=8
      local.tee 3
      local.get 1
      i32.add
      i32.store offset=8
      local.get 3
      local.get 0
      i32.load
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4 (type 4) (param i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.add
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 2
          i32.const 1
          i32.shl
          local.tee 3
          local.get 1
          local.get 3
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 1
              i32.const 1
              call $__rust_alloc
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            i32.load
            local.get 2
            i32.const 1
            local.get 1
            call $__rust_realloc
            local.set 2
          end
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store
        end
        return
      end
      local.get 1
      i32.const 1
      call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
      unreachable
    end
    call $alloc::raw_vec::capacity_overflow::hc538c246d520d486
    unreachable)
  (func $<&mut_W_as_core::fmt::Write>::write_char::hdd43bb9ce4ee3aa1 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $core::fmt::Write::write_char::h2ef624cf48b9cdf4)
  (func $<&mut_W_as_core::fmt::Write>::write_fmt::h2b2a24f11dbb5e86 (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048624
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $<&mut_W_as_core::fmt::Write>::write_fmt::ha9d8918552803d5f (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048648
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $<&mut_W_as_core::fmt::Write>::write_str::h292f3bef30be5ae9 (type 6) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 2
    call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
    local.get 0
    local.get 0
    i32.load offset=8
    local.tee 3
    local.get 2
    i32.add
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    i32.const 0)
  (func $<&mut_W_as_core::fmt::Write>::write_str::h6cf1ca1d364d8309 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i64 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    i32.load
    local.tee 0
    i32.load
    local.get 1
    local.get 2
    call $std::io::Write::write_all::h24f1b9deb3916b44
    i32.const 0
    local.set 1
    block  ;; label = @1
      local.get 3
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.load offset=8
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 1
        i32.load
        local.get 1
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 2
          i32.load offset=4
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.get 5
          local.get 2
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      local.get 4
      i64.store offset=4 align=4
      i32.const 1
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab (type 1) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 16
      i32.add
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 0
      i32.store8
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=16
      local.get 2
      i32.const 1
      call $__rust_dealloc
    end
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.const 1
    i32.const 1
    call $__rust_dealloc
    local.get 0
    i32.load
    local.tee 1
    local.get 1
    i32.load offset=4
    local.tee 1
    i32.const -1
    i32.add
    i32.store offset=4
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load
      i32.const 48
      i32.const 8
      call $__rust_dealloc
    end)
  (func $<alloc::string::String_as_core::fmt::Display>::fmt::h1c3337deac11fc7c (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.get 1
    call $<str_as_core::fmt::Display>::fmt::h38ffb460df0bf4b9)
  (func $<alloc::boxed::Box<F>_as_core::ops::function::FnOnce<A>>::call_once::h6b65f798dfc4041d (type 4) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    local.tee 2
    local.set 3
    local.get 2
    local.get 1
    i32.load offset=4
    local.tee 4
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    local.get 4
    call $memcpy
    local.get 1
    i32.load offset=12
    call_indirect (type 1)
    block  ;; label = @1
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 4
      local.get 1
      i32.load offset=8
      call $__rust_dealloc
    end
    local.get 3
    global.set 0)
  (func $std::sys_common::thread_info::ThreadInfo::with::__closure__::h34502497b740844d (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load
            local.tee 2
            i32.const 1
            i32.add
            i32.const 0
            i32.le_s
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.store
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 3
              br_if 0 (;@5;)
              local.get 1
              i32.const 0
              i32.store offset=8
              local.get 1
              i32.const 8
              i32.add
              call $std::thread::Thread::new::h5b7d617e777ade8f
              local.set 3
              local.get 0
              i32.load
              br_if 2 (;@3;)
              local.get 0
              i32.const -1
              i32.store
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                local.get 2
                i32.load
                local.tee 4
                i32.const -1
                i32.add
                i32.store
                local.get 4
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 4
                i32.add
                call $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab
              end
              local.get 0
              local.get 3
              i32.store offset=4
              local.get 0
              local.get 0
              i32.load
              i32.const 1
              i32.add
              local.tee 2
              i32.store
            end
            local.get 2
            br_if 2 (;@2;)
            local.get 0
            i32.const -1
            i32.store
            local.get 3
            local.get 3
            i32.load
            local.tee 2
            i32.const 1
            i32.add
            i32.store
            local.get 2
            i32.const -1
            i32.le_s
            br_if 3 (;@1;)
            local.get 0
            local.get 0
            i32.load
            i32.const 1
            i32.add
            i32.store
            local.get 1
            i32.const 32
            i32.add
            global.set 0
            local.get 3
            return
          end
          i32.const 1048704
          i32.const 24
          local.get 1
          i32.const 24
          i32.add
          i32.const 1048960
          call $core::result::unwrap_failed::hd11b409f5ba7889e
          unreachable
        end
        i32.const 1048688
        i32.const 16
        local.get 1
        i32.const 24
        i32.add
        i32.const 1048976
        call $core::result::unwrap_failed::hd11b409f5ba7889e
        unreachable
      end
      i32.const 1048688
      i32.const 16
      local.get 1
      i32.const 24
      i32.add
      i32.const 1048976
      call $core::result::unwrap_failed::hd11b409f5ba7889e
      unreachable
    end
    unreachable
    unreachable)
  (func $std::thread::park::h0838d30e3c3b47de (type 0)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      i32.const 0
      i32.load offset=1056672
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      i64.const 1
      i64.store offset=1056672 align=4
      i32.const 0
      i32.const 0
      i32.store offset=1056680
    end
    i32.const 1056676
    call $std::sys_common::thread_info::ThreadInfo::with::__closure__::h34502497b740844d
    local.tee 1
    i32.const 0
    local.get 1
    i32.load offset=24
    local.tee 2
    local.get 2
    i32.const 2
    i32.eq
    local.tee 2
    select
    i32.store offset=24
    local.get 0
    local.get 1
    i32.store offset=8
    block  ;; label = @1
      local.get 2
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            i32.const 28
            i32.add
            local.tee 4
            i32.load
            local.tee 1
            i32.load8_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 1
            i32.store8
            i32.const 0
            local.set 2
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1057136
                i32.const 1
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.load offset=1057140
                local.set 2
                br 1 (;@5;)
              end
              i32.const 0
              i64.const 1
              i64.store offset=1057136
            end
            i32.const 0
            local.get 2
            i32.store offset=1057140
            local.get 3
            i32.const 32
            i32.add
            i32.load8_u
            br_if 1 (;@3;)
            local.get 3
            i32.const 24
            i32.add
            local.tee 1
            local.get 1
            i32.load
            local.tee 1
            i32.const 1
            local.get 1
            select
            i32.store
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 1
                i32.const 2
                i32.eq
                br_if 0 (;@6;)
                i32.const 1049192
                i32.const 23
                i32.const 1049176
                call $std::panicking::begin_panic::h837c5ed1e256db31
                unreachable
              end
              local.get 0
              i32.load offset=8
              i32.const 24
              i32.add
              local.tee 5
              i32.load
              local.set 1
              local.get 5
              i32.const 0
              i32.store
              local.get 0
              local.get 1
              i32.store offset=12
              local.get 1
              i32.const 2
              i32.ne
              br_if 3 (;@2;)
              block  ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1057136
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  i32.const 0
                  i64.const 1
                  i64.store offset=1057136
                  br 1 (;@6;)
                end
                i32.const 0
                i32.load offset=1057140
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 1
                i32.store8 offset=32
              end
              local.get 4
              i32.load
              i32.const 0
              i32.store8
              br 4 (;@1;)
            end
            local.get 0
            i32.load offset=8
            i32.const 36
            i32.add
            local.tee 0
            local.get 4
            i32.load
            local.tee 1
            call $std::sync::condvar::Condvar::verify::h8b5040a5a080d804
            local.get 0
            i32.load
            local.get 1
            call $std::sys_common::condvar::Condvar::wait::h07e73ff239a34dad
            unreachable
          end
          i32.const 1050772
          i32.const 32
          i32.const 1050756
          call $std::panicking::begin_panic::h837c5ed1e256db31
          unreachable
        end
        local.get 0
        local.get 4
        i32.store offset=72
        local.get 0
        local.get 2
        i32.const 0
        i32.ne
        i32.store8 offset=76
        i32.const 1048992
        i32.const 43
        local.get 0
        i32.const 72
        i32.add
        i32.const 1049036
        call $core::result::unwrap_failed::hd11b409f5ba7889e
        unreachable
      end
      local.get 0
      i32.const 40
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get 0
      i32.const 52
      i32.add
      i32.const 6
      i32.store
      local.get 0
      i32.const 16
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 0
      local.get 0
      i32.const 12
      i32.add
      i32.store offset=64
      local.get 0
      i32.const 1049216
      i32.store offset=68
      local.get 0
      i64.const 3
      i64.store offset=20 align=4
      local.get 0
      i32.const 1048832
      i32.store offset=16
      local.get 0
      i32.const 6
      i32.store offset=44
      local.get 0
      i64.const 4
      i64.store offset=88
      local.get 0
      i64.const 1
      i64.store offset=76 align=4
      local.get 0
      i32.const 1049252
      i32.store offset=72
      local.get 0
      local.get 0
      i32.const 40
      i32.add
      i32.store offset=32
      local.get 0
      local.get 0
      i32.const 72
      i32.add
      i32.store offset=56
      local.get 0
      local.get 0
      i32.const 68
      i32.add
      i32.store offset=48
      local.get 0
      local.get 0
      i32.const 64
      i32.add
      i32.store offset=40
      local.get 0
      i32.const 16
      i32.add
      i32.const 1049260
      call $std::panicking::begin_panic_fmt::h62d8474306d91923
      unreachable
    end
    local.get 0
    i32.load offset=8
    local.tee 1
    local.get 1
    i32.load
    local.tee 1
    i32.const -1
    i32.add
    i32.store
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      call $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab
    end
    local.get 0
    i32.const 96
    i32.add
    global.set 0)
  (func $std::sync::condvar::Condvar::verify::h8b5040a5a080d804 (type 4) (param i32 i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load offset=4
    local.tee 2
    local.get 1
    local.get 2
    select
    i32.store offset=4
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1050060
      i32.const 54
      i32.const 1050044
      call $std::panicking::begin_panic::h837c5ed1e256db31
      unreachable
    end)
  (func $std::sys_common::condvar::Condvar::wait::h07e73ff239a34dad (type 4) (param i32 i32)
    (local i32)
    local.get 2
    local.get 2
    call $std::sys::wasm::condvar::Condvar::wait::hda825f3f4310d4db
    unreachable)
  (func $std::panicking::begin_panic_fmt::h62d8474306d91923 (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 32
    i32.add
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call $core::panic::Location::internal_constructor::hcf293bdd1161e916
    local.get 2
    i32.const 20
    i32.add
    local.get 2
    i32.const 40
    i32.add
    i64.load
    i64.store align=4
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 1048812
    i32.store offset=4
    local.get 2
    i32.const 1
    i32.store
    local.get 2
    local.get 2
    i64.load offset=32
    i64.store offset=12 align=4
    local.get 2
    call $std::panicking::continue_panic_fmt::hb5b3e4b5160fe2ab
    unreachable)
  (func $std::thread::Thread::new::h5b7d617e777ade8f (type 5) (param i32) (result i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 2
                br_if 0 (;@6;)
                i32.const 0
                local.set 3
                br 1 (;@5;)
              end
              local.get 1
              local.get 0
              i64.load offset=4 align=4
              i64.store offset=36 align=4
              local.get 1
              local.get 2
              i32.store offset=32
              local.get 1
              i32.const 16
              i32.add
              local.get 1
              i32.const 32
              i32.add
              call $alloc::string::<impl_core::convert::From<alloc::string::String>_for_alloc::vec::Vec<u8>>::from::h6ba865a046d5c1cb
              local.get 1
              i32.const 8
              i32.add
              i32.const 0
              local.get 1
              i32.load offset=16
              local.tee 0
              local.get 1
              i32.load offset=24
              call $core::slice::memchr::memchr::h2031004febcdc2f2
              local.get 1
              i32.load offset=8
              br_if 1 (;@4;)
              local.get 1
              i32.const 32
              i32.add
              i32.const 8
              i32.add
              local.get 1
              i32.const 16
              i32.add
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get 1
              local.get 1
              i64.load offset=16
              i64.store offset=32
              local.get 1
              local.get 1
              i32.const 32
              i32.add
              call $std::ffi::c_str::CString::from_vec_unchecked::h0b52e8b5109f9666
              local.get 1
              i32.load offset=4
              local.set 4
              local.get 1
              i32.load
              local.set 3
            end
            i32.const 0
            i32.load8_u offset=1057144
            br_if 1 (;@3;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1057144
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i64.load offset=1056616
                local.tee 5
                i64.const -1
                i64.eq
                br_if 0 (;@6;)
                i32.const 0
                local.get 5
                i64.const 1
                i64.add
                i64.store offset=1056616
                local.get 5
                i64.const 0
                i64.ne
                br_if 1 (;@5;)
                i32.const 1048920
                call $core::panicking::panic::h0142ee7f4c64bd08
                unreachable
              end
              i32.const 1049292
              i32.const 55
              i32.const 1049276
              call $std::panicking::begin_panic::h837c5ed1e256db31
              unreachable
            end
            i32.const 0
            i32.const 0
            i32.store8 offset=1057144
            i32.const 1
            i32.const 1
            call $__rust_alloc
            local.tee 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.const 0
            i32.store8
            i32.const 48
            i32.const 8
            call $__rust_alloc
            local.tee 0
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            i64.const 1
            i64.store offset=36 align=4
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            local.get 4
            i32.store offset=20
            local.get 0
            local.get 3
            i32.store offset=16
            local.get 0
            local.get 5
            i64.store offset=8
            local.get 0
            i64.const 4294967297
            i64.store
            local.get 0
            local.get 2
            i64.extend_i32_u
            i64.store offset=28 align=4
            local.get 1
            i32.const 48
            i32.add
            global.set 0
            local.get 0
            return
          end
          local.get 1
          i32.load offset=12
          local.set 2
          local.get 1
          i32.const 40
          i32.add
          local.get 1
          i64.load offset=20 align=4
          i64.store
          local.get 1
          local.get 0
          i32.store offset=36
          local.get 1
          local.get 2
          i32.store offset=32
          i32.const 1049347
          i32.const 47
          local.get 1
          i32.const 32
          i32.add
          i32.const 1048944
          call $core::result::unwrap_failed::hd11b409f5ba7889e
          unreachable
        end
        i32.const 1050772
        i32.const 32
        i32.const 1050756
        call $std::panicking::begin_panic::h837c5ed1e256db31
        unreachable
      end
      i32.const 1
      i32.const 1
      call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
      unreachable
    end
    i32.const 48
    i32.const 8
    call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
    unreachable)
  (func $std::ffi::c_str::CString::from_vec_unchecked::h0b52e8b5109f9666 (type 4) (param i32 i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=4
              local.tee 2
              local.get 1
              i32.load offset=8
              local.tee 3
              i32.ne
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              i32.add
              local.tee 2
              local.get 3
              i32.lt_u
              br_if 2 (;@3;)
              local.get 2
              i32.const 0
              i32.lt_s
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 1
                  call $__rust_alloc
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 1
                i32.load
                local.get 3
                i32.const 1
                local.get 2
                call $__rust_realloc
                local.set 4
              end
              local.get 4
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              local.get 2
              i32.store offset=4
              local.get 1
              local.get 4
              i32.store
            end
            block  ;; label = @5
              local.get 3
              local.get 2
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 1
              call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
              local.get 1
              i32.load offset=4
              local.set 2
              local.get 1
              i32.load offset=8
              local.set 3
            end
            local.get 1
            local.get 3
            i32.const 1
            i32.add
            local.tee 4
            i32.store offset=8
            local.get 1
            i32.load
            local.tee 5
            local.get 3
            i32.add
            i32.const 0
            i32.store8
            block  ;; label = @5
              local.get 2
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 1
              local.get 2
              local.set 4
              br 4 (;@1;)
            end
            local.get 2
            local.get 4
            i32.lt_u
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              i32.const 1
              local.set 1
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 5
              local.get 2
              i32.const 1
              call $__rust_dealloc
              br 4 (;@1;)
            end
            local.get 5
            local.get 2
            i32.const 1
            local.get 4
            call $__rust_realloc
            local.tee 1
            br_if 3 (;@1;)
            local.get 4
            i32.const 1
            call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
            unreachable
          end
          local.get 2
          i32.const 1
          call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
          unreachable
        end
        call $alloc::raw_vec::capacity_overflow::hc538c246d520d486
        unreachable
      end
      i32.const 1049112
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $std::thread::Thread::unpark::h51f257038f7af062 (type 1) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load
    i32.const 24
    i32.add
    local.tee 2
    i32.load
    local.set 3
    local.get 2
    i32.const 2
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 2
              i32.gt_u
              br_if 0 (;@5;)
              local.get 3
              br_table 2 (;@3;) 1 (;@4;) 2 (;@3;) 2 (;@3;)
            end
            i32.const 1049412
            i32.const 28
            i32.const 1049396
            call $std::panicking::begin_panic::h837c5ed1e256db31
            unreachable
          end
          local.get 0
          i32.load
          local.tee 0
          i32.const 28
          i32.add
          local.tee 2
          i32.load
          local.tee 3
          i32.load8_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 1
          i32.store8
          i32.const 0
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load offset=1057136
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1057140
              local.set 3
              br 1 (;@4;)
            end
            i32.const 0
            i64.const 1
            i64.store offset=1057136
          end
          i32.const 0
          local.get 3
          i32.store offset=1057140
          local.get 0
          i32.const 32
          i32.add
          i32.load8_u
          br_if 2 (;@1;)
          local.get 2
          i32.load
          i32.const 0
          i32.store8
        end
        local.get 1
        i32.const 16
        i32.add
        global.set 0
        return
      end
      i32.const 1050772
      i32.const 32
      i32.const 1050756
      call $std::panicking::begin_panic::h837c5ed1e256db31
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=8
    local.get 1
    local.get 3
    i32.const 0
    i32.ne
    i32.store8 offset=12
    i32.const 1048992
    i32.const 43
    local.get 1
    i32.const 8
    i32.add
    i32.const 1049036
    call $core::result::unwrap_failed::hd11b409f5ba7889e
    unreachable)
  (func $<std::io::error::Error_as_core::fmt::Display>::fmt::h98edb03f11a4710e (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load8_u
              br_table 0 (;@5;) 2 (;@3;) 1 (;@4;) 0 (;@5;)
            end
            local.get 2
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.store offset=4
            i32.const 20
            i32.const 1
            call $__rust_alloc
            local.tee 0
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            i32.const 16
            i32.add
            i32.const 0
            i32.load offset=1050644 align=1
            i32.store align=1
            local.get 0
            i32.const 8
            i32.add
            i32.const 0
            i64.load offset=1050636 align=1
            i64.store align=1
            local.get 0
            i32.const 0
            i64.load offset=1050628 align=1
            i64.store align=1
            local.get 2
            i64.const 85899345940
            i64.store offset=12 align=4
            local.get 2
            local.get 0
            i32.store offset=8
            local.get 2
            i32.const 40
            i32.add
            i32.const 20
            i32.add
            i32.const 2
            i32.store
            local.get 2
            i32.const 36
            i32.add
            i32.const 7
            i32.store
            local.get 2
            i64.const 3
            i64.store offset=44 align=4
            local.get 2
            i32.const 1049800
            i32.store offset=40
            local.get 2
            i32.const 8
            i32.store offset=28
            local.get 2
            local.get 2
            i32.const 24
            i32.add
            i32.store offset=56
            local.get 2
            local.get 2
            i32.const 4
            i32.add
            i32.store offset=32
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=24
            local.get 1
            local.get 2
            i32.const 40
            i32.add
            call $core::fmt::Formatter::write_fmt::hb3bb9e03c3e75964
            local.set 0
            local.get 2
            i32.load offset=12
            local.tee 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.load offset=8
            local.get 1
            i32.const 1
            call $__rust_dealloc
            br 2 (;@2;)
          end
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 0
          i32.load
          local.get 1
          local.get 0
          i32.load offset=4
          i32.load offset=32
          call_indirect (type 2)
          local.set 0
          br 1 (;@2;)
        end
        i32.const 1049480
        local.set 3
        i32.const 22
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              local.get 0
                                              i32.load8_u offset=1
                                              br_table 0 (;@21;) 1 (;@20;) 2 (;@19;) 3 (;@18;) 4 (;@17;) 5 (;@16;) 6 (;@15;) 7 (;@14;) 8 (;@13;) 9 (;@12;) 10 (;@11;) 11 (;@10;) 12 (;@9;) 13 (;@8;) 14 (;@7;) 15 (;@6;) 16 (;@5;) 18 (;@3;) 0 (;@21;)
                                            end
                                            i32.const 1049761
                                            local.set 3
                                            i32.const 16
                                            local.set 4
                                            br 17 (;@3;)
                                          end
                                          i32.const 1049744
                                          local.set 3
                                          i32.const 17
                                          local.set 4
                                          br 16 (;@3;)
                                        end
                                        i32.const 1049726
                                        local.set 3
                                        i32.const 18
                                        local.set 4
                                        br 15 (;@3;)
                                      end
                                      i32.const 1049710
                                      local.set 3
                                      i32.const 16
                                      local.set 4
                                      br 14 (;@3;)
                                    end
                                    i32.const 1049692
                                    local.set 3
                                    i32.const 18
                                    local.set 4
                                    br 13 (;@3;)
                                  end
                                  i32.const 1049679
                                  local.set 3
                                  i32.const 13
                                  local.set 4
                                  br 12 (;@3;)
                                end
                                i32.const 1049665
                                local.set 3
                                br 10 (;@4;)
                              end
                              i32.const 1049644
                              local.set 3
                              i32.const 21
                              local.set 4
                              br 10 (;@3;)
                            end
                            i32.const 1049633
                            local.set 3
                            i32.const 11
                            local.set 4
                            br 9 (;@3;)
                          end
                          i32.const 1049612
                          local.set 3
                          i32.const 21
                          local.set 4
                          br 8 (;@3;)
                        end
                        i32.const 1049591
                        local.set 3
                        i32.const 21
                        local.set 4
                        br 7 (;@3;)
                      end
                      i32.const 1049568
                      local.set 3
                      i32.const 23
                      local.set 4
                      br 6 (;@3;)
                    end
                    i32.const 1049556
                    local.set 3
                    i32.const 12
                    local.set 4
                    br 5 (;@3;)
                  end
                  i32.const 1049547
                  local.set 3
                  i32.const 9
                  local.set 4
                  br 4 (;@3;)
                end
                i32.const 1049537
                local.set 3
                i32.const 10
                local.set 4
                br 3 (;@3;)
              end
              i32.const 1049516
              local.set 3
              i32.const 21
              local.set 4
              br 2 (;@3;)
            end
            i32.const 1049502
            local.set 3
          end
          i32.const 14
          local.set 4
        end
        local.get 2
        i32.const 60
        i32.add
        i32.const 1
        i32.store
        local.get 2
        local.get 4
        i32.store offset=28
        local.get 2
        local.get 3
        i32.store offset=24
        local.get 2
        i32.const 9
        i32.store offset=12
        local.get 2
        i64.const 1
        i64.store offset=44 align=4
        local.get 2
        i32.const 1049780
        i32.store offset=40
        local.get 2
        local.get 2
        i32.const 24
        i32.add
        i32.store offset=8
        local.get 2
        local.get 2
        i32.const 8
        i32.add
        i32.store offset=56
        local.get 1
        local.get 2
        i32.const 40
        i32.add
        call $core::fmt::Formatter::write_fmt::hb3bb9e03c3e75964
        local.set 0
      end
      local.get 2
      i32.const 64
      i32.add
      global.set 0
      local.get 0
      return
    end
    i32.const 20
    i32.const 1
    call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
    unreachable)
  (func $std::error::Error::cause::h0a468326a46f05ba (type 4) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store)
  (func $std::error::Error::type_id::h3398eed2c8189c49 (type 10) (param i32) (result i64)
    i64.const -3605044646072173466)
  (func $std::error::Error::backtrace::h4da4cddfeacc4aca (type 5) (param i32) (result i32)
    i32.const 0)
  (func $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_std::error::Error>::description::h9ca06f358a7fd8af (type 4) (param i32 i32)
    local.get 0
    local.get 1
    i32.load offset=8
    i32.store offset=4
    local.get 0
    local.get 1
    i32.load
    i32.store)
  (func $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_core::fmt::Display>::fmt::hb37607c61a21babc (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.get 1
    call $<str_as_core::fmt::Display>::fmt::h38ffb460df0bf4b9)
  (func $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_core::fmt::Debug>::fmt::h5fd9380e7b2f9c51 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.get 1
    call $<str_as_core::fmt::Debug>::fmt::h195f820ca2dc4e68)
  (func $<std::io::buffered::LineWriter<W>_as_std::io::Write>::write::h447b0ce154db372f (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.load8_u offset=16
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load8_u offset=12
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.load offset=8
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 255
            i32.and
            i32.const 2
            i32.eq
            br_if 1 (;@3;)
            local.get 1
            i32.const 0
            i32.store offset=8
            local.get 1
            i32.const 0
            i32.store8 offset=13
            br 2 (;@2;)
          end
          local.get 5
          i32.const 255
          i32.and
          i32.const 2
          i32.ne
          br_if 1 (;@2;)
          i32.const 1048920
          call $core::panicking::panic::h0142ee7f4c64bd08
          unreachable
        end
        local.get 1
        i32.const 1
        i32.store8 offset=13
        i32.const 1048920
        call $core::panicking::panic::h0142ee7f4c64bd08
        unreachable
      end
      local.get 1
      i32.const 0
      i32.store8 offset=16
    end
    local.get 4
    i32.const 8
    i32.add
    i32.const 10
    local.get 2
    local.get 3
    call $core::slice::memchr::memrchr::hd17908245138f97b
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=8
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 1
                        i32.load offset=8
                        local.tee 6
                        local.get 3
                        i32.add
                        local.get 1
                        i32.load offset=4
                        local.tee 5
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 6
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        i32.load8_u offset=12
                        i32.const 2
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 1
                        i32.const 0
                        i32.store offset=8
                        local.get 1
                        i32.const 0
                        i32.store8 offset=13
                      end
                      block  ;; label = @10
                        local.get 5
                        local.get 3
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 3
                        call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
                        local.get 1
                        local.get 1
                        i32.load offset=8
                        local.tee 5
                        local.get 3
                        i32.add
                        i32.store offset=8
                        local.get 5
                        local.get 1
                        i32.load
                        i32.add
                        local.get 2
                        local.get 3
                        call $memcpy
                        drop
                        local.get 0
                        i32.const 0
                        i32.store
                        br 2 (;@8;)
                      end
                      local.get 1
                      i32.const 1
                      i32.store8 offset=13
                      local.get 1
                      i32.load8_u offset=12
                      i32.const 2
                      i32.eq
                      br_if 4 (;@5;)
                      local.get 0
                      local.get 3
                      i32.store offset=4
                      local.get 0
                      i32.const 0
                      i32.store
                      local.get 1
                      i32.const 0
                      i32.store8 offset=13
                      br 2 (;@7;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 6
                        i32.const -1
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 6
                        i32.const 1
                        i32.add
                        local.set 5
                        local.get 6
                        local.get 3
                        i32.lt_u
                        br_if 1 (;@9;)
                        local.get 5
                        local.get 3
                        call $core::slice::slice_index_len_fail::h08f636efd7156c0a
                        unreachable
                      end
                      call $core::slice::slice_index_overflow_fail::h1d032eb7c15bac81
                      unreachable
                    end
                    block  ;; label = @9
                      local.get 1
                      i32.load offset=8
                      local.tee 6
                      local.get 5
                      i32.add
                      local.get 1
                      i32.load offset=4
                      local.tee 7
                      i32.le_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 6
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 6
                        br 1 (;@9;)
                      end
                      local.get 1
                      i32.load8_u offset=12
                      i32.const 2
                      i32.eq
                      br_if 5 (;@4;)
                      i32.const 0
                      local.set 6
                      local.get 1
                      i32.const 0
                      i32.store offset=8
                      local.get 1
                      i32.const 0
                      i32.store8 offset=13
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 7
                        local.get 5
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 5
                        call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
                        local.get 1
                        local.get 1
                        i32.load offset=8
                        local.tee 6
                        local.get 5
                        i32.add
                        i32.store offset=8
                        local.get 6
                        local.get 1
                        i32.load
                        i32.add
                        local.get 2
                        local.get 5
                        call $memcpy
                        drop
                        local.get 1
                        i32.load8_u offset=12
                        local.set 7
                        local.get 1
                        i32.load offset=8
                        local.set 6
                        br 1 (;@9;)
                      end
                      local.get 1
                      i32.const 1
                      i32.store8 offset=13
                      local.get 1
                      i32.load8_u offset=12
                      local.tee 7
                      i32.const 2
                      i32.eq
                      br_if 6 (;@3;)
                      local.get 1
                      i32.const 0
                      i32.store8 offset=13
                    end
                    local.get 1
                    i32.const 1
                    i32.store8 offset=16
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 6
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        i32.const 255
                        i32.and
                        i32.const 2
                        i32.eq
                        br_if 8 (;@2;)
                        local.get 1
                        i32.const 0
                        i32.store offset=8
                        local.get 1
                        i32.const 0
                        i32.store8 offset=13
                        br 1 (;@9;)
                      end
                      local.get 7
                      i32.const 255
                      i32.and
                      i32.const 2
                      i32.eq
                      br_if 8 (;@1;)
                    end
                    local.get 1
                    i32.const 0
                    i32.store8 offset=16
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.load offset=4
                        local.get 3
                        local.get 5
                        i32.sub
                        local.tee 6
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 6
                        call $alloc::vec::Vec<T>::reserve::h7fa9d0b59b44b5e4
                        local.get 1
                        local.get 1
                        i32.load offset=8
                        local.tee 7
                        local.get 6
                        i32.add
                        i32.store offset=8
                        local.get 7
                        local.get 1
                        i32.load
                        i32.add
                        local.get 2
                        local.get 5
                        i32.add
                        local.get 6
                        call $memcpy
                        drop
                        br 1 (;@9;)
                      end
                      local.get 1
                      i32.const 0
                      i32.store8 offset=13
                    end
                    local.get 0
                    i32.const 0
                    i32.store
                  end
                  local.get 0
                  local.get 3
                  i32.store offset=4
                end
                local.get 4
                i32.const 16
                i32.add
                global.set 0
                return
              end
              local.get 1
              i32.const 1
              i32.store8 offset=13
              i32.const 1048920
              call $core::panicking::panic::h0142ee7f4c64bd08
              unreachable
            end
            i32.const 1048920
            call $core::panicking::panic::h0142ee7f4c64bd08
            unreachable
          end
          local.get 1
          i32.const 1
          i32.store8 offset=13
          i32.const 1048920
          call $core::panicking::panic::h0142ee7f4c64bd08
          unreachable
        end
        i32.const 1048920
        call $core::panicking::panic::h0142ee7f4c64bd08
        unreachable
      end
      local.get 1
      i32.const 1
      i32.store8 offset=13
      i32.const 1048920
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    i32.const 1048920
    call $core::panicking::panic::h0142ee7f4c64bd08
    unreachable)
  (func $std::sys_common::at_exit_imp::push::hef2069aaba8ace41 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1057145
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1057145
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1056632
                  local.tee 2
                  i32.const 1
                  i32.gt_u
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 2
                    br_table 0 (;@8;) 2 (;@6;) 0 (;@8;)
                  end
                  i32.const 12
                  i32.const 4
                  call $__rust_alloc
                  local.tee 2
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 2
                  i32.const 0
                  i32.store offset=8
                  local.get 2
                  i64.const 4
                  i64.store align=4
                  i32.const 0
                  local.get 2
                  i32.store offset=1056632
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load offset=8
                    local.tee 3
                    local.get 2
                    i32.load offset=4
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 2
                    i32.load
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 3
                  i32.const 1
                  i32.add
                  local.tee 4
                  local.get 3
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 3
                  i32.const 1
                  i32.shl
                  local.tee 5
                  local.get 4
                  local.get 5
                  local.get 4
                  i32.gt_u
                  select
                  local.tee 5
                  i32.const 536870911
                  i32.and
                  local.tee 4
                  local.get 5
                  i32.ne
                  br_if 5 (;@2;)
                  local.get 5
                  i32.const 3
                  i32.shl
                  local.tee 6
                  i32.const 0
                  i32.lt_s
                  br_if 5 (;@2;)
                  local.get 4
                  local.get 5
                  i32.eq
                  i32.const 2
                  i32.shl
                  local.set 7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      br_if 0 (;@9;)
                      local.get 6
                      local.get 7
                      call $__rust_alloc
                      local.set 4
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.load
                    local.get 3
                    i32.const 3
                    i32.shl
                    i32.const 4
                    local.get 6
                    call $__rust_realloc
                    local.set 4
                  end
                  local.get 4
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 2
                  local.get 5
                  i32.store offset=4
                  local.get 2
                  local.get 4
                  i32.store
                  local.get 2
                  i32.load offset=8
                  local.set 3
                end
                local.get 4
                local.get 3
                i32.const 3
                i32.shl
                i32.add
                local.tee 3
                local.get 1
                i32.store offset=4
                local.get 3
                local.get 0
                i32.store
                i32.const 1
                local.set 3
                local.get 2
                local.get 2
                i32.load offset=8
                i32.const 1
                i32.add
                i32.store offset=8
                i32.const 0
                i32.const 0
                i32.store8 offset=1057145
                br 5 (;@1;)
              end
              i32.const 0
              local.set 3
              i32.const 0
              i32.const 0
              i32.store8 offset=1057145
              local.get 0
              local.get 1
              i32.load
              call_indirect (type 1)
              local.get 1
              i32.load offset=4
              local.tee 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 2
              local.get 1
              i32.load offset=8
              call $__rust_dealloc
              i32.const 0
              return
            end
            i32.const 1050772
            i32.const 32
            i32.const 1050756
            call $std::panicking::begin_panic::h837c5ed1e256db31
            unreachable
          end
          i32.const 12
          i32.const 4
          call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
          unreachable
        end
        local.get 6
        local.get 7
        call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
        unreachable
      end
      call $alloc::raw_vec::capacity_overflow::hc538c246d520d486
      unreachable
    end
    local.get 3)
  (func $std::io::stdio::stdout::h3e462d5f91515f07 (type 8) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load8_u offset=1056628
                br_if 0 (;@6;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1056628
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1056624
                    local.tee 1
                    i32.const 1
                    i32.gt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        br_table 1 (;@9;) 0 (;@10;) 1 (;@9;)
                      end
                      i32.const 0
                      i32.const 0
                      i32.store8 offset=1056628
                      i32.const 1049824
                      i32.const 36
                      call $core::option::expect_failed::h38f732006dee06b2
                      unreachable
                    end
                    i32.const 4
                    i32.const 4
                    call $__rust_alloc
                    local.tee 1
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 1
                    i32.const 1056624
                    i32.store
                    local.get 1
                    i32.const 1050500
                    call $std::sys_common::at_exit_imp::push::hef2069aaba8ace41
                    local.set 2
                    i32.const 1024
                    i32.const 1
                    call $__rust_alloc
                    local.tee 3
                    i32.eqz
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 10
                    i32.add
                    i32.const 2
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 13
                    i32.add
                    i32.const 2
                    i32.add
                    i32.load8_u
                    i32.store8
                    local.get 0
                    local.get 0
                    i32.load16_u offset=13 align=1
                    i32.store16 offset=10
                    i32.const 40
                    i32.const 4
                    call $__rust_alloc
                    local.tee 1
                    i32.eqz
                    br_if 5 (;@3;)
                    local.get 1
                    i32.const 0
                    i32.store8 offset=32
                    local.get 1
                    i32.const 0
                    i32.store16 offset=28
                    local.get 1
                    i64.const 1024
                    i64.store offset=20 align=4
                    local.get 1
                    local.get 3
                    i32.store offset=16
                    local.get 1
                    i64.const 1
                    i64.store offset=8 align=4
                    local.get 1
                    i64.const 4294967297
                    i64.store align=4
                    local.get 1
                    local.get 0
                    i32.load16_u offset=10
                    i32.store16 offset=33 align=1
                    local.get 1
                    i32.const 0
                    i32.store8 offset=36
                    local.get 1
                    local.get 0
                    i32.load16_u offset=7 align=1
                    i32.store16 offset=37 align=1
                    local.get 1
                    i32.const 35
                    i32.add
                    local.get 4
                    i32.load8_u
                    i32.store8
                    local.get 1
                    i32.const 39
                    i32.add
                    local.get 0
                    i32.const 7
                    i32.add
                    i32.const 2
                    i32.add
                    i32.load8_u
                    i32.store8
                    local.get 2
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 1
                    local.get 1
                    i32.load
                    local.tee 2
                    i32.const 1
                    i32.add
                    i32.store
                    local.get 2
                    i32.const -1
                    i32.le_s
                    br_if 6 (;@2;)
                    i32.const 4
                    i32.const 4
                    call $__rust_alloc
                    local.tee 2
                    i32.eqz
                    br_if 7 (;@1;)
                    i32.const 0
                    local.get 2
                    i32.store offset=1056624
                    local.get 2
                    local.get 1
                    i32.store
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.load
                  local.tee 1
                  local.get 1
                  i32.load
                  local.tee 2
                  i32.const 1
                  i32.add
                  i32.store
                  local.get 2
                  i32.const -1
                  i32.le_s
                  br_if 5 (;@2;)
                end
                i32.const 0
                i32.const 0
                i32.store8 offset=1056628
                local.get 0
                i32.const 16
                i32.add
                global.set 0
                local.get 1
                return
              end
              i32.const 1050772
              i32.const 32
              i32.const 1050756
              call $std::panicking::begin_panic::h837c5ed1e256db31
              unreachable
            end
            i32.const 4
            i32.const 4
            call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
            unreachable
          end
          i32.const 1024
          i32.const 1
          call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
          unreachable
        end
        i32.const 40
        i32.const 4
        call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
        unreachable
      end
      unreachable
      unreachable
    end
    i32.const 4
    i32.const 4
    call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
    unreachable)
  (func $<std::io::stdio::Stdout_as_std::io::Write>::write_fmt::ha3ef20bd359ff2ca (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 0
    local.set 4
    local.get 1
    i32.load
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1057136
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1057140
        local.set 4
        br 1 (;@1;)
      end
      i32.const 0
      i64.const 1
      i64.store offset=1057136
    end
    i32.const 0
    local.get 4
    i32.store offset=1057140
    local.get 3
    local.get 4
    i32.const 0
    i32.ne
    i32.store8 offset=4
    local.get 3
    local.get 1
    i32.const 8
    i32.add
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=12
    local.get 3
    local.get 3
    i32.store offset=8
    local.get 3
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    local.get 2
    i64.load align=4
    i64.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 8
              i32.add
              i32.const 1049976
              local.get 3
              i32.const 24
              i32.add
              call $core::fmt::write::hb137f2496e0ed1b6
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 3
                i32.load8_u offset=12
                i32.const 3
                i32.ne
                br_if 0 (;@6;)
                i32.const 15
                i32.const 1
                call $__rust_alloc
                local.tee 2
                i32.eqz
                br_if 3 (;@3;)
                local.get 2
                i32.const 7
                i32.add
                i32.const 0
                i64.load offset=1050007 align=1
                i64.store align=1
                local.get 2
                i32.const 0
                i64.load offset=1050000 align=1
                i64.store align=1
                i32.const 12
                i32.const 4
                call $__rust_alloc
                local.tee 4
                i32.eqz
                br_if 4 (;@2;)
                local.get 4
                i64.const 64424509455
                i64.store offset=4 align=4
                local.get 4
                local.get 2
                i32.store
                i32.const 12
                i32.const 4
                call $__rust_alloc
                local.tee 2
                i32.eqz
                br_if 5 (;@1;)
                local.get 2
                i32.const 16
                i32.store8 offset=8
                local.get 2
                i32.const 1049440
                i32.store offset=4
                local.get 2
                local.get 4
                i32.store
                local.get 2
                local.get 3
                i32.load16_u offset=24 align=1
                i32.store16 offset=9 align=1
                local.get 2
                i32.const 11
                i32.add
                local.get 3
                i32.const 24
                i32.add
                i32.const 2
                i32.add
                i32.load8_u
                i32.store8
                local.get 0
                i32.const 4
                i32.add
                local.get 2
                i32.store
                local.get 0
                i32.const 2
                i32.store
                br 2 (;@4;)
              end
              local.get 0
              local.get 3
              i64.load offset=12 align=4
              i64.store align=4
              br 1 (;@4;)
            end
            local.get 0
            i32.const 3
            i32.store8
            block  ;; label = @5
              i32.const 0
              br_if 0 (;@5;)
              local.get 3
              i32.load8_u offset=12
              i32.const 2
              i32.ne
              br_if 1 (;@4;)
            end
            local.get 3
            i32.const 16
            i32.add
            i32.load
            local.tee 2
            i32.load
            local.get 2
            i32.load offset=4
            i32.load
            call_indirect (type 1)
            block  ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 4
              i32.load offset=4
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.load
              local.get 0
              local.get 4
              i32.load offset=8
              call $__rust_dealloc
            end
            local.get 3
            i32.load offset=16
            i32.const 12
            i32.const 4
            call $__rust_dealloc
          end
          block  ;; label = @4
            local.get 3
            i32.load8_u offset=4
            br_if 0 (;@4;)
            block  ;; label = @5
              i32.const 0
              i32.load offset=1057136
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              i64.const 1
              i64.store offset=1057136
              br 1 (;@4;)
            end
            i32.const 0
            i32.load offset=1057140
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.load
            i32.const 1
            i32.store8 offset=28
          end
          local.get 3
          i32.const 48
          i32.add
          global.set 0
          return
        end
        i32.const 15
        i32.const 1
        call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
        unreachable
      end
      i32.const 12
      i32.const 4
      call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
      unreachable
    end
    i32.const 12
    i32.const 4
    call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
    unreachable)
  (func $std::io::stdio::_print::hc753675f57d09bbc (type 1) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 1
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 1
    local.get 0
    i64.load align=4
    i64.store offset=8
    local.get 1
    i32.const 6
    i32.store offset=36
    local.get 1
    i32.const 1049940
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1056656
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i64.const -4294967295
            i64.store offset=1056656 align=4
            i32.const 0
            i32.const 0
            i32.store offset=1056664
            local.get 1
            i32.const 56
            i32.add
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          i32.const 56
          i32.add
          local.set 2
          i32.const 0
          i32.load offset=1056660
          br_if 1 (;@2;)
          i32.const 0
          i32.const -1
          i32.store offset=1056660
          local.get 1
          i32.const 56
          i32.add
          local.set 2
          i32.const 0
          i32.load offset=1056664
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1056668
          local.set 3
          local.get 1
          i32.const 72
          i32.add
          i32.const 16
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 1
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 1
          local.get 1
          i64.load offset=8
          i64.store offset=72
          local.get 1
          i32.const 56
          i32.add
          local.get 0
          local.get 1
          i32.const 72
          i32.add
          local.get 3
          i32.load offset=28
          call_indirect (type 3)
          i32.const 0
          i32.const 0
          i32.load offset=1056660
          i32.const 1
          i32.add
          i32.store offset=1056660
          br 2 (;@1;)
        end
        i32.const 0
        i32.const 0
        i32.store offset=1056660
      end
      local.get 1
      call $std::io::stdio::stdout::h3e462d5f91515f07
      local.tee 0
      i32.store offset=48
      local.get 1
      i32.const 72
      i32.add
      i32.const 16
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i32.const 16
      i32.add
      i64.load
      i64.store
      local.get 1
      i32.const 72
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      i64.load
      i64.store
      local.get 1
      local.get 1
      i64.load offset=8
      i64.store offset=72
      local.get 2
      local.get 1
      i32.const 48
      i32.add
      local.get 1
      i32.const 72
      i32.add
      call $<std::io::stdio::Stdout_as_std::io::Write>::write_fmt::ha3ef20bd359ff2ca
      local.get 0
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.add
      i32.store
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 48
        i32.add
        call $alloc::sync::Arc<T>::drop_slow::hf5b61bcc786904c0
      end
      local.get 1
      i32.const 56
      i32.add
      local.set 2
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=56
        local.tee 0
        i32.const 255
        i32.and
        i32.const 4
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.load offset=4
        i32.store offset=44
        local.get 1
        local.get 0
        i32.store offset=40
        br 1 (;@1;)
      end
      local.get 1
      call $std::io::stdio::stdout::h3e462d5f91515f07
      local.tee 0
      i32.store offset=56
      local.get 1
      i32.const 72
      i32.add
      i32.const 16
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i32.const 16
      i32.add
      i64.load
      i64.store
      local.get 1
      i32.const 72
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      i64.load
      i64.store
      local.get 1
      local.get 1
      i64.load offset=8
      i64.store offset=72
      local.get 1
      i32.const 40
      i32.add
      local.get 1
      i32.const 56
      i32.add
      local.get 1
      i32.const 72
      i32.add
      call $<std::io::stdio::Stdout_as_std::io::Write>::write_fmt::ha3ef20bd359ff2ca
      local.get 0
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.add
      i32.store
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 56
        i32.add
        call $alloc::sync::Arc<T>::drop_slow::hf5b61bcc786904c0
      end
      local.get 1
      i32.load8_u offset=40
      local.set 0
    end
    block  ;; label = @1
      local.get 0
      i32.const 255
      i32.and
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          br_if 0 (;@3;)
          local.get 0
          i32.const 3
          i32.and
          i32.const 2
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 1
        i32.load offset=44
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.get 3
          local.get 2
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 1
      i32.const 96
      i32.add
      global.set 0
      return
    end
    local.get 1
    local.get 1
    i64.load offset=40
    i64.store offset=48
    local.get 1
    i32.const 92
    i32.add
    i32.const 2
    i32.store
    local.get 1
    i32.const 68
    i32.add
    i32.const 10
    i32.store
    local.get 1
    i64.const 2
    i64.store offset=76 align=4
    local.get 1
    i32.const 1049884
    i32.store offset=72
    local.get 1
    i32.const 9
    i32.store offset=60
    local.get 1
    local.get 1
    i32.const 56
    i32.add
    i32.store offset=88
    local.get 1
    local.get 1
    i32.const 48
    i32.add
    i32.store offset=64
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    i32.store offset=56
    local.get 1
    i32.const 72
    i32.add
    i32.const 1049924
    call $std::panicking::begin_panic_fmt::h62d8474306d91923
    unreachable)
  (func $<std::io::Write::write_fmt::Adaptor<T>_as_core::fmt::Write>::write_str::h190ca40cd38a8fb4 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i64 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $std::io::Write::write_all::h24f1b9deb3916b44
    i32.const 0
    local.set 1
    block  ;; label = @1
      local.get 3
      i32.load8_u offset=8
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      i64.load offset=8
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=4
          i32.const 2
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 1
        i32.load
        local.get 1
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 2
          i32.load offset=4
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.get 5
          local.get 2
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.load offset=8
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      local.get 4
      i64.store offset=4 align=4
      i32.const 1
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $std::sync::once::Once::call_inner::h65db2b3b821c8bf1 (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 2
    i32.or
    local.set 5
    local.get 0
    i32.load
    local.set 6
    loop  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.tee 7
              i32.const 3
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 7
                    br_table 1 (;@7;) 0 (;@8;) 3 (;@5;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 1
                  i32.eqz
                  br_if 3 (;@4;)
                end
                local.get 0
                i32.const 2
                local.get 0
                i32.load
                local.tee 6
                local.get 6
                local.get 7
                i32.eq
                select
                i32.store
                local.get 6
                local.get 7
                i32.ne
                br_if 5 (;@1;)
                local.get 4
                local.get 0
                i32.store
                local.get 2
                local.get 7
                i32.const 1
                i32.eq
                local.get 3
                i32.load offset=12
                call_indirect (type 4)
                local.get 4
                i32.const 0
                i32.store8 offset=4
                local.get 4
                call $<std::sync::once::Finish_as_core::ops::drop::Drop>::drop::h0d9642e4a07bbdfb
              end
              local.get 4
              i32.const 16
              i32.add
              global.set 0
              return
            end
            local.get 7
            i32.const 3
            i32.and
            i32.const 2
            i32.ne
            br_if 1 (;@3;)
            block  ;; label = @5
              i32.const 0
              i32.load offset=1056672
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              i64.const 1
              i64.store offset=1056672 align=4
              i32.const 0
              i32.const 0
              i32.store offset=1056680
            end
            i32.const 1056676
            call $std::sys_common::thread_info::ThreadInfo::with::__closure__::h34502497b740844d
            local.set 6
            local.get 4
            i32.const 0
            i32.store8 offset=8
            local.get 4
            local.get 6
            i32.store
            local.get 4
            i32.const 0
            i32.store offset=4
            loop  ;; label = @5
              block  ;; label = @6
                local.get 7
                i32.const 3
                i32.and
                i32.const 2
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 4
                  i32.load
                  local.tee 6
                  br_if 0 (;@7;)
                  local.get 7
                  local.set 6
                  br 6 (;@1;)
                end
                local.get 6
                local.get 6
                i32.load
                local.tee 8
                i32.const -1
                i32.add
                i32.store
                block  ;; label = @7
                  local.get 8
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 7
                  local.set 6
                  br 6 (;@1;)
                end
                local.get 4
                call $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab
                local.get 7
                local.set 6
                br 5 (;@1;)
              end
              local.get 0
              local.get 5
              local.get 0
              i32.load
              local.tee 6
              local.get 6
              local.get 7
              i32.eq
              select
              i32.store
              local.get 4
              local.get 7
              i32.const -4
              i32.and
              i32.store offset=4
              local.get 6
              local.get 7
              i32.ne
              local.set 8
              local.get 6
              local.set 7
              local.get 8
              br_if 0 (;@5;)
            end
            local.get 4
            i32.load8_u offset=8
            br_if 2 (;@2;)
            loop  ;; label = @5
              call $std::thread::park::h0838d30e3c3b47de
              local.get 4
              i32.load8_u offset=8
              i32.eqz
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          i32.const 1050240
          i32.const 42
          i32.const 1050224
          call $std::panicking::begin_panic::h837c5ed1e256db31
          unreachable
        end
        i32.const 1050176
        i32.const 47
        i32.const 1050160
        call $std::panicking::begin_panic::h837c5ed1e256db31
        unreachable
      end
      local.get 0
      i32.load
      local.set 6
      local.get 4
      i32.load
      local.tee 7
      i32.eqz
      br_if 0 (;@1;)
      local.get 7
      local.get 7
      i32.load
      local.tee 8
      i32.const -1
      i32.add
      i32.store
      local.get 8
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 4
      call $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab
      br 0 (;@1;)
    end)
  (func $<std::sync::once::Finish_as_core::ops::drop::Drop>::drop::h0d9642e4a07bbdfb (type 1) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load
    local.tee 2
    i32.load
    local.set 3
    local.get 2
    i32.const 1
    i32.const 3
    local.get 0
    i32.load8_u offset=4
    select
    i32.store
    local.get 1
    local.get 3
    i32.const 3
    i32.and
    local.tee 0
    i32.store offset=12
    block  ;; label = @1
      local.get 0
      i32.const 2
      i32.ne
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const -4
          i32.and
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 0
            i32.load offset=4
            local.set 3
            local.get 0
            i32.load
            local.set 2
            local.get 0
            i32.const 0
            i32.store
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.const 1
            i32.store8 offset=8
            local.get 1
            local.get 2
            i32.store offset=16
            local.get 1
            i32.const 16
            i32.add
            call $std::thread::Thread::unpark::h51f257038f7af062
            local.get 1
            i32.load offset=16
            local.tee 0
            local.get 0
            i32.load
            local.tee 0
            i32.const -1
            i32.add
            i32.store
            block  ;; label = @5
              local.get 0
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 16
              i32.add
              call $alloc::sync::Arc<T>::drop_slow::h61d24e70e2bd1cab
            end
            local.get 3
            local.set 0
            local.get 3
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 64
        i32.add
        global.set 0
        return
      end
      i32.const 1048920
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    local.get 1
    i32.const 52
    i32.add
    i32.const 6
    i32.store
    local.get 1
    i32.const 36
    i32.add
    i32.const 2
    i32.store
    local.get 1
    i64.const 3
    i64.store offset=20 align=4
    local.get 1
    i32.const 1048788
    i32.store offset=16
    local.get 1
    i32.const 6
    i32.store offset=44
    local.get 1
    local.get 1
    i32.const 12
    i32.add
    i32.store offset=56
    local.get 1
    i32.const 1049216
    i32.store offset=60
    local.get 1
    local.get 1
    i32.const 40
    i32.add
    i32.store offset=32
    local.get 1
    local.get 1
    i32.const 60
    i32.add
    i32.store offset=48
    local.get 1
    local.get 1
    i32.const 56
    i32.add
    i32.store offset=40
    local.get 1
    i32.const 16
    i32.add
    i32.const 1050284
    call $std::panicking::begin_panic_fmt::h62d8474306d91923
    unreachable)
  (func $std::sys_common::backtrace::__rust_begin_short_backtrace::h07df8a1ef7fa7e6e (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.load offset=12
    call_indirect (type 5))
  (func $std::sys::wasm::condvar::Condvar::wait::hda825f3f4310d4db (type 4) (param i32 i32)
    i32.const 1050696
    i32.const 29
    i32.const 1050680
    call $std::panicking::begin_panic::h837c5ed1e256db31
    unreachable)
  (func $<std::sys_common::poison::PoisonError<T>_as_core::fmt::Debug>::fmt::h2aeb25b601b8e9c6 (type 2) (param i32 i32) (result i32)
    i32.const 1050383
    i32.const 25
    local.get 1
    call $<str_as_core::fmt::Debug>::fmt::h195f820ca2dc4e68)
  (func $<std::sys_common::thread_local::Key_as_core::ops::drop::Drop>::drop::ha98c40f1657718ec (type 1) (param i32))
  (func $std::alloc::default_alloc_error_hook::h4c4aa82eea9626e8 (type 4) (param i32 i32))
  (func $rust_oom (type 4) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=1056640
    local.tee 2
    i32.const 11
    local.get 2
    select
    call_indirect (type 4)
    unreachable
    unreachable)
  (func $__rdl_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      i32.const 1056684
      call $dlmalloc::dlmalloc::Dlmalloc::malloc_alignment::h51e2543d99a8534c
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 1056684
      local.get 1
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc::memalign::h222fd434a08907f4
      return
    end
    i32.const 1056684
    local.get 0
    call $dlmalloc::dlmalloc::Dlmalloc::malloc::h469ed7441b6b922b)
  (func $__rdl_dealloc (type 3) (param i32 i32 i32)
    i32.const 1056684
    local.get 0
    call $dlmalloc::dlmalloc::Dlmalloc::free::h81e375dc80848071)
  (func $__rdl_realloc (type 9) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1056684
        call $dlmalloc::dlmalloc::Dlmalloc::malloc_alignment::h51e2543d99a8534c
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            i32.const 1056684
            call $dlmalloc::dlmalloc::Dlmalloc::malloc_alignment::h51e2543d99a8534c
            local.get 2
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 1056684
            local.get 2
            local.get 3
            call $dlmalloc::dlmalloc::Dlmalloc::memalign::h222fd434a08907f4
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1056684
          local.get 3
          call $dlmalloc::dlmalloc::Dlmalloc::malloc::h469ed7441b6b922b
          local.set 2
        end
        local.get 2
        br_if 1 (;@1;)
        i32.const 0
        return
      end
      i32.const 1056684
      local.get 0
      local.get 3
      call $dlmalloc::dlmalloc::Dlmalloc::realloc::h4166d0a35cb7b3c3
      return
    end
    local.get 2
    local.get 0
    local.get 3
    local.get 1
    local.get 1
    local.get 3
    i32.gt_u
    select
    call $memcpy
    local.set 2
    i32.const 1056684
    local.get 0
    call $dlmalloc::dlmalloc::Dlmalloc::free::h81e375dc80848071
    local.get 2)
  (func $std::panicking::try::do_call::h7511ee0f6f4a8e2a (type 1) (param i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load
    local.tee 1
    i32.load
    local.get 1
    i32.load offset=4
    call $std::sys_common::backtrace::__rust_begin_short_backtrace::h07df8a1ef7fa7e6e
    i32.store)
  (func $rust_begin_unwind (type 1) (param i32)
    local.get 0
    call $std::panicking::continue_panic_fmt::hb5b3e4b5160fe2ab
    unreachable)
  (func $std::panicking::continue_panic_fmt::hb5b3e4b5160fe2ab (type 1) (param i32)
    (local i32 i32 i32 i64 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    call $core::panic::PanicInfo::location::hbc5e44a64eaf706a
    call $core::option::Option<T>::unwrap::h684599df4939e5f6
    local.set 2
    local.get 0
    call $core::panic::PanicInfo::message::hc730610bb8056e74
    call $core::option::Option<T>::unwrap::hc5bf9494982dd003
    local.set 3
    local.get 1
    i32.const 8
    i32.add
    local.get 2
    call $core::panic::Location::file::hfbb9014eea889c61
    local.get 1
    i64.load offset=8
    local.set 4
    local.get 2
    call $core::panic::Location::line::h75a85319172d348e
    local.set 5
    local.get 1
    local.get 2
    call $core::panic::Location::column::h4bc83a66cb1b6958
    i32.store offset=28
    local.get 1
    local.get 5
    i32.store offset=24
    local.get 1
    local.get 4
    i64.store offset=16
    local.get 1
    i32.const 0
    i32.store offset=36
    local.get 1
    local.get 3
    i32.store offset=32
    local.get 1
    i32.const 32
    i32.add
    i32.const 1050516
    local.get 0
    call $core::panic::PanicInfo::message::hc730610bb8056e74
    local.get 1
    i32.const 16
    i32.add
    call $std::panicking::rust_panic_with_hook::h5e7c2dc110ae79d4
    unreachable)
  (func $std::panicking::rust_panic_with_hook::h5e7c2dc110ae79d4 (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 4
    global.set 0
    i32.const 1
    local.set 5
    local.get 3
    i32.load offset=12
    local.set 6
    local.get 3
    i32.load offset=8
    local.set 7
    local.get 3
    i32.load offset=4
    local.set 8
    local.get 3
    i32.load
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1057136
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i64.const 4294967297
            i64.store offset=1057136
            br 1 (;@3;)
          end
          i32.const 0
          i32.const 0
          i32.load offset=1057140
          i32.const 1
          i32.add
          local.tee 5
          i32.store offset=1057140
          local.get 5
          i32.const 2
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 4
        i32.const 48
        i32.add
        local.get 3
        local.get 8
        local.get 7
        local.get 6
        call $core::panic::Location::internal_constructor::hcf293bdd1161e916
        local.get 4
        i32.const 36
        i32.add
        local.get 4
        i32.const 56
        i32.add
        i64.load
        i64.store align=4
        local.get 4
        local.get 2
        i32.store offset=24
        local.get 4
        i32.const 1048812
        i32.store offset=20
        local.get 4
        i32.const 1
        i32.store offset=16
        local.get 4
        local.get 4
        i64.load offset=48
        i64.store offset=28 align=4
        i32.const 0
        i32.load offset=1056644
        local.tee 3
        i32.const -1
        i32.le_s
        br_if 0 (;@2;)
        i32.const 0
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        i32.store offset=1056644
        block  ;; label = @3
          i32.const 0
          i32.load offset=1056652
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1056648
          local.set 3
          local.get 4
          i32.const 8
          i32.add
          local.get 0
          local.get 1
          i32.load offset=16
          call_indirect (type 4)
          local.get 4
          local.get 4
          i64.load offset=8
          i64.store offset=16
          local.get 3
          local.get 4
          i32.const 16
          i32.add
          local.get 2
          i32.load offset=12
          call_indirect (type 4)
          i32.const 0
          i32.load offset=1056644
          local.set 3
        end
        i32.const 0
        local.get 3
        i32.const -1
        i32.add
        i32.store offset=1056644
        local.get 5
        i32.const 1
        i32.le_u
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    call $rust_panic
    unreachable)
  (func $<std::panicking::continue_panic_fmt::PanicPayload_as_core::panic::BoxMeUp>::box_me_up::ha93a5fbf0ceb0d85 (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 3
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      local.set 3
      local.get 1
      i32.load
      local.set 4
      local.get 2
      i32.const 0
      i32.store offset=32
      local.get 2
      i64.const 1
      i64.store offset=24
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 4
      i64.load align=4
      i64.store offset=40
      local.get 2
      i32.const 36
      i32.add
      i32.const 1048624
      local.get 2
      i32.const 40
      i32.add
      call $core::fmt::write::hb137f2496e0ed1b6
      drop
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      local.get 2
      i32.load offset=32
      i32.store
      local.get 2
      local.get 2
      i64.load offset=24
      i64.store offset=8
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        i32.load
        local.tee 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 6
        i32.const 1
        call $__rust_dealloc
      end
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 3
      i32.load
      local.set 3
    end
    local.get 1
    i32.const 1
    i32.store offset=4
    local.get 1
    i32.const 12
    i32.add
    i32.load
    local.set 4
    local.get 1
    i32.const 8
    i32.add
    local.tee 1
    i32.load
    local.set 5
    local.get 1
    i64.const 0
    i64.store align=4
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
      unreachable
    end
    local.get 1
    local.get 4
    i32.store offset=8
    local.get 1
    local.get 5
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1050536
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 64
    i32.add
    global.set 0)
  (func $<std::panicking::continue_panic_fmt::PanicPayload_as_core::panic::BoxMeUp>::get::h57815b869d589859 (type 4) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.set 4
      local.get 2
      i32.const 0
      i32.store offset=32
      local.get 2
      i64.const 1
      i64.store offset=24
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 4
      i64.load align=4
      i64.store offset=40
      local.get 2
      i32.const 36
      i32.add
      i32.const 1048624
      local.get 2
      i32.const 40
      i32.add
      call $core::fmt::write::hb137f2496e0ed1b6
      drop
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      local.get 2
      i32.load offset=32
      i32.store
      local.get 2
      local.get 2
      i64.load offset=24
      i64.store offset=8
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
    end
    local.get 0
    i32.const 1050536
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 64
    i32.add
    global.set 0)
  (func $<std::panicking::begin_panic::PanicPayload<A>_as_core::panic::BoxMeUp>::box_me_up::h78e2a7abf0003e1e (type 4) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load
    local.set 2
    local.get 1
    i32.const 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          i32.const 1050588
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.load offset=4
        local.set 3
        i32.const 8
        i32.const 4
        call $__rust_alloc
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 3
        i32.store offset=4
        local.get 1
        local.get 2
        i32.store
        i32.const 1050572
        local.set 2
      end
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      return
    end
    i32.const 8
    i32.const 4
    call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
    unreachable)
  (func $<std::panicking::begin_panic::PanicPayload<A>_as_core::panic::BoxMeUp>::get::h9b1c7408f888c50f (type 4) (param i32 i32)
    (local i32)
    local.get 0
    i32.const 1050572
    i32.const 1050588
    local.get 1
    i32.load
    local.tee 2
    select
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 1
    local.get 2
    select
    i32.store)
  (func $rust_panic (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call $__rust_start_panic
    drop
    unreachable
    unreachable)
  (func $std::rt::lang_start_internal::h7924b428fced5051 (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 1
    i32.store offset=4
    local.get 4
    local.get 0
    i32.store
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 4
            i32.const 1
            call $__rust_alloc
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 1852399981
            i32.store align=1
            local.get 4
            i64.const 17179869188
            i64.store offset=12 align=4
            local.get 4
            local.get 0
            i32.store offset=8
            local.get 4
            i32.const 8
            i32.add
            call $std::thread::Thread::new::h5b7d617e777ade8f
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1056672
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                i32.const 0
                i64.const 1
                i64.store offset=1056672 align=4
                i32.const 0
                i32.const 0
                i32.store offset=1056680
                br 1 (;@5;)
              end
              i32.const 0
              i32.load offset=1056676
              local.tee 0
              i32.const 1
              i32.add
              i32.const 0
              i32.le_s
              br_if 2 (;@3;)
              i32.const 0
              i32.load offset=1056680
              br_if 3 (;@2;)
              local.get 0
              br_if 4 (;@1;)
            end
            i32.const 0
            local.set 0
            i32.const 0
            local.get 1
            i32.store offset=1056680
            i32.const 0
            i32.const 0
            i32.store offset=1056676
            local.get 4
            i32.const 0
            i32.store offset=24
            local.get 4
            i32.const 0
            i32.store offset=28
            local.get 4
            local.get 4
            i32.store offset=8
            block  ;; label = @5
              block  ;; label = @6
                i32.const 12
                local.get 4
                i32.const 8
                i32.add
                local.get 4
                i32.const 24
                i32.add
                local.get 4
                i32.const 28
                i32.add
                call $__rust_maybe_catch_panic
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1057136
                    i32.const 1
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load offset=1057140
                    i32.const -1
                    i32.add
                    local.set 0
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i64.const 1
                  i64.store offset=1057136
                  i32.const -1
                  local.set 0
                end
                i32.const 0
                local.get 0
                i32.store offset=1057140
                i32.const 1
                local.set 0
                local.get 4
                i32.load offset=28
                local.set 5
                local.get 4
                i32.load offset=24
                local.set 1
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 1
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1056636
              i32.const 3
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.const 1
              i32.store8 offset=28
              local.get 4
              local.get 4
              i32.const 28
              i32.add
              i32.store offset=8
              i32.const 1056636
              i32.const 0
              local.get 4
              i32.const 8
              i32.add
              i32.const 1050116
              call $std::sync::once::Once::call_inner::h65db2b3b821c8bf1
            end
            i32.const 101
            local.get 1
            local.get 0
            select
            local.set 6
            block  ;; label = @5
              local.get 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 5
              i32.load
              call_indirect (type 1)
              local.get 5
              i32.load offset=4
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              local.get 5
              i32.load offset=8
              call $__rust_dealloc
            end
            local.get 4
            i32.const 32
            i32.add
            global.set 0
            local.get 6
            return
          end
          i32.const 4
          i32.const 1
          call $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6
          unreachable
        end
        i32.const 1048704
        i32.const 24
        local.get 4
        i32.const 8
        i32.add
        i32.const 1048960
        call $core::result::unwrap_failed::hd11b409f5ba7889e
        unreachable
      end
      i32.const 1050460
      i32.const 38
      i32.const 1050444
      call $std::panicking::begin_panic::h837c5ed1e256db31
      unreachable
    end
    i32.const 1048688
    i32.const 16
    local.get 4
    i32.const 8
    i32.add
    i32.const 1048976
    call $core::result::unwrap_failed::hd11b409f5ba7889e
    unreachable)
  (func $<std::ffi::c_str::NulError_as_core::fmt::Debug>::fmt::h66d28a634ebb9809 (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.const 1050604
    i32.const 8
    call $core::fmt::Formatter::debug_tuple::h5e5a0a16c42c97a8
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.const 1049136
    call $core::fmt::builders::DebugTuple::field::h8c8629865c98eba0
    drop
    local.get 2
    local.get 0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.const 1050612
    call $core::fmt::builders::DebugTuple::field::h8c8629865c98eba0
    drop
    local.get 2
    call $core::fmt::builders::DebugTuple::finish::h782a14fa96fe1df8
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func $std::sys::wasm::process::ExitCode::as_i32::h4c9c3e9590aabf51 (type 5) (param i32) (result i32)
    local.get 0
    i32.load8_u)
  (func $__rust_maybe_catch_panic (type 9) (param i32 i32 i32 i32) (result i32)
    local.get 1
    local.get 0
    call_indirect (type 1)
    i32.const 0)
  (func $__rust_start_panic (type 5) (param i32) (result i32)
    unreachable
    unreachable)
  (func $dlmalloc::dlmalloc::Dlmalloc::malloc_alignment::h51e2543d99a8534c (type 5) (param i32) (result i32)
    i32.const 8)
  (func $dlmalloc::dlmalloc::Dlmalloc::malloc::h469ed7441b6b922b (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 245
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 2
          local.get 1
          i32.const -65587
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 11
          i32.add
          local.tee 1
          i32.const -8
          i32.and
          local.set 3
          local.get 0
          i32.load offset=4
          local.tee 4
          i32.eqz
          br_if 1 (;@2;)
          i32.const 0
          local.set 5
          block  ;; label = @4
            local.get 1
            i32.const 8
            i32.shr_u
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            i32.const 31
            local.set 5
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 6
            local.get 1
            i32.clz
            local.tee 1
            i32.sub
            i32.const 31
            i32.and
            i32.shr_u
            i32.const 1
            i32.and
            local.get 1
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 5
          end
          i32.const 0
          local.get 3
          i32.sub
          local.set 2
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 5
                i32.const 2
                i32.shl
                i32.add
                i32.const 272
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                local.get 3
                i32.const 0
                i32.const 25
                local.get 5
                i32.const 1
                i32.shr_u
                i32.sub
                i32.const 31
                i32.and
                local.get 5
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set 7
                i32.const 0
                local.set 8
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.tee 9
                    local.get 3
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 9
                    local.get 3
                    i32.sub
                    local.tee 9
                    local.get 2
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 9
                    local.set 2
                    local.get 1
                    local.set 8
                    local.get 9
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 2
                    local.get 1
                    local.set 8
                    br 3 (;@5;)
                  end
                  local.get 1
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee 9
                  local.get 6
                  local.get 9
                  local.get 1
                  local.get 7
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee 1
                  i32.ne
                  select
                  local.get 6
                  local.get 9
                  select
                  local.set 6
                  local.get 7
                  i32.const 1
                  i32.shl
                  local.set 7
                  local.get 1
                  br_if 0 (;@7;)
                end
                block  ;; label = @7
                  local.get 6
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 6
                  local.set 1
                  br 2 (;@5;)
                end
                local.get 8
                br_if 2 (;@4;)
              end
              i32.const 0
              local.set 8
              i32.const 2
              local.get 5
              i32.const 31
              i32.and
              i32.shl
              local.tee 1
              i32.const 0
              local.get 1
              i32.sub
              i32.or
              local.get 4
              i32.and
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              i32.const 0
              local.get 1
              i32.sub
              i32.and
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              i32.load
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
            end
            loop  ;; label = @5
              local.get 1
              i32.load offset=4
              i32.const -8
              i32.and
              local.tee 6
              local.get 3
              i32.ge_u
              local.get 6
              local.get 3
              i32.sub
              local.tee 9
              local.get 2
              i32.lt_u
              i32.and
              local.set 7
              block  ;; label = @6
                local.get 1
                i32.load offset=16
                local.tee 6
                br_if 0 (;@6;)
                local.get 1
                i32.const 20
                i32.add
                i32.load
                local.set 6
              end
              local.get 1
              local.get 8
              local.get 7
              select
              local.set 8
              local.get 9
              local.get 2
              local.get 7
              select
              local.set 2
              local.get 6
              local.set 1
              local.get 6
              br_if 0 (;@5;)
            end
            local.get 8
            i32.eqz
            br_if 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=400
            local.tee 1
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            local.get 3
            i32.sub
            i32.ge_u
            br_if 2 (;@2;)
          end
          local.get 0
          local.get 8
          call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 8
              local.get 3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 8
              local.get 3
              i32.add
              local.tee 1
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 2
              i32.add
              local.get 2
              i32.store
              block  ;; label = @6
                local.get 2
                i32.const 256
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                local.get 2
                call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5
                br 2 (;@4;)
              end
              local.get 0
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 2
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.set 3
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load
                  local.tee 6
                  i32.const 1
                  local.get 2
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee 2
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 0
                local.get 6
                local.get 2
                i32.or
                i32.store
                local.get 3
                local.set 2
              end
              local.get 3
              local.get 1
              i32.store offset=8
              local.get 2
              local.get 1
              i32.store offset=12
              local.get 1
              local.get 3
              i32.store offset=12
              local.get 1
              local.get 2
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 8
            local.get 2
            local.get 3
            i32.add
            local.tee 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 1
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
          end
          local.get 8
          i32.const 8
          i32.add
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load
              local.tee 8
              i32.const 16
              local.get 1
              i32.const 11
              i32.add
              i32.const -8
              i32.and
              local.get 1
              i32.const 11
              i32.lt_u
              select
              local.tee 3
              i32.const 3
              i32.shr_u
              local.tee 2
              i32.const 31
              i32.and
              local.tee 6
              i32.shr_u
              local.tee 1
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 3
              local.get 0
              i32.load offset=400
              i32.le_u
              br_if 3 (;@2;)
              local.get 1
              br_if 1 (;@4;)
              local.get 0
              i32.load offset=4
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              i32.const 0
              local.get 1
              i32.sub
              i32.and
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              i32.load
              local.tee 6
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.sub
              local.set 2
              local.get 6
              local.set 7
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 6
                  i32.load offset=16
                  local.tee 1
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee 1
                  i32.eqz
                  br_if 4 (;@3;)
                end
                local.get 1
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 3
                i32.sub
                local.tee 6
                local.get 2
                local.get 6
                local.get 2
                i32.lt_u
                local.tee 6
                select
                local.set 2
                local.get 1
                local.get 7
                local.get 6
                select
                local.set 7
                local.get 1
                local.set 6
                br 0 (;@6;)
              end
            end
            local.get 0
            local.get 1
            i32.const -1
            i32.xor
            i32.const 1
            i32.and
            local.get 2
            i32.add
            local.tee 3
            i32.const 3
            i32.shl
            i32.add
            local.tee 7
            i32.const 16
            i32.add
            i32.load
            local.tee 1
            i32.const 8
            i32.add
            local.set 2
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.load offset=8
                local.tee 6
                local.get 7
                i32.const 8
                i32.add
                local.tee 7
                i32.eq
                br_if 0 (;@6;)
                local.get 6
                local.get 7
                i32.store offset=12
                local.get 7
                local.get 6
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 0
              local.get 8
              i32.const -2
              local.get 3
              i32.rotl
              i32.and
              i32.store
            end
            local.get 1
            local.get 3
            i32.const 3
            i32.shl
            local.tee 3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 1
            local.get 3
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 3 (;@1;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 1
              local.get 6
              i32.shl
              i32.const 2
              local.get 6
              i32.shl
              local.tee 1
              i32.const 0
              local.get 1
              i32.sub
              i32.or
              i32.and
              local.tee 1
              i32.const 0
              local.get 1
              i32.sub
              i32.and
              i32.ctz
              local.tee 2
              i32.const 3
              i32.shl
              i32.add
              local.tee 7
              i32.const 16
              i32.add
              i32.load
              local.tee 1
              i32.load offset=8
              local.tee 6
              local.get 7
              i32.const 8
              i32.add
              local.tee 7
              i32.eq
              br_if 0 (;@5;)
              local.get 6
              local.get 7
              i32.store offset=12
              local.get 7
              local.get 6
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            local.get 8
            i32.const -2
            local.get 2
            i32.rotl
            i32.and
            i32.store
          end
          local.get 1
          i32.const 8
          i32.add
          local.set 6
          local.get 1
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 1
          local.get 3
          i32.add
          local.tee 7
          local.get 2
          i32.const 3
          i32.shl
          local.tee 2
          local.get 3
          i32.sub
          local.tee 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 2
          i32.add
          local.get 3
          i32.store
          block  ;; label = @4
            local.get 0
            i32.load offset=400
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            i32.const 3
            i32.shr_u
            local.tee 8
            i32.const 3
            i32.shl
            i32.add
            i32.const 8
            i32.add
            local.set 2
            local.get 0
            i32.load offset=408
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 9
                i32.const 1
                local.get 8
                i32.const 31
                i32.and
                i32.shl
                local.tee 8
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.load offset=8
                local.set 8
                br 1 (;@5;)
              end
              local.get 0
              local.get 9
              local.get 8
              i32.or
              i32.store
              local.get 2
              local.set 8
            end
            local.get 2
            local.get 1
            i32.store offset=8
            local.get 8
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 2
            i32.store offset=12
            local.get 1
            local.get 8
            i32.store offset=8
          end
          local.get 0
          local.get 7
          i32.store offset=408
          local.get 0
          local.get 3
          i32.store offset=400
          local.get 6
          return
        end
        local.get 0
        local.get 7
        call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 7
            local.get 3
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 7
            local.get 3
            i32.add
            local.tee 3
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 3
            local.get 2
            i32.add
            local.get 2
            i32.store
            block  ;; label = @5
              local.get 0
              i32.load offset=400
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.const 3
              i32.shr_u
              local.tee 8
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.add
              local.set 6
              local.get 0
              i32.load offset=408
              local.set 1
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load
                  local.tee 9
                  i32.const 1
                  local.get 8
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee 8
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 6
                  i32.load offset=8
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 0
                local.get 9
                local.get 8
                i32.or
                i32.store
                local.get 6
                local.set 8
              end
              local.get 6
              local.get 1
              i32.store offset=8
              local.get 8
              local.get 1
              i32.store offset=12
              local.get 1
              local.get 6
              i32.store offset=12
              local.get 1
              local.get 8
              i32.store offset=8
            end
            local.get 0
            local.get 3
            i32.store offset=408
            local.get 0
            local.get 2
            i32.store offset=400
            br 1 (;@3;)
          end
          local.get 7
          local.get 2
          local.get 3
          i32.add
          local.tee 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 1
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 7
        i32.const 8
        i32.add
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=400
                  local.tee 2
                  local.get 3
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=404
                  local.tee 1
                  local.get 3
                  i32.gt_u
                  br_if 3 (;@4;)
                  i32.const 0
                  local.set 2
                  local.get 3
                  i32.const 65583
                  i32.add
                  local.tee 6
                  i32.const 16
                  i32.shr_u
                  memory.grow
                  local.tee 1
                  i32.const -1
                  i32.eq
                  br_if 6 (;@1;)
                  local.get 1
                  i32.const 16
                  i32.shl
                  local.tee 8
                  i32.eqz
                  br_if 6 (;@1;)
                  local.get 0
                  local.get 0
                  i32.load offset=416
                  local.get 6
                  i32.const -65536
                  i32.and
                  local.tee 5
                  i32.add
                  local.tee 1
                  i32.store offset=416
                  local.get 0
                  local.get 0
                  i32.load offset=420
                  local.tee 6
                  local.get 1
                  local.get 6
                  local.get 1
                  i32.gt_u
                  select
                  i32.store offset=420
                  local.get 0
                  i32.load offset=412
                  local.tee 6
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 0
                  i32.const 424
                  i32.add
                  local.tee 4
                  local.set 1
                  loop  ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 7
                    local.get 1
                    i32.load offset=4
                    local.tee 9
                    i32.add
                    local.get 8
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 1
                    i32.load offset=8
                    local.tee 1
                    br_if 0 (;@8;)
                    br 5 (;@3;)
                  end
                end
                local.get 0
                i32.load offset=408
                local.set 1
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    local.get 3
                    i32.sub
                    local.tee 6
                    i32.const 15
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 0
                    i32.store offset=408
                    local.get 0
                    i32.const 0
                    i32.store offset=400
                    local.get 1
                    local.get 2
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 2
                    i32.add
                    local.tee 2
                    i32.const 4
                    i32.add
                    local.set 3
                    local.get 2
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    local.set 2
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 6
                  i32.store offset=400
                  local.get 0
                  local.get 1
                  local.get 3
                  i32.add
                  local.tee 7
                  i32.store offset=408
                  local.get 7
                  local.get 6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 2
                  i32.add
                  local.get 6
                  i32.store
                  local.get 3
                  i32.const 3
                  i32.or
                  local.set 2
                  local.get 1
                  i32.const 4
                  i32.add
                  local.set 3
                end
                local.get 3
                local.get 2
                i32.store
                local.get 1
                i32.const 8
                i32.add
                return
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=444
                  local.tee 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 8
                  i32.le_u
                  br_if 1 (;@6;)
                end
                local.get 0
                local.get 8
                i32.store offset=444
              end
              local.get 0
              i32.const 4095
              i32.store offset=448
              local.get 0
              local.get 8
              i32.store offset=424
              i32.const 0
              local.set 1
              local.get 0
              i32.const 436
              i32.add
              i32.const 0
              i32.store
              local.get 0
              i32.const 428
              i32.add
              local.get 5
              i32.store
              loop  ;; label = @6
                local.get 0
                local.get 1
                i32.add
                local.tee 6
                i32.const 16
                i32.add
                local.get 6
                i32.const 8
                i32.add
                local.tee 7
                i32.store
                local.get 6
                i32.const 20
                i32.add
                local.get 7
                i32.store
                local.get 1
                i32.const 8
                i32.add
                local.tee 1
                i32.const 256
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 0
              local.get 8
              i32.store offset=412
              local.get 0
              local.get 5
              i32.const -40
              i32.add
              local.tee 1
              i32.store offset=404
              local.get 8
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 8
              local.get 1
              i32.add
              i32.const 40
              i32.store offset=4
              local.get 0
              i32.const 2097152
              i32.store offset=440
              br 3 (;@2;)
            end
            local.get 1
            i32.load offset=12
            br_if 1 (;@3;)
            local.get 8
            local.get 6
            i32.le_u
            br_if 1 (;@3;)
            local.get 7
            local.get 6
            i32.gt_u
            br_if 1 (;@3;)
            local.get 1
            local.get 9
            local.get 5
            i32.add
            i32.store offset=4
            local.get 0
            local.get 0
            i32.load offset=412
            local.tee 1
            i32.const 15
            i32.add
            i32.const -8
            i32.and
            local.tee 6
            i32.const -8
            i32.add
            i32.store offset=412
            local.get 0
            local.get 1
            local.get 6
            i32.sub
            local.get 0
            i32.load offset=404
            local.get 5
            i32.add
            local.tee 7
            i32.add
            i32.const 8
            i32.add
            local.tee 8
            i32.store offset=404
            local.get 6
            i32.const -4
            i32.add
            local.get 8
            i32.const 1
            i32.or
            i32.store
            local.get 1
            local.get 7
            i32.add
            i32.const 40
            i32.store offset=4
            local.get 0
            i32.const 2097152
            i32.store offset=440
            br 2 (;@2;)
          end
          local.get 0
          local.get 1
          local.get 3
          i32.sub
          local.tee 2
          i32.store offset=404
          local.get 0
          local.get 0
          i32.load offset=412
          local.tee 1
          local.get 3
          i32.add
          local.tee 6
          i32.store offset=412
          local.get 6
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 8
          i32.add
          return
        end
        local.get 0
        local.get 0
        i32.load offset=444
        local.tee 1
        local.get 8
        local.get 1
        local.get 8
        i32.lt_u
        select
        i32.store offset=444
        local.get 8
        local.get 5
        i32.add
        local.set 7
        local.get 4
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            loop  ;; label = @5
              local.get 1
              i32.load
              local.get 7
              i32.eq
              br_if 1 (;@4;)
              local.get 1
              i32.load offset=8
              local.tee 1
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          local.get 1
          i32.load offset=12
          br_if 0 (;@3;)
          local.get 1
          local.get 8
          i32.store
          local.get 1
          local.get 1
          i32.load offset=4
          local.get 5
          i32.add
          i32.store offset=4
          local.get 8
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 3
          i32.add
          local.set 1
          local.get 7
          local.get 8
          i32.sub
          local.get 3
          i32.sub
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=412
                local.get 7
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=408
                local.get 7
                i32.eq
                br_if 1 (;@5;)
                block  ;; label = @7
                  local.get 7
                  i32.load offset=4
                  local.tee 2
                  i32.const 3
                  i32.and
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 2
                      i32.const -8
                      i32.and
                      local.tee 6
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 7
                      call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
                      br 1 (;@8;)
                    end
                    block  ;; label = @9
                      local.get 7
                      i32.load offset=12
                      local.tee 9
                      local.get 7
                      i32.load offset=8
                      local.tee 5
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 5
                      local.get 9
                      i32.store offset=12
                      local.get 9
                      local.get 5
                      i32.store offset=8
                      br 1 (;@8;)
                    end
                    local.get 0
                    local.get 0
                    i32.load
                    i32.const -2
                    local.get 2
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store
                  end
                  local.get 6
                  local.get 3
                  i32.add
                  local.set 3
                  local.get 7
                  local.get 6
                  i32.add
                  local.set 7
                end
                local.get 7
                local.get 7
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 1
                local.get 3
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 3
                i32.add
                local.get 3
                i32.store
                block  ;; label = @7
                  local.get 3
                  i32.const 256
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 1
                  local.get 3
                  call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5
                  br 3 (;@4;)
                end
                local.get 0
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 2
                i32.const 3
                i32.shl
                i32.add
                i32.const 8
                i32.add
                local.set 3
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load
                    local.tee 6
                    i32.const 1
                    local.get 2
                    i32.const 31
                    i32.and
                    i32.shl
                    local.tee 2
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load offset=8
                    local.set 2
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 6
                  local.get 2
                  i32.or
                  i32.store
                  local.get 3
                  local.set 2
                end
                local.get 3
                local.get 1
                i32.store offset=8
                local.get 2
                local.get 1
                i32.store offset=12
                local.get 1
                local.get 3
                i32.store offset=12
                local.get 1
                local.get 2
                i32.store offset=8
                br 2 (;@4;)
              end
              local.get 0
              local.get 1
              i32.store offset=412
              local.get 0
              local.get 0
              i32.load offset=404
              local.get 3
              i32.add
              local.tee 3
              i32.store offset=404
              local.get 1
              local.get 3
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 0
            local.get 1
            i32.store offset=408
            local.get 0
            local.get 0
            i32.load offset=400
            local.get 3
            i32.add
            local.tee 3
            i32.store offset=400
            local.get 1
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 3
            i32.add
            local.get 3
            i32.store
          end
          local.get 8
          i32.const 8
          i32.add
          return
        end
        local.get 4
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 7
              local.get 6
              i32.gt_u
              br_if 0 (;@5;)
              local.get 7
              local.get 1
              i32.load offset=4
              i32.add
              local.tee 7
              local.get 6
              i32.gt_u
              br_if 2 (;@3;)
            end
            local.get 1
            i32.load offset=8
            local.set 1
            br 0 (;@4;)
          end
        end
        local.get 0
        local.get 8
        i32.store offset=412
        local.get 0
        local.get 5
        i32.const -40
        i32.add
        local.tee 1
        i32.store offset=404
        local.get 8
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 8
        local.get 1
        i32.add
        i32.const 40
        i32.store offset=4
        local.get 0
        i32.const 2097152
        i32.store offset=440
        local.get 6
        local.get 7
        i32.const -32
        i32.add
        i32.const -8
        i32.and
        i32.const -8
        i32.add
        local.tee 1
        local.get 1
        local.get 6
        i32.const 16
        i32.add
        i32.lt_u
        select
        local.tee 9
        i32.const 27
        i32.store offset=4
        local.get 4
        i64.load align=4
        local.set 10
        local.get 9
        i32.const 16
        i32.add
        local.get 4
        i32.const 8
        i32.add
        i64.load align=4
        i64.store align=4
        local.get 9
        local.get 10
        i64.store offset=8 align=4
        local.get 0
        i32.const 436
        i32.add
        i32.const 0
        i32.store
        local.get 0
        i32.const 428
        i32.add
        local.get 5
        i32.store
        local.get 0
        local.get 8
        i32.store offset=424
        local.get 0
        i32.const 432
        i32.add
        local.get 9
        i32.const 8
        i32.add
        i32.store
        local.get 9
        i32.const 28
        i32.add
        local.set 1
        loop  ;; label = @3
          local.get 1
          i32.const 7
          i32.store
          local.get 7
          local.get 1
          i32.const 4
          i32.add
          local.tee 1
          i32.gt_u
          br_if 0 (;@3;)
        end
        local.get 9
        local.get 6
        i32.eq
        br_if 0 (;@2;)
        local.get 9
        local.get 9
        i32.load offset=4
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 6
        local.get 9
        local.get 6
        i32.sub
        local.tee 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 9
        local.get 1
        i32.store
        block  ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 6
          local.get 1
          call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.const 3
        i32.shr_u
        local.tee 7
        i32.const 3
        i32.shl
        i32.add
        i32.const 8
        i32.add
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load
            local.tee 8
            i32.const 1
            local.get 7
            i32.const 31
            i32.and
            i32.shl
            local.tee 7
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=8
            local.set 7
            br 1 (;@3;)
          end
          local.get 0
          local.get 8
          local.get 7
          i32.or
          i32.store
          local.get 1
          local.set 7
        end
        local.get 1
        local.get 6
        i32.store offset=8
        local.get 7
        local.get 6
        i32.store offset=12
        local.get 6
        local.get 1
        i32.store offset=12
        local.get 6
        local.get 7
        i32.store offset=8
      end
      local.get 0
      i32.load offset=404
      local.tee 1
      local.get 3
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 3
      i32.sub
      local.tee 2
      i32.store offset=404
      local.get 0
      local.get 0
      i32.load offset=412
      local.tee 1
      local.get 3
      i32.add
      local.tee 6
      i32.store offset=412
      local.get 6
      local.get 2
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 3
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 1
      i32.const 8
      i32.add
      return
    end
    local.get 2)
  (func $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88 (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.load offset=24
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=12
          local.tee 3
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 20
          i32.const 16
          local.get 1
          i32.const 20
          i32.add
          local.tee 3
          i32.load
          local.tee 4
          select
          i32.add
          i32.load
          local.tee 5
          br_if 1 (;@2;)
          i32.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 1
        i32.load offset=8
        local.tee 5
        local.get 3
        i32.store offset=12
        local.get 3
        local.get 5
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 3
      local.get 1
      i32.const 16
      i32.add
      local.get 4
      select
      local.set 4
      loop  ;; label = @2
        local.get 4
        local.set 6
        block  ;; label = @3
          local.get 5
          local.tee 3
          i32.const 20
          i32.add
          local.tee 4
          i32.load
          local.tee 5
          br_if 0 (;@3;)
          local.get 3
          i32.const 16
          i32.add
          local.set 4
          local.get 3
          i32.load offset=16
          local.set 5
        end
        local.get 5
        br_if 0 (;@2;)
      end
      local.get 6
      i32.const 0
      i32.store
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 1
          i32.load offset=28
          i32.const 2
          i32.shl
          i32.add
          i32.const 272
          i32.add
          local.tee 5
          i32.load
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          i32.store
          local.get 3
          br_if 1 (;@2;)
          local.get 0
          local.get 0
          i32.load offset=4
          i32.const -2
          local.get 1
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=4
          return
        end
        local.get 2
        i32.const 16
        i32.const 20
        local.get 2
        i32.load offset=16
        local.get 1
        i32.eq
        select
        i32.add
        local.get 3
        i32.store
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 3
      local.get 2
      i32.store offset=24
      block  ;; label = @2
        local.get 1
        i32.load offset=16
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 3
        i32.store offset=24
      end
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 20
      i32.add
      local.get 5
      i32.store
      local.get 5
      local.get 3
      i32.store offset=24
    end)
  (func $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5 (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 8
        i32.shr_u
        local.tee 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      i32.const 31
      local.set 4
      local.get 2
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 6
      local.get 3
      i32.clz
      local.tee 4
      i32.sub
      i32.const 31
      i32.and
      i32.shr_u
      i32.const 1
      i32.and
      local.get 4
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
      local.set 4
    end
    local.get 1
    i64.const 0
    i64.store offset=16 align=4
    local.get 1
    local.get 4
    i32.store offset=28
    local.get 0
    local.get 4
    i32.const 2
    i32.shl
    i32.add
    i32.const 272
    i32.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 5
              i32.const 1
              local.get 4
              i32.const 31
              i32.and
              i32.shl
              local.tee 6
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.load
              local.tee 3
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 2
              i32.ne
              br_if 1 (;@4;)
              local.get 3
              local.set 4
              br 2 (;@3;)
            end
            local.get 0
            local.get 5
            local.get 6
            i32.or
            i32.store offset=4
            local.get 3
            local.get 1
            i32.store
            local.get 1
            local.get 3
            i32.store offset=24
            br 3 (;@1;)
          end
          local.get 2
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          i32.const 31
          i32.and
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 0
          loop  ;; label = @4
            local.get 3
            local.get 0
            i32.const 29
            i32.shr_u
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 5
            i32.load
            local.tee 4
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.const 1
            i32.shl
            local.set 0
            local.get 4
            local.set 3
            local.get 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 2
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 4
        i32.load offset=8
        local.tee 0
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        local.get 1
        i32.const 0
        i32.store offset=24
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      local.get 5
      local.get 1
      i32.store
      local.get 1
      local.get 3
      i32.store offset=24
    end
    local.get 1
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 1
    i32.store offset=8)
  (func $dlmalloc::dlmalloc::Dlmalloc::realloc::h4166d0a35cb7b3c3 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.const -65588
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 16
      local.get 2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 2
      i32.const 11
      i32.lt_u
      select
      local.set 4
      local.get 1
      i32.const -4
      i32.add
      local.tee 5
      i32.load
      local.tee 6
      i32.const -8
      i32.and
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 6
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -8
                    i32.add
                    local.tee 8
                    local.get 7
                    i32.add
                    local.set 9
                    local.get 7
                    local.get 4
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 0
                    i32.load offset=412
                    local.get 9
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 0
                    i32.load offset=408
                    local.get 9
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 9
                    i32.load offset=4
                    local.tee 6
                    i32.const 2
                    i32.and
                    br_if 6 (;@2;)
                    local.get 6
                    i32.const -8
                    i32.and
                    local.tee 10
                    local.get 7
                    i32.add
                    local.tee 7
                    local.get 4
                    i32.ge_u
                    br_if 4 (;@4;)
                    br 6 (;@2;)
                  end
                  local.get 4
                  i32.const 256
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 7
                  local.get 4
                  i32.const 4
                  i32.or
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 7
                  local.get 4
                  i32.sub
                  i32.const 131073
                  i32.ge_u
                  br_if 5 (;@2;)
                  br 4 (;@3;)
                end
                local.get 7
                local.get 4
                i32.sub
                local.tee 2
                i32.const 16
                i32.lt_u
                br_if 3 (;@3;)
                local.get 5
                local.get 4
                local.get 6
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 8
                local.get 4
                i32.add
                local.tee 3
                local.get 2
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 9
                local.get 9
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 0
                local.get 3
                local.get 2
                call $dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::h1b2a50d578697c95
                br 3 (;@3;)
              end
              local.get 0
              i32.load offset=404
              local.get 7
              i32.add
              local.tee 7
              local.get 4
              i32.le_u
              br_if 3 (;@2;)
              local.get 5
              local.get 4
              local.get 6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 8
              local.get 4
              i32.add
              local.tee 2
              local.get 7
              local.get 4
              i32.sub
              local.tee 3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 3
              i32.store offset=404
              local.get 0
              local.get 2
              i32.store offset=412
              br 2 (;@3;)
            end
            local.get 0
            i32.load offset=400
            local.get 7
            i32.add
            local.tee 7
            local.get 4
            i32.lt_u
            br_if 2 (;@2;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                local.get 4
                i32.sub
                local.tee 2
                i32.const 15
                i32.gt_u
                br_if 0 (;@6;)
                local.get 5
                local.get 6
                i32.const 1
                i32.and
                local.get 7
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 8
                local.get 7
                i32.add
                local.tee 2
                local.get 2
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set 2
                i32.const 0
                local.set 3
                br 1 (;@5;)
              end
              local.get 5
              local.get 4
              local.get 6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 8
              local.get 4
              i32.add
              local.tee 3
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 8
              local.get 7
              i32.add
              local.tee 4
              local.get 2
              i32.store
              local.get 4
              local.get 4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
            end
            local.get 0
            local.get 3
            i32.store offset=408
            local.get 0
            local.get 2
            i32.store offset=400
            br 1 (;@3;)
          end
          local.get 7
          local.get 4
          i32.sub
          local.set 2
          block  ;; label = @4
            block  ;; label = @5
              local.get 10
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 9
              call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 9
              i32.load offset=12
              local.tee 3
              local.get 9
              i32.load offset=8
              local.tee 9
              i32.eq
              br_if 0 (;@5;)
              local.get 9
              local.get 3
              i32.store offset=12
              local.get 3
              local.get 9
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            local.get 0
            i32.load
            i32.const -2
            local.get 6
            i32.const 3
            i32.shr_u
            i32.rotl
            i32.and
            i32.store
          end
          block  ;; label = @4
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 5
            local.get 4
            local.get 5
            i32.load
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 8
            local.get 4
            i32.add
            local.tee 3
            local.get 2
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 7
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 3
            local.get 2
            call $dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::h1b2a50d578697c95
            br 1 (;@3;)
          end
          local.get 5
          local.get 7
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 8
          local.get 7
          i32.add
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 1
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      call $dlmalloc::dlmalloc::Dlmalloc::malloc::h469ed7441b6b922b
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.get 1
      local.get 2
      local.get 5
      i32.load
      local.tee 3
      i32.const -8
      i32.and
      i32.const 4
      i32.const 8
      local.get 3
      i32.const 3
      i32.and
      select
      i32.sub
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      call $memcpy
      local.set 2
      local.get 0
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc::free::h81e375dc80848071
      local.get 2
      return
    end
    local.get 3)
  (func $dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::h1b2a50d578697c95 (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    i32.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 4
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 4
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          i32.load
          local.tee 4
          local.get 2
          i32.add
          local.set 2
          block  ;; label = @4
            local.get 0
            i32.load offset=408
            local.get 1
            local.get 4
            i32.sub
            local.tee 1
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.load offset=4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
            local.get 0
            local.get 2
            i32.store offset=400
            local.get 3
            local.get 3
            i32.load offset=4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 3
            local.get 2
            i32.store
            return
          end
          block  ;; label = @4
            local.get 4
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 1
            i32.load offset=12
            local.tee 5
            local.get 1
            i32.load offset=8
            local.tee 6
            i32.eq
            br_if 0 (;@4;)
            local.get 6
            local.get 5
            i32.store offset=12
            local.get 5
            local.get 6
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 0
          local.get 0
          i32.load
          i32.const -2
          local.get 4
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store
        end
        block  ;; label = @3
          local.get 3
          i32.load offset=4
          local.tee 4
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 2
          i32.add
          local.get 2
          i32.store
          br 2 (;@1;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=412
            local.get 3
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=408
            local.get 3
            i32.ne
            br_if 1 (;@3;)
            local.get 0
            local.get 1
            i32.store offset=408
            local.get 0
            local.get 0
            i32.load offset=400
            local.get 2
            i32.add
            local.tee 2
            i32.store offset=400
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 2
            i32.add
            local.get 2
            i32.store
            return
          end
          local.get 0
          local.get 1
          i32.store offset=412
          local.get 0
          local.get 0
          i32.load offset=404
          local.get 2
          i32.add
          local.tee 2
          i32.store offset=404
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 0
          i32.load offset=408
          i32.ne
          br_if 1 (;@2;)
          local.get 0
          i32.const 0
          i32.store offset=400
          local.get 0
          i32.const 0
          i32.store offset=408
          return
        end
        local.get 4
        i32.const -8
        i32.and
        local.tee 5
        local.get 2
        i32.add
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.load offset=12
            local.tee 5
            local.get 3
            i32.load offset=8
            local.tee 3
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            local.get 5
            i32.store offset=12
            local.get 5
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 0
          local.get 0
          i32.load
          i32.const -2
          local.get 4
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store
        end
        local.get 1
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 2
        i32.add
        local.get 2
        i32.store
        local.get 1
        local.get 0
        i32.load offset=408
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i32.store offset=400
      end
      return
    end
    block  ;; label = @1
      local.get 2
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5
      return
    end
    local.get 0
    local.get 2
    i32.const 3
    i32.shr_u
    local.tee 3
    i32.const 3
    i32.shl
    i32.add
    i32.const 8
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 4
        i32.const 1
        local.get 3
        i32.const 31
        i32.and
        i32.shl
        local.tee 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=8
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      local.get 3
      i32.or
      i32.store
      local.get 2
      local.set 0
    end
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 2
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8)
  (func $dlmalloc::dlmalloc::Dlmalloc::free::h81e375dc80848071 (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.const -8
    i32.add
    local.tee 2
    local.get 1
    i32.const -4
    i32.add
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 1
    i32.add
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 3
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 2
            i32.load
            local.tee 3
            local.get 1
            i32.add
            local.set 1
            block  ;; label = @5
              local.get 0
              i32.load offset=408
              local.get 2
              local.get 3
              i32.sub
              local.tee 2
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 1 (;@4;)
              local.get 0
              local.get 1
              i32.store offset=400
              local.get 4
              local.get 4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              i32.add
              local.get 1
              i32.store
              return
            end
            block  ;; label = @5
              local.get 3
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 2
              call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 2
              i32.load offset=12
              local.tee 5
              local.get 2
              i32.load offset=8
              local.tee 6
              i32.eq
              br_if 0 (;@5;)
              local.get 6
              local.get 5
              i32.store offset=12
              local.get 5
              local.get 6
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            local.get 0
            i32.load
            i32.const -2
            local.get 3
            i32.const 3
            i32.shr_u
            i32.rotl
            i32.and
            i32.store
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              i32.add
              local.get 1
              i32.store
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=412
                local.get 4
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=408
                local.get 4
                i32.ne
                br_if 1 (;@5;)
                local.get 0
                local.get 2
                i32.store offset=408
                local.get 0
                local.get 0
                i32.load offset=400
                local.get 1
                i32.add
                local.tee 1
                i32.store offset=400
                local.get 2
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 2
                local.get 1
                i32.add
                local.get 1
                i32.store
                return
              end
              local.get 0
              local.get 2
              i32.store offset=412
              local.get 0
              local.get 0
              i32.load offset=404
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=404
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              block  ;; label = @6
                local.get 2
                local.get 0
                i32.load offset=408
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 0
                i32.store offset=400
                local.get 0
                i32.const 0
                i32.store offset=408
              end
              local.get 0
              i32.load offset=440
              local.tee 3
              local.get 1
              i32.ge_u
              br_if 2 (;@3;)
              local.get 0
              i32.load offset=412
              local.tee 1
              i32.eqz
              br_if 2 (;@3;)
              block  ;; label = @6
                local.get 0
                i32.load offset=404
                local.tee 5
                i32.const 41
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 424
                i32.add
                local.set 2
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load
                    local.tee 4
                    local.get 1
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 2
                    i32.load offset=4
                    i32.add
                    local.get 1
                    i32.gt_u
                    br_if 2 (;@6;)
                  end
                  local.get 2
                  i32.load offset=8
                  local.tee 2
                  br_if 0 (;@7;)
                end
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 432
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 0 (;@7;)
                  i32.const 4095
                  local.set 2
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 2
                loop  ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load offset=8
                  local.tee 1
                  br_if 0 (;@7;)
                end
                local.get 2
                i32.const 4095
                local.get 2
                i32.const 4095
                i32.gt_u
                select
                local.set 2
              end
              local.get 0
              local.get 2
              i32.store offset=448
              local.get 5
              local.get 3
              i32.le_u
              br_if 2 (;@3;)
              local.get 0
              i32.const -1
              i32.store offset=440
              return
            end
            local.get 3
            i32.const -8
            i32.and
            local.tee 5
            local.get 1
            i32.add
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 5
                i32.const 256
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                local.get 4
                call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::h2e885e67fd9a5a88
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 4
                i32.load offset=12
                local.tee 5
                local.get 4
                i32.load offset=8
                local.tee 4
                i32.eq
                br_if 0 (;@6;)
                local.get 4
                local.get 5
                i32.store offset=12
                local.get 5
                local.get 4
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 0
              local.get 0
              i32.load
              i32.const -2
              local.get 3
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store
            end
            local.get 2
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            i32.add
            local.get 1
            i32.store
            local.get 2
            local.get 0
            i32.load offset=408
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            i32.store offset=400
            br 1 (;@3;)
          end
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          local.get 2
          local.get 1
          call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::ha1d98616644771a5
          local.get 0
          local.get 0
          i32.load offset=448
          i32.const -1
          i32.add
          local.tee 2
          i32.store offset=448
          local.get 2
          br_if 0 (;@3;)
          local.get 0
          i32.const 432
          i32.add
          i32.load
          local.tee 1
          br_if 2 (;@1;)
          local.get 0
          i32.const 4095
          i32.store offset=448
          return
        end
        return
      end
      local.get 0
      local.get 1
      i32.const 3
      i32.shr_u
      local.tee 4
      i32.const 3
      i32.shl
      i32.add
      i32.const 8
      i32.add
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load
          local.tee 3
          i32.const 1
          local.get 4
          i32.const 31
          i32.and
          i32.shl
          local.tee 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=8
          local.set 0
          br 1 (;@2;)
        end
        local.get 0
        local.get 3
        local.get 4
        i32.or
        i32.store
        local.get 1
        local.set 0
      end
      local.get 1
      local.get 2
      i32.store offset=8
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 2
      local.get 1
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      return
    end
    i32.const 0
    local.set 2
    loop  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.set 2
      local.get 1
      i32.load offset=8
      local.tee 1
      br_if 0 (;@1;)
    end
    local.get 0
    local.get 2
    i32.const 4095
    local.get 2
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448)
  (func $dlmalloc::dlmalloc::Dlmalloc::memalign::h222fd434a08907f4 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      i32.const -65587
      local.get 1
      i32.const 16
      local.get 1
      i32.const 16
      i32.gt_u
      select
      local.tee 1
      i32.sub
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 16
      local.get 2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 2
      i32.const 11
      i32.lt_u
      select
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call $dlmalloc::dlmalloc::Dlmalloc::malloc::h469ed7441b6b922b
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const -8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 5
          local.get 2
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        i32.const -4
        i32.add
        local.tee 6
        i32.load
        local.tee 7
        i32.const -8
        i32.and
        local.get 5
        local.get 2
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 2
        local.get 2
        local.get 1
        i32.add
        local.get 2
        local.get 3
        i32.sub
        i32.const 16
        i32.gt_u
        select
        local.tee 1
        local.get 3
        i32.sub
        local.tee 2
        i32.sub
        local.set 5
        block  ;; label = @3
          local.get 7
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 1
          local.get 5
          i32.add
          local.tee 5
          local.get 5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 6
          local.get 2
          local.get 6
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          local.get 2
          call $dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::h1b2a50d578697c95
          br 1 (;@2;)
        end
        local.get 3
        i32.load
        local.set 3
        local.get 1
        local.get 5
        i32.store offset=4
        local.get 1
        local.get 3
        local.get 2
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const -8
        i32.and
        local.tee 3
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        local.get 2
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 1
        local.get 4
        i32.add
        local.tee 2
        local.get 3
        local.get 4
        i32.sub
        local.tee 4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        local.get 3
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 4
        call $dlmalloc::dlmalloc::Dlmalloc::dispose_chunk::h1b2a50d578697c95
      end
      local.get 1
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3)
  (func $alloc::alloc::handle_alloc_error::had196cbeaa38b1f6 (type 4) (param i32 i32)
    local.get 0
    local.get 1
    call $rust_oom
    unreachable)
  (func $alloc::raw_vec::capacity_overflow::hc538c246d520d486 (type 0)
    i32.const 1050844
    call $core::panicking::panic::h0142ee7f4c64bd08
    unreachable)
  (func $alloc::string::<impl_core::convert::From<alloc::string::String>_for_alloc::vec::Vec<u8>>::from::h6ba865a046d5c1cb (type 4) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i32.load
    i32.store)
  (func $core::ptr::real_drop_in_place::h219a4dd4886f8f7b (type 1) (param i32))
  (func $core::ptr::real_drop_in_place::he0f5620a77bcc8c4 (type 1) (param i32))
  (func $core::panicking::panic_bounds_check::h1fae5a314994f748 (type 3) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    local.get 1
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 53
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1050976
    i32.store offset=8
    local.get 3
    i32.const 53
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $core::slice::slice_index_len_fail::h08f636efd7156c0a (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 2
    i32.const 44
    i32.add
    i32.const 53
    i32.store
    local.get 2
    i64.const 2
    i64.store offset=12 align=4
    local.get 2
    i32.const 1051228
    i32.store offset=8
    local.get 2
    i32.const 53
    i32.store offset=36
    local.get 2
    local.get 2
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 2
    local.get 2
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 2
    local.get 2
    i32.store offset=32
    local.get 2
    i32.const 8
    i32.add
    i32.const 1051244
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $core::panicking::panic::h0142ee7f4c64bd08 (type 1) (param i32)
    (local i32 i64 i64 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i64.load offset=8 align=4
    local.set 2
    local.get 0
    i64.load offset=16 align=4
    local.set 3
    local.get 0
    i64.load align=4
    local.set 4
    local.get 1
    i64.const 4
    i64.store offset=16
    local.get 1
    i64.const 1
    i64.store offset=4 align=4
    local.get 1
    local.get 4
    i64.store offset=24
    local.get 1
    local.get 1
    i32.const 24
    i32.add
    i32.store
    local.get 1
    local.get 3
    i64.store offset=40
    local.get 1
    local.get 2
    i64.store offset=32
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $core::slice::slice_index_order_fail::h45638c641c9b3b30 (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 2
    i32.const 44
    i32.add
    i32.const 53
    i32.store
    local.get 2
    i64.const 2
    i64.store offset=12 align=4
    local.get 2
    i32.const 1051296
    i32.store offset=8
    local.get 2
    i32.const 53
    i32.store offset=36
    local.get 2
    local.get 2
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 2
    local.get 2
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 2
    local.get 2
    i32.store offset=32
    local.get 2
    i32.const 8
    i32.add
    i32.const 1051312
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $core::panicking::panic_fmt::h095d4614168d6bd6 (type 4) (param i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i64.load align=4
    local.set 3
    local.get 2
    i32.const 20
    i32.add
    local.get 1
    i64.load offset=8 align=4
    i64.store align=4
    local.get 2
    local.get 3
    i64.store offset=12 align=4
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 1050908
    i32.store offset=4
    local.get 2
    i32.const 1
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $core::fmt::Formatter::pad::hd367b6bcbe89f492 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=16
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 4
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 6)
            local.set 3
            br 3 (;@1;)
          end
          local.get 3
          i32.eqz
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          local.get 2
          i32.add
          local.set 5
          local.get 0
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 7
          local.get 1
          local.set 3
          local.get 1
          local.set 8
          loop  ;; label = @4
            local.get 3
            i32.const 1
            i32.add
            local.set 9
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.load8_s
                  local.tee 10
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 9
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 11
                      local.get 5
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u offset=1
                    i32.const 63
                    i32.and
                    local.set 11
                    local.get 3
                    i32.const 2
                    i32.add
                    local.tee 9
                    local.set 3
                  end
                  local.get 10
                  i32.const 31
                  i32.and
                  local.set 12
                  block  ;; label = @8
                    local.get 10
                    i32.const 255
                    i32.and
                    local.tee 10
                    i32.const 223
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 13
                      local.get 5
                      local.set 14
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 13
                    local.get 3
                    i32.const 1
                    i32.add
                    local.tee 9
                    local.set 14
                  end
                  local.get 13
                  local.get 11
                  i32.const 6
                  i32.shl
                  i32.or
                  local.set 11
                  block  ;; label = @8
                    local.get 10
                    i32.const 240
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 14
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 10
                      local.get 9
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 14
                    i32.const 1
                    i32.add
                    local.set 3
                    local.get 14
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 10
                  end
                  local.get 11
                  i32.const 6
                  i32.shl
                  local.get 12
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.get 10
                  i32.or
                  local.tee 10
                  i32.const 1114112
                  i32.ne
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                local.get 10
                i32.const 255
                i32.and
                local.set 10
              end
              local.get 9
              local.set 3
            end
            block  ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 8
              i32.sub
              local.get 3
              i32.add
              local.set 7
              local.get 3
              local.set 8
              local.get 5
              local.get 3
              i32.ne
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
          end
          local.get 10
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 2
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              local.get 7
              local.get 2
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 1
            local.set 3
          end
          local.get 7
          local.get 2
          local.get 3
          select
          local.set 2
          local.get 3
          local.get 1
          local.get 3
          select
          local.set 1
        end
        local.get 4
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        return
      end
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 2
        local.get 9
        i32.sub
        local.get 0
        i32.load offset=12
        local.tee 6
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        return
      end
      i32.const 0
      local.set 7
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 9
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 2
      i32.sub
      local.get 6
      i32.add
      local.set 10
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            local.get 0
            i32.load8_u offset=48
            local.tee 3
            local.get 3
            i32.const 3
            i32.eq
            select
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 10
          local.set 7
          i32.const 0
          local.set 10
          br 1 (;@2;)
        end
        local.get 10
        i32.const 1
        i32.shr_u
        local.set 7
        local.get 10
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 10
      end
      local.get 7
      i32.const 1
      i32.add
      local.set 3
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=28
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      local.get 0
      i32.load offset=4
      local.set 9
      i32.const 1
      local.set 3
      local.get 0
      i32.load offset=24
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 6)
      br_if 0 (;@1;)
      local.get 10
      i32.const 1
      i32.add
      local.set 3
      local.get 0
      i32.load offset=28
      local.set 10
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 0
        local.get 9
        local.get 10
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      return
    end
    local.get 3)
  (func $core::str::slice_error_fail::h571f7e6f7dc53361 (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 2
    i32.store offset=8
    i32.const 1
    local.set 5
    local.get 1
    local.set 6
    block  ;; label = @1
      local.get 1
      i32.const 257
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.sub
      local.set 7
      i32.const 256
      local.set 8
      loop  ;; label = @2
        block  ;; label = @3
          local.get 8
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          local.get 0
          local.get 8
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          local.get 8
          local.set 6
          br 2 (;@1;)
        end
        local.get 8
        i32.const -1
        i32.add
        local.set 6
        i32.const 0
        local.set 5
        local.get 8
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
        local.get 7
        local.get 8
        i32.add
        local.set 9
        local.get 6
        local.set 8
        local.get 9
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 4
    local.get 6
    i32.store offset=20
    local.get 4
    local.get 0
    i32.store offset=16
    local.get 4
    i32.const 0
    i32.const 5
    local.get 5
    select
    i32.store offset=28
    local.get 4
    i32.const 1050869
    i32.const 1051419
    local.get 5
    select
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 1
            i32.gt_u
            local.tee 8
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 3
            i32.gt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.le_u
                br_if 1 (;@5;)
                local.get 0
                local.get 2
                i32.add
                i32.load8_s
                i32.const -64
                i32.lt_s
                br_if 1 (;@5;)
              end
              local.get 3
              local.set 2
            end
            local.get 4
            local.get 2
            i32.store offset=32
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.eq
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.add
            local.set 9
            loop  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 1
                i32.ge_u
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.add
                i32.load8_s
                i32.const -64
                i32.ge_s
                br_if 4 (;@2;)
              end
              local.get 2
              i32.const -1
              i32.add
              local.set 8
              local.get 2
              i32.const 1
              i32.eq
              br_if 4 (;@1;)
              local.get 9
              local.get 2
              i32.eq
              local.set 6
              local.get 8
              local.set 2
              local.get 6
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          local.get 4
          local.get 2
          local.get 3
          local.get 8
          select
          i32.store offset=40
          local.get 4
          i32.const 48
          i32.add
          i32.const 20
          i32.add
          i32.const 3
          i32.store
          local.get 4
          i32.const 72
          i32.add
          i32.const 20
          i32.add
          i32.const 54
          i32.store
          local.get 4
          i32.const 84
          i32.add
          i32.const 54
          i32.store
          local.get 4
          i64.const 3
          i64.store offset=52 align=4
          local.get 4
          i32.const 1051460
          i32.store offset=48
          local.get 4
          i32.const 53
          i32.store offset=76
          local.get 4
          local.get 4
          i32.const 72
          i32.add
          i32.store offset=64
          local.get 4
          local.get 4
          i32.const 24
          i32.add
          i32.store offset=88
          local.get 4
          local.get 4
          i32.const 16
          i32.add
          i32.store offset=80
          local.get 4
          local.get 4
          i32.const 40
          i32.add
          i32.store offset=72
          local.get 4
          i32.const 48
          i32.add
          i32.const 1051484
          call $core::panicking::panic_fmt::h095d4614168d6bd6
          unreachable
        end
        local.get 4
        i32.const 100
        i32.add
        i32.const 54
        i32.store
        local.get 4
        i32.const 72
        i32.add
        i32.const 20
        i32.add
        i32.const 54
        i32.store
        local.get 4
        i32.const 84
        i32.add
        i32.const 53
        i32.store
        local.get 4
        i32.const 48
        i32.add
        i32.const 20
        i32.add
        i32.const 4
        i32.store
        local.get 4
        i64.const 4
        i64.store offset=52 align=4
        local.get 4
        i32.const 1051536
        i32.store offset=48
        local.get 4
        i32.const 53
        i32.store offset=76
        local.get 4
        local.get 4
        i32.const 72
        i32.add
        i32.store offset=64
        local.get 4
        local.get 4
        i32.const 24
        i32.add
        i32.store offset=96
        local.get 4
        local.get 4
        i32.const 16
        i32.add
        i32.store offset=88
        local.get 4
        local.get 4
        i32.const 12
        i32.add
        i32.store offset=80
        local.get 4
        local.get 4
        i32.const 8
        i32.add
        i32.store offset=72
        local.get 4
        i32.const 48
        i32.add
        i32.const 1051568
        call $core::panicking::panic_fmt::h095d4614168d6bd6
        unreachable
      end
      local.get 2
      local.set 8
    end
    block  ;; label = @1
      local.get 8
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1
      local.set 6
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 8
              i32.add
              local.tee 9
              i32.load8_s
              local.tee 2
              i32.const -1
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              local.set 5
              local.get 0
              local.get 1
              i32.add
              local.tee 6
              local.set 1
              block  ;; label = @6
                local.get 9
                i32.const 1
                i32.add
                local.get 6
                i32.eq
                br_if 0 (;@6;)
                local.get 9
                i32.const 2
                i32.add
                local.set 1
                local.get 9
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 5
              end
              local.get 2
              i32.const 31
              i32.and
              local.set 9
              local.get 2
              i32.const 255
              i32.and
              i32.const 223
              i32.gt_u
              br_if 1 (;@4;)
              local.get 5
              local.get 9
              i32.const 6
              i32.shl
              i32.or
              local.set 1
              br 2 (;@3;)
            end
            local.get 4
            local.get 2
            i32.const 255
            i32.and
            i32.store offset=36
            local.get 4
            i32.const 40
            i32.add
            local.set 2
            br 2 (;@2;)
          end
          i32.const 0
          local.set 0
          local.get 6
          local.set 7
          block  ;; label = @4
            local.get 1
            local.get 6
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 1
            i32.add
            local.set 7
            local.get 1
            i32.load8_u
            i32.const 63
            i32.and
            local.set 0
          end
          local.get 0
          local.get 5
          i32.const 6
          i32.shl
          i32.or
          local.set 1
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.and
            i32.const 240
            i32.ge_u
            br_if 0 (;@4;)
            local.get 1
            local.get 9
            i32.const 12
            i32.shl
            i32.or
            local.set 1
            br 1 (;@3;)
          end
          i32.const 0
          local.set 2
          block  ;; label = @4
            local.get 7
            local.get 6
            i32.eq
            br_if 0 (;@4;)
            local.get 7
            i32.load8_u
            i32.const 63
            i32.and
            local.set 2
          end
          local.get 1
          i32.const 6
          i32.shl
          local.get 9
          i32.const 18
          i32.shl
          i32.const 1835008
          i32.and
          i32.or
          local.get 2
          i32.or
          local.tee 1
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
        end
        local.get 4
        local.get 1
        i32.store offset=36
        i32.const 1
        local.set 6
        local.get 4
        i32.const 40
        i32.add
        local.set 2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 6
        local.get 1
        i32.const 2048
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 3
        i32.const 4
        local.get 1
        i32.const 65536
        i32.lt_u
        select
        local.set 6
      end
      local.get 4
      local.get 8
      i32.store offset=40
      local.get 4
      local.get 6
      local.get 8
      i32.add
      i32.store offset=44
      local.get 4
      i32.const 48
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get 4
      i32.const 108
      i32.add
      i32.const 54
      i32.store
      local.get 4
      i32.const 100
      i32.add
      i32.const 54
      i32.store
      local.get 4
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 55
      i32.store
      local.get 4
      i32.const 84
      i32.add
      i32.const 56
      i32.store
      local.get 4
      i64.const 5
      i64.store offset=52 align=4
      local.get 4
      i32.const 1051636
      i32.store offset=48
      local.get 4
      local.get 2
      i32.store offset=88
      local.get 4
      i32.const 53
      i32.store offset=76
      local.get 4
      local.get 4
      i32.const 72
      i32.add
      i32.store offset=64
      local.get 4
      local.get 4
      i32.const 24
      i32.add
      i32.store offset=104
      local.get 4
      local.get 4
      i32.const 16
      i32.add
      i32.store offset=96
      local.get 4
      local.get 4
      i32.const 36
      i32.add
      i32.store offset=80
      local.get 4
      local.get 4
      i32.const 32
      i32.add
      i32.store offset=72
      local.get 4
      i32.const 48
      i32.add
      i32.const 1051676
      call $core::panicking::panic_fmt::h095d4614168d6bd6
      unreachable
    end
    i32.const 1051056
    call $core::panicking::panic::h0142ee7f4c64bd08
    unreachable)
  (func $core::fmt::num::imp::<impl_core::fmt::Display_for_u32>::fmt::h3518dbff2fc7fe22 (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $core::fmt::num::imp::fmt_u64::h6560fb621643a867)
  (func $core::fmt::write::hb137f2496e0ed1b6 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 52
    i32.add
    local.get 2
    i32.const 20
    i32.add
    i32.load
    local.tee 4
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=56
    local.get 3
    i32.const 44
    i32.add
    local.get 2
    i32.load offset=16
    local.tee 5
    local.get 4
    i32.const 3
    i32.shl
    i32.add
    i32.store
    local.get 3
    i64.const 137438953472
    i64.store offset=8
    local.get 3
    local.get 0
    i32.store offset=32
    i32.const 0
    local.set 6
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    local.get 5
    i32.store offset=48
    local.get 3
    local.get 5
    i32.store offset=40
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=8
              local.tee 7
              br_if 0 (;@5;)
              local.get 2
              i32.load
              local.set 8
              local.get 2
              i32.load offset=4
              local.tee 9
              local.get 4
              local.get 4
              local.get 9
              i32.gt_u
              select
              local.tee 10
              i32.eqz
              br_if 1 (;@4;)
              i32.const 1
              local.set 4
              local.get 0
              local.get 8
              i32.load
              local.get 8
              i32.load offset=4
              local.get 1
              i32.load offset=12
              call_indirect (type 6)
              br_if 4 (;@1;)
              local.get 8
              i32.const 12
              i32.add
              local.set 2
              i32.const 1
              local.set 6
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load
                  local.get 3
                  i32.const 8
                  i32.add
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  call_indirect (type 2)
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 1
                  local.set 4
                  br 6 (;@1;)
                end
                local.get 6
                local.get 10
                i32.ge_u
                br_if 2 (;@4;)
                local.get 2
                i32.const -4
                i32.add
                local.set 0
                local.get 2
                i32.load
                local.set 1
                local.get 2
                i32.const 8
                i32.add
                local.set 2
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                i32.const 1
                local.set 4
                local.get 6
                i32.const 1
                i32.add
                local.set 6
                local.get 3
                i32.load offset=32
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 6)
                i32.eqz
                br_if 0 (;@6;)
                br 5 (;@1;)
              end
            end
            local.get 2
            i32.load
            local.set 8
            local.get 2
            i32.load offset=4
            local.tee 9
            local.get 2
            i32.const 12
            i32.add
            i32.load
            local.tee 5
            local.get 5
            local.get 9
            i32.gt_u
            select
            local.tee 10
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            local.get 0
            local.get 8
            i32.load
            local.get 8
            i32.load offset=4
            local.get 1
            i32.load offset=12
            call_indirect (type 6)
            br_if 3 (;@1;)
            local.get 8
            i32.const 12
            i32.add
            local.set 2
            local.get 7
            i32.const 16
            i32.add
            local.set 5
            i32.const 1
            local.set 6
            loop  ;; label = @5
              local.get 3
              local.get 5
              i32.const -8
              i32.add
              i32.load
              i32.store offset=12
              local.get 3
              local.get 5
              i32.const 16
              i32.add
              i32.load8_u
              i32.store8 offset=56
              local.get 3
              local.get 5
              i32.const -4
              i32.add
              i32.load
              i32.store offset=8
              i32.const 0
              local.set 1
              i32.const 0
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 5
                      i32.const 8
                      i32.add
                      i32.load
                      br_table 0 (;@9;) 1 (;@8;) 2 (;@7;) 3 (;@6;) 0 (;@9;)
                    end
                    local.get 5
                    i32.const 12
                    i32.add
                    i32.load
                    local.set 0
                    i32.const 1
                    local.set 4
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    local.get 5
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee 7
                    local.get 3
                    i32.load offset=52
                    local.tee 4
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 4
                    local.get 3
                    i32.load offset=48
                    local.get 7
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.load offset=4
                    i32.const 57
                    i32.ne
                    br_if 2 (;@6;)
                    local.get 7
                    i32.load
                    i32.load
                    local.set 0
                    i32.const 1
                    local.set 4
                    br 2 (;@6;)
                  end
                  i32.const 1052008
                  local.get 7
                  local.get 4
                  call $core::panicking::panic_bounds_check::h1fae5a314994f748
                  unreachable
                end
                i32.const 0
                local.set 4
                local.get 3
                i32.load offset=40
                local.tee 7
                local.get 3
                i32.load offset=44
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                local.get 7
                i32.const 8
                i32.add
                i32.store offset=40
                i32.const 0
                local.set 4
                local.get 7
                i32.load offset=4
                i32.const 57
                i32.ne
                br_if 0 (;@6;)
                local.get 7
                i32.load
                i32.load
                local.set 0
                i32.const 1
                local.set 4
              end
              local.get 3
              local.get 0
              i32.store offset=20
              local.get 3
              local.get 4
              i32.store offset=16
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 5
                            i32.load
                            br_table 4 (;@8;) 1 (;@11;) 0 (;@12;) 6 (;@6;) 4 (;@8;)
                          end
                          local.get 3
                          i32.load offset=40
                          local.tee 0
                          local.get 3
                          i32.load offset=44
                          i32.ne
                          br_if 1 (;@10;)
                          br 5 (;@6;)
                        end
                        local.get 5
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 0
                        local.get 3
                        i32.load offset=52
                        local.tee 4
                        i32.ge_u
                        br_if 1 (;@9;)
                        local.get 3
                        i32.load offset=48
                        local.get 0
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 0
                        i32.load offset=4
                        i32.const 57
                        i32.ne
                        br_if 4 (;@6;)
                        local.get 0
                        i32.load
                        i32.load
                        local.set 4
                        br 3 (;@7;)
                      end
                      local.get 3
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.store offset=40
                      local.get 0
                      i32.load offset=4
                      i32.const 57
                      i32.ne
                      br_if 3 (;@6;)
                      local.get 0
                      i32.load
                      i32.load
                      local.set 4
                      br 2 (;@7;)
                    end
                    i32.const 1052008
                    local.get 0
                    local.get 4
                    call $core::panicking::panic_bounds_check::h1fae5a314994f748
                    unreachable
                  end
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  local.set 4
                end
                i32.const 1
                local.set 1
              end
              local.get 3
              local.get 4
              i32.store offset=28
              local.get 3
              local.get 1
              i32.store offset=24
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.const -16
                  i32.add
                  i32.load
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=40
                  local.tee 4
                  local.get 3
                  i32.load offset=44
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 3
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.store offset=40
                  br 1 (;@6;)
                end
                local.get 5
                i32.const -12
                i32.add
                i32.load
                local.tee 4
                local.get 3
                i32.load offset=52
                local.tee 0
                i32.ge_u
                br_if 4 (;@2;)
                local.get 3
                i32.load offset=48
                local.get 4
                i32.const 3
                i32.shl
                i32.add
                local.set 4
              end
              block  ;; label = @6
                local.get 4
                i32.load
                local.get 3
                i32.const 8
                i32.add
                local.get 4
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 2)
                i32.eqz
                br_if 0 (;@6;)
                i32.const 1
                local.set 4
                br 5 (;@1;)
              end
              local.get 6
              local.get 10
              i32.ge_u
              br_if 1 (;@4;)
              local.get 2
              i32.const -4
              i32.add
              local.set 0
              local.get 2
              i32.load
              local.set 1
              local.get 2
              i32.const 8
              i32.add
              local.set 2
              local.get 5
              i32.const 36
              i32.add
              local.set 5
              i32.const 1
              local.set 4
              local.get 6
              i32.const 1
              i32.add
              local.set 6
              local.get 3
              i32.load offset=32
              local.get 0
              i32.load
              local.get 1
              local.get 3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type 6)
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          block  ;; label = @4
            local.get 9
            local.get 6
            i32.le_u
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            local.get 3
            i32.load offset=32
            local.get 8
            local.get 6
            i32.const 3
            i32.shl
            i32.add
            local.tee 5
            i32.load
            local.get 5
            i32.load offset=4
            local.get 3
            i32.load offset=36
            i32.load offset=12
            call_indirect (type 6)
            br_if 3 (;@1;)
          end
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        i32.const 1051056
        call $core::panicking::panic::h0142ee7f4c64bd08
        unreachable
      end
      i32.const 1051992
      local.get 4
      local.get 0
      call $core::panicking::panic_bounds_check::h1fae5a314994f748
      unreachable
    end
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 4)
  (func $<core::ops::range::Range<Idx>_as_core::fmt::Debug>::fmt::h7eaf6892c126f203 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 0
      local.get 1
      call $core::fmt::num::<impl_core::fmt::Debug_for_usize>::fmt::h3b488599f5faa9c0
      br_if 0 (;@1;)
      local.get 1
      i32.const 28
      i32.add
      i32.load
      local.set 3
      local.get 1
      i32.load offset=24
      local.set 4
      local.get 2
      i64.const 4
      i64.store offset=24
      local.get 2
      i64.const 1
      i64.store offset=12 align=4
      local.get 2
      i32.const 1050872
      i32.store offset=8
      local.get 4
      local.get 3
      local.get 2
      i32.const 8
      i32.add
      call $core::fmt::write::hb137f2496e0ed1b6
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      local.get 1
      call $core::fmt::num::<impl_core::fmt::Debug_for_usize>::fmt::h3b488599f5faa9c0
      local.set 1
      local.get 2
      i32.const 32
      i32.add
      global.set 0
      local.get 1
      return
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    i32.const 1)
  (func $core::fmt::num::<impl_core::fmt::Debug_for_usize>::fmt::h3b488599f5faa9c0 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 0
              i32.load
              local.set 4
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 4
              i64.extend_i32_u
              i32.const 1
              local.get 1
              call $core::fmt::num::imp::fmt_u64::h6560fb621643a867
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 4
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 127
              i32.add
              local.get 4
              i32.const 15
              i32.and
              local.tee 3
              i32.const 48
              i32.or
              local.get 3
              i32.const 87
              i32.add
              local.get 3
              i32.const 10
              i32.lt_u
              select
              i32.store8
              local.get 0
              i32.const -1
              i32.add
              local.set 0
              local.get 4
              i32.const 4
              i32.shr_u
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 0
            i32.const 128
            i32.add
            local.tee 4
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1051692
            i32.const 2
            local.get 2
            local.get 0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 0
            i32.sub
            call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
            local.set 0
            br 1 (;@3;)
          end
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 2
            local.get 0
            i32.add
            i32.const 127
            i32.add
            local.get 4
            i32.const 15
            i32.and
            local.tee 3
            i32.const 48
            i32.or
            local.get 3
            i32.const 55
            i32.add
            local.get 3
            i32.const 10
            i32.lt_u
            select
            i32.store8
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 4
            i32.const 4
            i32.shr_u
            local.tee 4
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 128
          i32.add
          local.tee 4
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1051692
          i32.const 2
          local.get 2
          local.get 0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 0
          i32.sub
          call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set 0
        local.get 0
        return
      end
      local.get 4
      i32.const 128
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 4
    i32.const 128
    call $core::slice::slice_index_order_fail::h45638c641c9b3b30
    unreachable)
  (func $<T_as_core::any::Any>::type_id::h40a48bfc40f5283f (type 10) (param i32) (result i64)
    i64.const 6308721582299515157)
  (func $<core::cell::BorrowError_as_core::fmt::Debug>::fmt::hcf9a3603fd81bdae (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 1050880
    i32.const 11
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 6))
  (func $<core::cell::BorrowMutError_as_core::fmt::Debug>::fmt::h41a576614c12f53d (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 1050891
    i32.const 14
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 6))
  (func $core::panic::PanicInfo::message::hc730610bb8056e74 (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=8)
  (func $core::panic::PanicInfo::location::hbc5e44a64eaf706a (type 5) (param i32) (result i32)
    local.get 0
    i32.const 12
    i32.add)
  (func $<&T_as_core::fmt::Display>::fmt::hbdb54b8c793ef0af (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $core::fmt::Formatter::pad::hd367b6bcbe89f492)
  (func $core::panic::Location::internal_constructor::hcf293bdd1161e916 (type 7) (param i32 i32 i32 i32 i32)
    local.get 0
    local.get 4
    i32.store offset=12
    local.get 0
    local.get 3
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $core::panic::Location::file::hfbb9014eea889c61 (type 4) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store align=4)
  (func $core::panic::Location::line::h75a85319172d348e (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=8)
  (func $core::panic::Location::column::h4bc83a66cb1b6958 (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=12)
  (func $core::option::expect_failed::h38f732006dee06b2 (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 36
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i64.const 1
    i64.store offset=20 align=4
    local.get 2
    i32.const 1051080
    i32.store offset=16
    local.get 2
    i32.const 54
    i32.store offset=44
    local.get 2
    local.get 2
    i32.const 40
    i32.add
    i32.store offset=32
    local.get 2
    local.get 2
    i32.const 8
    i32.add
    i32.store offset=40
    local.get 2
    i32.const 16
    i32.add
    i32.const 1051088
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $<&T_as_core::fmt::Debug>::fmt::h33f1a352ef5b670f (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2))
  (func $core::result::unwrap_failed::hd11b409f5ba7889e (type 11) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 1
    i32.store offset=12
    local.get 4
    local.get 0
    i32.store offset=8
    local.get 4
    local.get 3
    i32.store offset=20
    local.get 4
    local.get 2
    i32.store offset=16
    local.get 4
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 4
    i32.const 60
    i32.add
    i32.const 58
    i32.store
    local.get 4
    i64.const 2
    i64.store offset=28 align=4
    local.get 4
    i32.const 1051108
    i32.store offset=24
    local.get 4
    i32.const 54
    i32.store offset=52
    local.get 4
    local.get 4
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 4
    local.get 4
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 4
    local.get 4
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 4
    i32.const 24
    i32.add
    i32.const 1051148
    call $core::panicking::panic_fmt::h095d4614168d6bd6
    unreachable)
  (func $<core::ffi::VaListImpl_as_core::ops::drop::Drop>::drop::ha1b90773b9afe6ad (type 1) (param i32))
  (func $core::slice::memchr::memchr::h2031004febcdc2f2 (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 3
        i32.and
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        i32.const 4
        local.get 5
        i32.sub
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        local.get 5
        local.get 5
        local.get 3
        i32.gt_u
        select
        local.tee 4
        i32.add
        local.set 6
        i32.const 0
        local.set 5
        local.get 1
        i32.const 255
        i32.and
        local.set 7
        local.get 4
        local.set 8
        local.get 2
        local.set 9
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 9
              i32.sub
              i32.const 3
              i32.gt_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 7
              local.get 1
              i32.const 255
              i32.and
              local.set 6
              loop  ;; label = @6
                local.get 8
                i32.eqz
                br_if 4 (;@2;)
                local.get 9
                local.get 7
                i32.add
                local.set 10
                local.get 8
                i32.const -1
                i32.add
                local.set 8
                local.get 7
                i32.const 1
                i32.add
                local.set 7
                local.get 10
                i32.load8_u
                local.tee 10
                local.get 6
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 5
              local.get 10
              local.get 1
              i32.const 255
              i32.and
              i32.eq
              i32.const 1
              i32.add
              i32.const 1
              i32.and
              i32.add
              local.get 7
              i32.add
              i32.const -1
              i32.add
              local.set 5
              br 2 (;@3;)
            end
            local.get 5
            local.get 9
            i32.load8_u
            local.tee 10
            local.get 7
            i32.ne
            i32.add
            local.set 5
            local.get 10
            local.get 7
            i32.eq
            br_if 1 (;@3;)
            local.get 5
            local.get 9
            i32.const 1
            i32.add
            i32.load8_u
            local.tee 10
            local.get 7
            i32.ne
            i32.add
            local.set 5
            local.get 10
            local.get 7
            i32.eq
            br_if 1 (;@3;)
            local.get 5
            local.get 9
            i32.const 2
            i32.add
            i32.load8_u
            local.tee 10
            local.get 7
            i32.ne
            i32.add
            local.set 5
            local.get 10
            local.get 7
            i32.eq
            br_if 1 (;@3;)
            local.get 5
            local.get 9
            i32.const 3
            i32.add
            i32.load8_u
            local.tee 10
            local.get 7
            i32.ne
            i32.add
            local.set 5
            local.get 8
            i32.const -4
            i32.add
            local.set 8
            local.get 9
            i32.const 4
            i32.add
            local.set 9
            local.get 10
            local.get 7
            i32.ne
            br_if 0 (;@4;)
          end
        end
        i32.const 1
        local.set 9
        br 1 (;@1;)
      end
      local.get 1
      i32.const 255
      i32.and
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 3
          i32.const -8
          i32.add
          local.tee 10
          i32.gt_u
          br_if 0 (;@3;)
          local.get 7
          i32.const 16843009
          i32.mul
          local.set 5
          block  ;; label = @4
            loop  ;; label = @5
              local.get 2
              local.get 4
              i32.add
              local.tee 9
              i32.const 4
              i32.add
              i32.load
              local.get 5
              i32.xor
              local.tee 8
              i32.const -1
              i32.xor
              local.get 8
              i32.const -16843009
              i32.add
              i32.and
              local.get 9
              i32.load
              local.get 5
              i32.xor
              local.tee 9
              i32.const -1
              i32.xor
              local.get 9
              i32.const -16843009
              i32.add
              i32.and
              i32.or
              i32.const -2139062144
              i32.and
              br_if 1 (;@4;)
              local.get 4
              i32.const 8
              i32.add
              local.tee 4
              local.get 10
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 4
          local.get 3
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 4
        i32.add
        local.set 9
        local.get 2
        local.get 3
        i32.add
        local.set 2
        local.get 3
        local.get 4
        i32.sub
        local.set 8
        i32.const 0
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            loop  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 9
                i32.sub
                i32.const 3
                i32.gt_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                local.get 1
                i32.const 255
                i32.and
                local.set 2
                loop  ;; label = @7
                  local.get 8
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 9
                  local.get 7
                  i32.add
                  local.set 10
                  local.get 8
                  i32.const -1
                  i32.add
                  local.set 8
                  local.get 7
                  i32.const 1
                  i32.add
                  local.set 7
                  local.get 10
                  i32.load8_u
                  local.tee 10
                  local.get 2
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 10
                local.get 1
                i32.const 255
                i32.and
                i32.eq
                i32.const 1
                i32.add
                i32.const 1
                i32.and
                local.get 5
                i32.add
                local.get 7
                i32.add
                i32.const -1
                i32.add
                local.set 5
                br 2 (;@4;)
              end
              local.get 5
              local.get 9
              i32.load8_u
              local.tee 10
              local.get 7
              i32.ne
              i32.add
              local.set 5
              local.get 10
              local.get 7
              i32.eq
              br_if 1 (;@4;)
              local.get 5
              local.get 9
              i32.const 1
              i32.add
              i32.load8_u
              local.tee 10
              local.get 7
              i32.ne
              i32.add
              local.set 5
              local.get 10
              local.get 7
              i32.eq
              br_if 1 (;@4;)
              local.get 5
              local.get 9
              i32.const 2
              i32.add
              i32.load8_u
              local.tee 10
              local.get 7
              i32.ne
              i32.add
              local.set 5
              local.get 10
              local.get 7
              i32.eq
              br_if 1 (;@4;)
              local.get 5
              local.get 9
              i32.const 3
              i32.add
              i32.load8_u
              local.tee 10
              local.get 7
              i32.ne
              i32.add
              local.set 5
              local.get 8
              i32.const -4
              i32.add
              local.set 8
              local.get 9
              i32.const 4
              i32.add
              local.set 9
              local.get 10
              local.get 7
              i32.ne
              br_if 0 (;@5;)
            end
          end
          i32.const 1
          local.set 9
          local.get 5
          local.get 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        i32.const 0
        local.set 9
        local.get 5
        local.get 7
        i32.add
        local.get 4
        i32.add
        local.set 5
        br 1 (;@1;)
      end
      local.get 4
      local.get 3
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 9
    i32.store)
  (func $core::slice::memchr::memrchr::hd17908245138f97b (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    local.get 3
    i32.const 4
    local.get 2
    i32.const 3
    i32.and
    local.tee 5
    i32.sub
    i32.const 0
    local.get 5
    select
    local.tee 5
    i32.sub
    i32.const 7
    i32.and
    local.get 3
    local.get 5
    i32.lt_u
    local.tee 6
    select
    local.tee 7
    i32.sub
    local.set 8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      local.get 7
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 3
                      local.get 5
                      local.get 6
                      select
                      local.set 9
                      local.get 2
                      local.get 8
                      i32.add
                      local.set 10
                      local.get 1
                      i32.const 255
                      i32.and
                      local.set 5
                      local.get 2
                      local.get 3
                      i32.add
                      local.tee 11
                      local.set 6
                      block  ;; label = @10
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 6
                            local.get 10
                            i32.sub
                            i32.const 3
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 7
                            local.get 4
                            i32.add
                            local.set 5
                            local.get 11
                            local.get 4
                            i32.add
                            local.set 12
                            i32.const 0
                            local.set 6
                            local.get 1
                            i32.const 255
                            i32.and
                            local.set 13
                            loop  ;; label = @13
                              local.get 5
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 12
                              local.get 6
                              i32.add
                              local.set 7
                              local.get 5
                              i32.const -1
                              i32.add
                              local.set 5
                              local.get 6
                              i32.const -1
                              i32.add
                              local.set 6
                              local.get 7
                              i32.const -1
                              i32.add
                              i32.load8_u
                              local.get 13
                              i32.ne
                              br_if 0 (;@13;)
                            end
                            local.get 11
                            local.get 10
                            i32.sub
                            local.get 4
                            i32.add
                            local.get 6
                            i32.add
                            local.set 5
                            br 10 (;@2;)
                          end
                          block  ;; label = @12
                            local.get 11
                            local.get 4
                            i32.add
                            local.tee 6
                            i32.const -1
                            i32.add
                            i32.load8_u
                            local.get 5
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 11
                            local.get 10
                            i32.sub
                            local.get 4
                            i32.add
                            i32.const -1
                            i32.add
                            local.set 5
                            br 10 (;@2;)
                          end
                          local.get 6
                          i32.const -2
                          i32.add
                          i32.load8_u
                          local.get 5
                          i32.eq
                          br_if 8 (;@3;)
                          local.get 6
                          i32.const -3
                          i32.add
                          i32.load8_u
                          local.get 5
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 4
                          i32.const -4
                          i32.add
                          local.set 4
                          local.get 6
                          i32.const -4
                          i32.add
                          local.tee 6
                          i32.load8_u
                          local.get 5
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 11
                        local.get 10
                        i32.sub
                        local.get 4
                        i32.add
                        local.set 5
                        br 8 (;@2;)
                      end
                      local.get 1
                      i32.const 255
                      i32.and
                      i32.const 16843009
                      i32.mul
                      local.set 6
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 8
                          local.tee 5
                          local.get 9
                          i32.le_u
                          br_if 1 (;@10;)
                          local.get 5
                          i32.const -8
                          i32.add
                          local.set 8
                          local.get 2
                          local.get 5
                          i32.add
                          local.tee 4
                          i32.const -4
                          i32.add
                          i32.load
                          local.get 6
                          i32.xor
                          local.tee 10
                          i32.const -1
                          i32.xor
                          local.get 10
                          i32.const -16843009
                          i32.add
                          i32.and
                          local.get 4
                          i32.const -8
                          i32.add
                          i32.load
                          local.get 6
                          i32.xor
                          local.tee 4
                          i32.const -1
                          i32.xor
                          local.get 4
                          i32.const -16843009
                          i32.add
                          i32.and
                          i32.or
                          i32.const -2139062144
                          i32.and
                          i32.eqz
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 5
                      local.get 3
                      i32.gt_u
                      br_if 1 (;@8;)
                      local.get 1
                      i32.const 255
                      i32.and
                      local.set 6
                      loop  ;; label = @10
                        local.get 2
                        local.get 5
                        i32.add
                        local.set 4
                        block  ;; label = @11
                          local.get 5
                          i32.const 3
                          i32.gt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 6
                          local.get 1
                          i32.const 255
                          i32.and
                          local.set 11
                          loop  ;; label = @12
                            local.get 5
                            local.get 6
                            i32.add
                            local.tee 10
                            i32.eqz
                            br_if 7 (;@5;)
                            local.get 4
                            local.get 6
                            i32.add
                            local.set 10
                            local.get 6
                            i32.const -1
                            i32.add
                            local.set 6
                            local.get 10
                            i32.const -1
                            i32.add
                            i32.load8_u
                            local.get 11
                            i32.ne
                            br_if 0 (;@12;)
                          end
                          local.get 5
                          local.get 6
                          i32.add
                          local.set 5
                          i32.const 1
                          local.set 6
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          local.get 4
                          i32.const -1
                          i32.add
                          local.tee 4
                          i32.load8_u
                          local.get 6
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const -1
                          i32.add
                          local.set 5
                          i32.const 1
                          local.set 6
                          br 10 (;@1;)
                        end
                        local.get 4
                        i32.const -1
                        i32.add
                        local.tee 4
                        i32.load8_u
                        local.get 6
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 4
                        i32.const -1
                        i32.add
                        local.tee 4
                        i32.load8_u
                        local.get 6
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 5
                        i32.const -4
                        i32.add
                        local.set 5
                        local.get 4
                        i32.const -1
                        i32.add
                        i32.load8_u
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      i32.const 1
                      local.set 6
                      br 8 (;@1;)
                    end
                    local.get 8
                    local.get 3
                    call $core::slice::slice_index_order_fail::h45638c641c9b3b30
                    unreachable
                  end
                  local.get 5
                  local.get 3
                  call $core::slice::slice_index_len_fail::h08f636efd7156c0a
                  unreachable
                end
                local.get 5
                i32.const -3
                i32.add
                local.set 5
                i32.const 1
                local.set 6
                br 5 (;@1;)
              end
              local.get 5
              i32.const -2
              i32.add
              local.set 5
              i32.const 1
              local.set 6
              br 4 (;@1;)
            end
            i32.const 0
            local.set 6
            local.get 10
            local.set 5
            br 3 (;@1;)
          end
          local.get 11
          local.get 10
          i32.sub
          local.get 4
          i32.add
          i32.const -3
          i32.add
          local.set 5
          br 1 (;@2;)
        end
        local.get 11
        local.get 10
        i32.sub
        local.get 4
        i32.add
        i32.const -2
        i32.add
        local.set 5
      end
      local.get 5
      local.get 8
      i32.add
      local.set 5
      i32.const 1
      local.set 6
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 6
    i32.store)
  (func $core::slice::slice_index_overflow_fail::h1d032eb7c15bac81 (type 0)
    i32.const 1051372
    call $core::panicking::panic::h0142ee7f4c64bd08
    unreachable)
  (func $core::unicode::bool_trie::BoolTrie::lookup::h5985ded232b92c4f (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 2048
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const 65536
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 12
                    i32.shr_u
                    i32.const -16
                    i32.add
                    local.tee 2
                    i32.const 256
                    i32.lt_u
                    br_if 1 (;@7;)
                    i32.const 1052096
                    local.get 2
                    i32.const 256
                    call $core::panicking::panic_bounds_check::h1fae5a314994f748
                    unreachable
                  end
                  local.get 1
                  i32.const 6
                  i32.shr_u
                  i32.const -32
                  i32.add
                  local.tee 2
                  i32.const 991
                  i32.gt_u
                  br_if 1 (;@6;)
                  local.get 0
                  i32.const 260
                  i32.add
                  i32.load
                  local.tee 3
                  local.get 0
                  local.get 2
                  i32.add
                  i32.const 280
                  i32.add
                  i32.load8_u
                  local.tee 2
                  i32.le_u
                  br_if 2 (;@5;)
                  local.get 0
                  i32.load offset=256
                  local.get 2
                  i32.const 3
                  i32.shl
                  i32.add
                  local.set 0
                  br 6 (;@1;)
                end
                local.get 0
                local.get 2
                i32.add
                i32.const 1272
                i32.add
                i32.load8_u
                i32.const 6
                i32.shl
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.or
                local.tee 2
                local.get 0
                i32.const 268
                i32.add
                i32.load
                local.tee 3
                i32.ge_u
                br_if 2 (;@4;)
                local.get 0
                i32.const 276
                i32.add
                i32.load
                local.tee 3
                local.get 0
                i32.load offset=264
                local.get 2
                i32.add
                i32.load8_u
                local.tee 2
                i32.le_u
                br_if 3 (;@3;)
                local.get 0
                i32.load offset=272
                local.get 2
                i32.const 3
                i32.shl
                i32.add
                local.set 0
                br 5 (;@1;)
              end
              i32.const 1052064
              local.get 2
              i32.const 992
              call $core::panicking::panic_bounds_check::h1fae5a314994f748
              unreachable
            end
            i32.const 1052080
            local.get 2
            local.get 3
            call $core::panicking::panic_bounds_check::h1fae5a314994f748
            unreachable
          end
          i32.const 1052112
          local.get 2
          local.get 3
          call $core::panicking::panic_bounds_check::h1fae5a314994f748
          unreachable
        end
        i32.const 1052128
        local.get 2
        local.get 3
        call $core::panicking::panic_bounds_check::h1fae5a314994f748
        unreachable
      end
      local.get 0
      local.get 1
      i32.const 3
      i32.shr_u
      i32.const 536870904
      i32.and
      i32.add
      local.set 0
    end
    local.get 0
    i64.load
    i64.const 1
    local.get 1
    i32.const 63
    i32.and
    i64.extend_i32_u
    i64.shl
    i64.and
    i64.const 0
    i64.ne)
  (func $core::unicode::printable::is_printable::haacf9edc45c1c4bf (type 5) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 131072
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          local.get 0
          i32.const -195102
          i32.add
          i32.const 722658
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -191457
          i32.add
          i32.const 3103
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -183970
          i32.add
          i32.const 14
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const 2097150
          i32.and
          i32.const 178206
          i32.eq
          br_if 1 (;@2;)
          local.get 0
          i32.const -173783
          i32.add
          i32.const 41
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -177973
          i32.add
          i32.const 11
          i32.lt_u
          br_if 1 (;@2;)
          local.get 0
          i32.const -918000
          i32.add
          i32.const 196111
          i32.gt_u
          return
        end
        local.get 0
        i32.const 1052833
        i32.const 35
        i32.const 1052903
        i32.const 166
        i32.const 1053069
        i32.const 408
        call $core::unicode::printable::check::hf6373bfc83e92c23
        local.set 1
      end
      local.get 1
      return
    end
    local.get 0
    i32.const 1052144
    i32.const 41
    i32.const 1052226
    i32.const 293
    i32.const 1052519
    i32.const 314
    call $core::unicode::printable::check::hf6373bfc83e92c23)
  (func $core::str::traits::<impl_core::slice::SliceIndex<str>_for_core::ops::range::Range<usize>>::index::__closure__::hdba06e33f13630a5 (type 1) (param i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load offset=4
    i32.load
    local.get 0
    i32.load offset=8
    i32.load
    call $core::str::slice_error_fail::h571f7e6f7dc53361
    unreachable)
  (func $core::fmt::num::<impl_core::fmt::LowerHex_for_i8>::fmt::he8f810381047cecd (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 87
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1051692
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $core::fmt::num::imp::<impl_core::fmt::Display_for_u8>::fmt::he44c0f717551868b (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load8_u
    i32.const 1
    local.get 1
    call $core::fmt::num::imp::fmt_u64::h6560fb621643a867)
  (func $core::fmt::builders::DebugInner::entry::h8c1ee5224b40cd89 (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i64 i64 i64 i64 i64)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 5
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          local.get 5
          i32.load offset=24
          i32.const 1051926
          i32.const 2
          local.get 5
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 6)
          br_if 2 (;@1;)
          local.get 0
          i32.load
          local.set 5
        end
        local.get 1
        local.get 5
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 5
        i32.load offset=24
        i32.const 1051932
        i32.const 1
        local.get 5
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.set 5
      end
      i32.const 1
      local.set 4
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 5
      i64.load offset=8 align=4
      local.set 6
      local.get 5
      i64.load offset=16 align=4
      local.set 7
      local.get 3
      i32.const 52
      i32.add
      i32.const 1051896
      i32.store
      local.get 3
      local.get 5
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 5
      i64.load offset=32 align=4
      local.set 8
      local.get 5
      i64.load offset=40 align=4
      local.set 9
      local.get 3
      local.get 5
      i32.load8_u offset=48
      i32.store8 offset=72
      local.get 5
      i64.load align=4
      local.set 10
      local.get 3
      local.get 9
      i64.store offset=64
      local.get 3
      local.get 8
      i64.store offset=56
      local.get 3
      local.get 7
      i64.store offset=40
      local.get 3
      local.get 6
      i64.store offset=32
      local.get 3
      local.get 10
      i64.store offset=24
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 1051924
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 6)
      local.set 4
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 4
    i32.store8 offset=4
    local.get 3
    i32.const 80
    i32.add
    global.set 0)
  (func $<char_as_core::fmt::Debug>::fmt::h50a7482d13f3c4e4 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i64)
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      i32.const 39
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type 2)
      br_if 0 (;@1;)
      i32.const 2
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 0
                i32.const -9
                i32.add
                local.tee 4
                i32.const 30
                i32.le_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 92
                i32.ne
                br_if 1 (;@5;)
                br 2 (;@4;)
              end
              i32.const 116
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  br_table 5 (;@2;) 1 (;@6;) 2 (;@5;) 2 (;@5;) 0 (;@7;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 3 (;@4;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 3 (;@4;) 5 (;@2;)
                end
                i32.const 114
                local.set 5
                br 4 (;@2;)
              end
              i32.const 110
              local.set 5
              br 3 (;@2;)
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 1055048
                  local.get 0
                  call $core::unicode::bool_trie::BoolTrie::lookup::h5985ded232b92c4f
                  br_if 0 (;@7;)
                  local.get 0
                  call $core::unicode::printable::is_printable::haacf9edc45c1c4bf
                  i32.eqz
                  br_if 1 (;@6;)
                  i32.const 1
                  local.set 3
                  br 4 (;@3;)
                end
                local.get 0
                i32.const 1
                i32.or
                i32.clz
                i32.const 2
                i32.shr_u
                i32.const 7
                i32.xor
                i64.extend_i32_u
                i64.const 21474836480
                i64.or
                local.set 6
                br 1 (;@5;)
              end
              local.get 0
              i32.const 1
              i32.or
              i32.clz
              i32.const 2
              i32.shr_u
              i32.const 7
              i32.xor
              i64.extend_i32_u
              i64.const 21474836480
              i64.or
              local.set 6
            end
            i32.const 3
            local.set 3
            br 1 (;@3;)
          end
        end
        local.get 0
        local.set 5
      end
      loop  ;; label = @2
        local.get 3
        local.set 4
        i32.const 92
        local.set 0
        i32.const 1
        local.set 2
        i32.const 1
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                br_table 1 (;@5;) 2 (;@4;) 3 (;@3;) 0 (;@6;) 1 (;@5;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 6
                        i64.const 32
                        i64.shr_u
                        i32.wrap_i64
                        i32.const 255
                        i32.and
                        br_table 5 (;@5;) 4 (;@6;) 3 (;@7;) 2 (;@8;) 1 (;@9;) 0 (;@10;) 5 (;@5;)
                      end
                      local.get 6
                      i64.const -1095216660481
                      i64.and
                      i64.const 17179869184
                      i64.or
                      local.set 6
                      i32.const 3
                      local.set 3
                      br 6 (;@3;)
                    end
                    local.get 6
                    i64.const -1095216660481
                    i64.and
                    i64.const 12884901888
                    i64.or
                    local.set 6
                    i32.const 117
                    local.set 0
                    i32.const 3
                    local.set 3
                    br 5 (;@3;)
                  end
                  local.get 6
                  i64.const -1095216660481
                  i64.and
                  i64.const 8589934592
                  i64.or
                  local.set 6
                  i32.const 123
                  local.set 0
                  i32.const 3
                  local.set 3
                  br 4 (;@3;)
                end
                local.get 5
                local.get 6
                i32.wrap_i64
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 28
                i32.and
                i32.shr_u
                i32.const 15
                i32.and
                local.tee 3
                i32.const 48
                i32.or
                local.get 3
                i32.const 87
                i32.add
                local.get 3
                i32.const 10
                i32.lt_u
                select
                local.set 0
                block  ;; label = @7
                  local.get 4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 6
                  i64.const -1
                  i64.add
                  i64.const 4294967295
                  i64.and
                  local.get 6
                  i64.const -4294967296
                  i64.and
                  i64.or
                  local.set 6
                  i32.const 3
                  local.set 3
                  br 4 (;@3;)
                end
                local.get 6
                i64.const -1095216660481
                i64.and
                i64.const 4294967296
                i64.or
                local.set 6
                i32.const 3
                local.set 3
                br 3 (;@3;)
              end
              local.get 6
              i64.const -1095216660481
              i64.and
              local.set 6
              i32.const 125
              local.set 0
              i32.const 3
              local.set 3
              br 2 (;@3;)
            end
            local.get 1
            i32.load offset=24
            i32.const 39
            local.get 1
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 2)
            return
          end
          i32.const 0
          local.set 3
          local.get 5
          local.set 0
        end
        local.get 1
        i32.load offset=24
        local.get 0
        local.get 1
        i32.load offset=28
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 2)
  (func $<core::fmt::builders::PadAdapter_as_core::fmt::Write>::write_str::h1176366f9b2ebdfc (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const 40
      i32.add
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=8
                  i32.load8_u
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load
                  i32.const 1051920
                  i32.const 4
                  local.get 0
                  i32.load offset=4
                  i32.load offset=12
                  call_indirect (type 6)
                  br_if 5 (;@2;)
                end
                local.get 3
                i32.const 10
                i32.store offset=40
                local.get 3
                i64.const 4294967306
                i64.store offset=32
                local.get 3
                local.get 2
                i32.store offset=28
                local.get 3
                i32.const 0
                i32.store offset=24
                local.get 3
                local.get 2
                i32.store offset=20
                local.get 3
                local.get 1
                i32.store offset=16
                local.get 3
                i32.const 8
                i32.add
                i32.const 10
                local.get 1
                local.get 2
                call $core::slice::memchr::memchr::h2031004febcdc2f2
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        i32.load offset=8
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.load offset=12
                        local.set 4
                        loop  ;; label = @11
                          local.get 3
                          local.get 4
                          local.get 3
                          i32.load offset=24
                          i32.add
                          i32.const 1
                          i32.add
                          local.tee 4
                          i32.store offset=24
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              local.get 3
                              i32.load offset=36
                              local.tee 6
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 3
                              i32.load offset=20
                              local.set 7
                              br 1 (;@12;)
                            end
                            local.get 3
                            i32.load offset=20
                            local.tee 7
                            local.get 4
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 5
                            i32.ge_u
                            br_if 7 (;@5;)
                            local.get 3
                            i32.load offset=16
                            local.get 4
                            local.get 6
                            i32.sub
                            local.tee 8
                            i32.add
                            local.tee 9
                            local.get 5
                            i32.eq
                            br_if 4 (;@8;)
                            local.get 9
                            local.get 5
                            local.get 6
                            call $memcmp
                            i32.eqz
                            br_if 4 (;@8;)
                          end
                          local.get 3
                          i32.load offset=28
                          local.tee 9
                          local.get 4
                          i32.lt_u
                          br_if 2 (;@9;)
                          local.get 7
                          local.get 9
                          i32.lt_u
                          br_if 2 (;@9;)
                          local.get 3
                          local.get 6
                          local.get 3
                          i32.const 16
                          i32.add
                          i32.add
                          i32.const 23
                          i32.add
                          i32.load8_u
                          local.get 3
                          i32.load offset=16
                          local.get 4
                          i32.add
                          local.get 9
                          local.get 4
                          i32.sub
                          call $core::slice::memchr::memchr::h2031004febcdc2f2
                          local.get 3
                          i32.load offset=4
                          local.set 4
                          local.get 3
                          i32.load
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 3
                      local.get 3
                      i32.load offset=28
                      i32.store offset=24
                    end
                    local.get 0
                    i32.load offset=8
                    i32.const 0
                    i32.store8
                    local.get 2
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=8
                  i32.const 1
                  i32.store8
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 4
                end
                local.get 0
                i32.load offset=4
                local.set 9
                local.get 0
                i32.load
                local.set 6
                block  ;; label = @7
                  local.get 4
                  i32.eqz
                  local.get 2
                  local.get 4
                  i32.eq
                  i32.or
                  local.tee 7
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 4
                  i32.le_u
                  br_if 3 (;@4;)
                  local.get 1
                  local.get 4
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 3 (;@4;)
                end
                local.get 6
                local.get 1
                local.get 4
                local.get 9
                i32.load offset=12
                call_indirect (type 6)
                br_if 4 (;@2;)
                block  ;; label = @7
                  local.get 7
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 4
                  i32.le_u
                  br_if 4 (;@3;)
                  local.get 1
                  local.get 4
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 4 (;@3;)
                end
                local.get 1
                local.get 4
                i32.add
                local.set 1
                local.get 2
                local.get 4
                i32.sub
                local.tee 2
                br_if 0 (;@6;)
              end
              i32.const 0
              local.set 4
              br 4 (;@1;)
            end
            local.get 6
            i32.const 4
            call $core::slice::slice_index_len_fail::h08f636efd7156c0a
            unreachable
          end
          local.get 1
          local.get 2
          i32.const 0
          local.get 4
          call $core::str::slice_error_fail::h571f7e6f7dc53361
          unreachable
        end
        local.get 1
        local.get 2
        local.get 4
        local.get 2
        call $core::str::slice_error_fail::h571f7e6f7dc53361
        unreachable
      end
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $core::fmt::builders::DebugTuple::field::h8c8629865c98eba0 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i64 i64 i64 i64 i64)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=8
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 5
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 6
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 1051926
        i32.const 1051930
        local.get 5
        select
        i32.const 2
        i32.const 1
        local.get 5
        select
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        br_if 1 (;@1;)
        local.get 1
        local.get 0
        i32.load
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 1051928
        i32.const 2
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.set 6
      end
      i32.const 1
      local.set 4
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 6
      i64.load offset=8 align=4
      local.set 7
      local.get 6
      i64.load offset=16 align=4
      local.set 8
      local.get 3
      i32.const 52
      i32.add
      i32.const 1051896
      i32.store
      local.get 3
      local.get 6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 6
      i64.load offset=32 align=4
      local.set 9
      local.get 6
      i64.load offset=40 align=4
      local.set 10
      local.get 3
      local.get 6
      i32.load8_u offset=48
      i32.store8 offset=72
      local.get 6
      i64.load align=4
      local.set 11
      local.get 3
      local.get 10
      i64.store offset=64
      local.get 3
      local.get 9
      i64.store offset=56
      local.get 3
      local.get 8
      i64.store offset=40
      local.get 3
      local.get 7
      i64.store offset=32
      local.get 3
      local.get 11
      i64.store offset=24
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 1051924
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 6)
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8 offset=8
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 3
    i32.const 80
    i32.add
    global.set 0
    local.get 0)
  (func $core::fmt::builders::DebugTuple::finish::h782a14fa96fe1df8 (type 5) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u offset=8
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      local.set 3
      i32.const 1
      local.set 1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=9
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.tee 3
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 3
          i32.load offset=24
          i32.const 1051931
          i32.const 1
          local.get 3
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 6)
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.tee 1
        i32.load offset=24
        i32.const 1051396
        i32.const 1
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store8 offset=8
    end
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne)
  (func $core::fmt::builders::DebugSet::entry::h201353a235e5dda9 (type 6) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $core::fmt::builders::DebugInner::entry::h8c1ee5224b40cd89
    local.get 0)
  (func $core::fmt::builders::DebugList::finish::h5e29443e3f085b64 (type 5) (param i32) (result i32)
    (local i32)
    i32.const 1
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.tee 0
      i32.load offset=24
      i32.const 1051934
      i32.const 1
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 6)
      local.set 1
    end
    local.get 1)
  (func $core::fmt::Write::write_char::hd305b6bb33c3ac9a (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 128
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 224
            i32.or
            i32.store8 offset=12
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=15
          local.get 2
          local.get 1
          i32.const 18
          i32.shr_u
          i32.const 240
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 4
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.store8 offset=12
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 31
      i32.and
      i32.const 192
      i32.or
      i32.store8 offset=12
      i32.const 2
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $<core::fmt::builders::PadAdapter_as_core::fmt::Write>::write_str::h1176366f9b2ebdfc
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $core::fmt::Write::write_fmt::h55cece1dd4fdc74f (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1051936
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $<&mut_W_as_core::fmt::Write>::write_str::h8a22eefaed8494fa (type 6) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $<core::fmt::builders::PadAdapter_as_core::fmt::Write>::write_str::h1176366f9b2ebdfc)
  (func $<&mut_W_as_core::fmt::Write>::write_char::ha316834a26c2dea1 (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $core::fmt::Write::write_char::hd305b6bb33c3ac9a)
  (func $<&mut_W_as_core::fmt::Write>::write_fmt::h71629794b2677f0c (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1051936
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $core::fmt::ArgumentV1::show_usize::h9435cf789a0efc8c (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $core::fmt::num::imp::fmt_u64::h6560fb621643a867)
  (func $core::fmt::num::imp::fmt_u64::h6560fb621643a867 (type 12) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 39
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop  ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1051694
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1051694
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_s
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 6
      local.get 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1051694
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_s
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1051694
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 1050869
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $<core::fmt::Arguments_as_core::fmt::Display>::fmt::h7bbd73d0f30c8175 (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 1
    i32.load offset=24
    local.set 1
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 0
    i64.load align=4
    i64.store offset=8
    local.get 1
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 0
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func $core::fmt::Formatter::pad_integral::hac3f8488e2699917 (type 13) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load
        local.tee 6
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 7
        local.get 1
        local.get 5
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load
      local.set 6
      i32.const 45
      local.set 7
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.set 10
        local.get 2
        local.set 1
        loop  ;; label = @3
          local.get 9
          local.get 1
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 8
      local.get 3
      i32.add
      local.get 9
      i32.sub
      local.set 8
    end
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $core::fmt::Formatter::pad_integral::write_prefix::h2cf83e6a56040156
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        return
      end
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 9
        local.get 8
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $core::fmt::Formatter::pad_integral::write_prefix::h2cf83e6a56040156
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 6)
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          i32.const 8
          i32.and
          br_if 0 (;@3;)
          local.get 9
          local.get 8
          i32.sub
          local.set 9
          i32.const 0
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 1
                local.get 0
                i32.load8_u offset=48
                local.tee 10
                local.get 10
                i32.const 3
                i32.eq
                select
                br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 0 (;@6;) 2 (;@4;)
              end
              local.get 9
              local.set 1
              i32.const 0
              local.set 9
              br 1 (;@4;)
            end
            local.get 9
            i32.const 1
            i32.shr_u
            local.set 1
            local.get 9
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 9
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 1
        local.get 0
        i32.const 1
        i32.store8 offset=48
        local.get 0
        i32.const 48
        i32.store offset=4
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $core::fmt::Formatter::pad_integral::write_prefix::h2cf83e6a56040156
        br_if 1 (;@1;)
        local.get 9
        local.get 8
        i32.sub
        local.set 9
        i32.const 0
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 0
              i32.load8_u offset=48
              local.tee 10
              local.get 10
              i32.const 3
              i32.eq
              select
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 0 (;@5;) 2 (;@3;)
            end
            local.get 9
            local.set 1
            i32.const 0
            local.set 9
            br 1 (;@3;)
          end
          local.get 9
          i32.const 1
          i32.shr_u
          local.set 1
          local.get 9
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 9
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=4
        local.set 10
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 6)
        br_if 1 (;@1;)
        local.get 9
        i32.const 1
        i32.add
        local.set 9
        local.get 0
        i32.load offset=28
        local.set 3
        local.get 0
        i32.load offset=24
        local.set 0
        loop  ;; label = @3
          block  ;; label = @4
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
            i32.const 0
            return
          end
          i32.const 1
          local.set 1
          local.get 0
          local.get 10
          local.get 3
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 0
      i32.load offset=4
      local.set 10
      i32.const 1
      local.set 1
      local.get 0
      local.get 7
      local.get 2
      local.get 3
      call $core::fmt::Formatter::pad_integral::write_prefix::h2cf83e6a56040156
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=24
      local.get 4
      local.get 5
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 6)
      br_if 0 (;@1;)
      local.get 9
      i32.const 1
      i32.add
      local.set 9
      local.get 0
      i32.load offset=28
      local.set 3
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        i32.const 1
        local.set 1
        local.get 0
        local.get 10
        local.get 3
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 1)
  (func $core::fmt::Formatter::pad_integral::write_prefix::h2cf83e6a56040156 (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1114112
        i32.eq
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type 2)
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=24
      local.get 2
      local.get 3
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 6)
      local.set 4
    end
    local.get 4)
  (func $core::fmt::Formatter::write_fmt::hb3bb9e03c3e75964 (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.const 28
    i32.add
    i32.load
    local.set 3
    local.get 0
    i32.load offset=24
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 0
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $core::fmt::write::hb137f2496e0ed1b6
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $core::fmt::Formatter::debug_lower_hex::hf149757ee45f6e30 (type 5) (param i32) (result i32)
    local.get 0
    i32.load8_u
    i32.const 16
    i32.and
    i32.const 4
    i32.shr_u)
  (func $core::fmt::Formatter::debug_upper_hex::h0919786d4217197b (type 5) (param i32) (result i32)
    local.get 0
    i32.load8_u
    i32.const 32
    i32.and
    i32.const 5
    i32.shr_u)
  (func $core::fmt::Formatter::debug_tuple::h5e5a0a16c42c97a8 (type 11) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.load offset=24
    local.get 2
    local.get 3
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 6)
    i32.store8 offset=8
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 3
    i32.eqz
    i32.store8 offset=9
    local.get 0
    i32.const 0
    i32.store offset=4)
  (func $core::fmt::Formatter::debug_list::h310f1e1cc9ab1b89 (type 4) (param i32 i32)
    (local i32)
    local.get 1
    i32.load offset=24
    i32.const 1051933
    i32.const 1
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 6)
    local.set 2
    local.get 0
    i32.const 0
    i32.store8 offset=5
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $<str_as_core::fmt::Debug>::fmt::h195f820ca2dc4e68 (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=24
        i32.const 34
        local.get 2
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type 2)
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.add
          local.set 6
          i32.const 0
          local.set 5
          local.get 0
          local.set 7
          local.get 0
          local.set 8
          i32.const 0
          local.set 9
          block  ;; label = @4
            loop  ;; label = @5
              local.get 7
              i32.const 1
              i32.add
              local.set 10
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 7
                    i32.load8_s
                    local.tee 11
                    i32.const -1
                    i32.gt_s
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 10
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 12
                        local.get 6
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 7
                      i32.load8_u offset=1
                      i32.const 63
                      i32.and
                      local.set 12
                      local.get 7
                      i32.const 2
                      i32.add
                      local.tee 10
                      local.set 7
                    end
                    local.get 11
                    i32.const 31
                    i32.and
                    local.set 4
                    block  ;; label = @9
                      local.get 11
                      i32.const 255
                      i32.and
                      local.tee 11
                      i32.const 223
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 12
                      local.get 4
                      i32.const 6
                      i32.shl
                      i32.or
                      local.set 12
                      br 2 (;@7;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 7
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 13
                        local.get 6
                        local.set 14
                        br 1 (;@9;)
                      end
                      local.get 7
                      i32.load8_u
                      i32.const 63
                      i32.and
                      local.set 13
                      local.get 7
                      i32.const 1
                      i32.add
                      local.tee 10
                      local.set 14
                    end
                    local.get 13
                    local.get 12
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set 12
                    block  ;; label = @9
                      local.get 11
                      i32.const 240
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 12
                      local.get 4
                      i32.const 12
                      i32.shl
                      i32.or
                      local.set 12
                      br 2 (;@7;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 14
                        local.get 6
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 11
                        local.get 10
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 14
                      i32.const 1
                      i32.add
                      local.set 7
                      local.get 14
                      i32.load8_u
                      i32.const 63
                      i32.and
                      local.set 11
                    end
                    local.get 12
                    i32.const 6
                    i32.shl
                    local.get 4
                    i32.const 18
                    i32.shl
                    i32.const 1835008
                    i32.and
                    i32.or
                    local.get 11
                    i32.or
                    local.tee 12
                    i32.const 1114112
                    i32.ne
                    br_if 2 (;@6;)
                    br 4 (;@4;)
                  end
                  local.get 11
                  i32.const 255
                  i32.and
                  local.set 12
                end
                local.get 10
                local.set 7
              end
              i32.const 2
              local.set 10
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 12
                          i32.const -9
                          i32.add
                          local.tee 11
                          i32.const 30
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 12
                          i32.const 92
                          i32.ne
                          br_if 1 (;@10;)
                          br 2 (;@9;)
                        end
                        i32.const 116
                        local.set 14
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 11
                            br_table 5 (;@7;) 1 (;@11;) 2 (;@10;) 2 (;@10;) 0 (;@12;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 3 (;@9;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 3 (;@9;) 5 (;@7;)
                          end
                          i32.const 114
                          local.set 14
                          br 4 (;@7;)
                        end
                        i32.const 110
                        local.set 14
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        i32.const 1055048
                        local.get 12
                        call $core::unicode::bool_trie::BoolTrie::lookup::h5985ded232b92c4f
                        br_if 0 (;@10;)
                        local.get 12
                        call $core::unicode::printable::is_printable::haacf9edc45c1c4bf
                        br_if 4 (;@6;)
                      end
                      local.get 12
                      i32.const 1
                      i32.or
                      i32.clz
                      i32.const 2
                      i32.shr_u
                      i32.const 7
                      i32.xor
                      i64.extend_i32_u
                      i64.const 21474836480
                      i64.or
                      local.set 15
                      i32.const 3
                      local.set 10
                      br 1 (;@8;)
                    end
                  end
                  local.get 12
                  local.set 14
                end
                local.get 3
                local.get 1
                i32.store offset=4
                local.get 3
                local.get 0
                i32.store
                local.get 3
                local.get 5
                i32.store offset=8
                local.get 3
                local.get 9
                i32.store offset=12
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    local.get 5
                    i32.lt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 5
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      local.get 1
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 5
                      local.get 1
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 0
                      local.get 5
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 1 (;@8;)
                    end
                    block  ;; label = @9
                      local.get 9
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 9
                      local.get 1
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 9
                      local.get 1
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 0
                      local.get 9
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 1 (;@8;)
                    end
                    local.get 2
                    i32.load offset=24
                    local.get 0
                    local.get 5
                    i32.add
                    local.get 9
                    local.get 5
                    i32.sub
                    local.get 2
                    i32.load offset=28
                    i32.load offset=12
                    call_indirect (type 6)
                    i32.eqz
                    br_if 1 (;@7;)
                    i32.const 1
                    local.set 4
                    br 6 (;@2;)
                  end
                  local.get 3
                  local.get 3
                  i32.const 12
                  i32.add
                  i32.store offset=24
                  local.get 3
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.store offset=20
                  local.get 3
                  local.get 3
                  i32.store offset=16
                  local.get 3
                  i32.const 16
                  i32.add
                  call $core::str::traits::<impl_core::slice::SliceIndex<str>_for_core::ops::range::Range<usize>>::index::__closure__::hdba06e33f13630a5
                  unreachable
                end
                loop  ;; label = @7
                  local.get 10
                  local.set 11
                  i32.const 1
                  local.set 4
                  i32.const 92
                  local.set 5
                  i32.const 1
                  local.set 10
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 11
                              br_table 2 (;@11;) 1 (;@12;) 5 (;@8;) 0 (;@13;) 2 (;@11;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 15
                                    i64.const 32
                                    i64.shr_u
                                    i32.wrap_i64
                                    i32.const 255
                                    i32.and
                                    br_table 5 (;@11;) 3 (;@13;) 2 (;@14;) 1 (;@15;) 0 (;@16;) 6 (;@10;) 5 (;@11;)
                                  end
                                  local.get 15
                                  i64.const -1095216660481
                                  i64.and
                                  i64.const 12884901888
                                  i64.or
                                  local.set 15
                                  i32.const 3
                                  local.set 10
                                  i32.const 117
                                  local.set 5
                                  br 7 (;@8;)
                                end
                                local.get 15
                                i64.const -1095216660481
                                i64.and
                                i64.const 8589934592
                                i64.or
                                local.set 15
                                i32.const 3
                                local.set 10
                                i32.const 123
                                local.set 5
                                br 6 (;@8;)
                              end
                              local.get 14
                              local.get 15
                              i32.wrap_i64
                              local.tee 11
                              i32.const 2
                              i32.shl
                              i32.const 28
                              i32.and
                              i32.shr_u
                              i32.const 15
                              i32.and
                              local.tee 10
                              i32.const 48
                              i32.or
                              local.get 10
                              i32.const 87
                              i32.add
                              local.get 10
                              i32.const 10
                              i32.lt_u
                              select
                              local.set 5
                              block  ;; label = @14
                                local.get 11
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 15
                                i64.const -1
                                i64.add
                                i64.const 4294967295
                                i64.and
                                local.get 15
                                i64.const -4294967296
                                i64.and
                                i64.or
                                local.set 15
                                br 5 (;@9;)
                              end
                              local.get 15
                              i64.const -1095216660481
                              i64.and
                              i64.const 4294967296
                              i64.or
                              local.set 15
                              br 4 (;@9;)
                            end
                            local.get 15
                            i64.const -1095216660481
                            i64.and
                            local.set 15
                            i32.const 3
                            local.set 10
                            i32.const 125
                            local.set 5
                            br 4 (;@8;)
                          end
                          i32.const 0
                          local.set 10
                          local.get 14
                          local.set 5
                          br 3 (;@8;)
                        end
                        i32.const 1
                        local.set 10
                        block  ;; label = @11
                          local.get 12
                          i32.const 128
                          i32.lt_u
                          br_if 0 (;@11;)
                          i32.const 2
                          local.set 10
                          local.get 12
                          i32.const 2048
                          i32.lt_u
                          br_if 0 (;@11;)
                          i32.const 3
                          i32.const 4
                          local.get 12
                          i32.const 65536
                          i32.lt_u
                          select
                          local.set 10
                        end
                        local.get 10
                        local.get 9
                        i32.add
                        local.set 5
                        br 4 (;@6;)
                      end
                      local.get 15
                      i64.const -1095216660481
                      i64.and
                      i64.const 17179869184
                      i64.or
                      local.set 15
                    end
                    i32.const 3
                    local.set 10
                  end
                  local.get 2
                  i32.load offset=24
                  local.get 5
                  local.get 2
                  i32.load offset=28
                  i32.load offset=16
                  call_indirect (type 2)
                  br_if 5 (;@2;)
                  br 0 (;@7;)
                end
              end
              local.get 9
              local.get 8
              i32.sub
              local.get 7
              i32.add
              local.set 9
              local.get 7
              local.set 8
              local.get 6
              local.get 7
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 1
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          local.get 1
          i32.ge_u
          br_if 2 (;@1;)
          local.get 0
          local.get 5
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 2 (;@1;)
        end
        i32.const 1
        local.set 4
        local.get 2
        i32.load offset=24
        local.get 0
        local.get 5
        i32.add
        local.get 1
        local.get 5
        i32.sub
        local.get 2
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 6)
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=24
        i32.const 34
        local.get 2
        i32.load offset=28
        i32.load offset=16
        call_indirect (type 2)
        local.set 4
      end
      local.get 3
      i32.const 32
      i32.add
      global.set 0
      local.get 4
      return
    end
    local.get 0
    local.get 1
    local.get 5
    local.get 1
    call $core::str::slice_error_fail::h571f7e6f7dc53361
    unreachable)
  (func $<str_as_core::fmt::Display>::fmt::h38ffb460df0bf4b9 (type 6) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $core::fmt::Formatter::pad::hd367b6bcbe89f492)
  (func $core::fmt::num::<impl_core::fmt::LowerHex_for_i32>::fmt::h09b955a98f459559 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 87
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1051692
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $core::unicode::printable::check::hf6373bfc83e92c23 (type 14) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.const 1
        i32.shl
        i32.add
        local.set 8
        local.get 0
        i32.const 65280
        i32.and
        i32.const 8
        i32.shr_u
        local.set 9
        i32.const 0
        local.set 10
        local.get 0
        i32.const 255
        i32.and
        local.set 11
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const 2
            i32.add
            local.set 12
            local.get 10
            local.get 1
            i32.load8_u offset=1
            local.tee 2
            i32.add
            local.set 13
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 1
              local.get 9
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.get 9
              i32.gt_u
              br_if 3 (;@2;)
              local.get 13
              local.set 10
              local.get 12
              local.set 1
              local.get 12
              local.get 8
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 13
              local.get 10
              i32.lt_u
              br_if 0 (;@5;)
              local.get 13
              local.get 4
              i32.gt_u
              br_if 2 (;@3;)
              local.get 3
              local.get 10
              i32.add
              local.set 1
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 2
                  i32.const -1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load8_u
                  local.set 10
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 10
                  local.get 11
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 7
                br 5 (;@1;)
              end
              local.get 13
              local.set 10
              local.get 12
              local.set 1
              local.get 12
              local.get 8
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
          end
          local.get 10
          local.get 13
          call $core::slice::slice_index_order_fail::h45638c641c9b3b30
          unreachable
        end
        local.get 13
        local.get 4
        call $core::slice::slice_index_len_fail::h08f636efd7156c0a
        unreachable
      end
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      local.get 6
      i32.add
      local.set 11
      local.get 0
      i32.const 65535
      i32.and
      local.set 1
      i32.const 1
      local.set 7
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.load8_u
              local.tee 2
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              local.tee 13
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 10
              local.set 5
              br 1 (;@4;)
            end
            local.get 10
            local.get 11
            i32.eq
            br_if 2 (;@2;)
            local.get 13
            i32.const 127
            i32.and
            i32.const 8
            i32.shl
            local.get 5
            i32.load8_u offset=1
            i32.or
            local.set 2
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          local.get 7
          i32.const 1
          i32.xor
          local.set 7
          local.get 5
          local.get 11
          i32.ne
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      i32.const 1051056
      call $core::panicking::panic::h0142ee7f4c64bd08
      unreachable
    end
    local.get 7
    i32.const 1
    i32.and)
  (func $core::fmt::num::<impl_core::fmt::UpperHex_for_i8>::fmt::hd760e3f648143fcc (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 55
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1051692
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $core::fmt::num::<impl_core::fmt::UpperHex_for_i32>::fmt::h288c0aa06d70df28 (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 55
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      call $core::slice::slice_index_order_fail::h45638c641c9b3b30
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1051692
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $core::fmt::Formatter::pad_integral::hac3f8488e2699917
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $core::fmt::num::imp::<impl_core::fmt::Display_for_i32>::fmt::hb257468ac8698236 (type 2) (param i32 i32) (result i32)
    (local i64)
    local.get 0
    i32.load
    local.tee 0
    i64.extend_i32_s
    local.tee 2
    local.get 2
    i64.const 63
    i64.shr_s
    local.tee 2
    i64.add
    local.get 2
    i64.xor
    local.get 0
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.get 1
    call $core::fmt::num::imp::fmt_u64::h6560fb621643a867)
  (func $memcpy (type 6) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcmp (type 6) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (table (;0;) 69 69 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1057146))
  (global (;2;) i32 (i32.const 1057146))
  (global (;3;) i32 (i32.const 1056576))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "__rustc_debug_gdb_scripts_section__" (global 3))
  (export "main" (func $main))
  (elem (;0;) (i32.const 1) func $core::ptr::real_drop_in_place::h3bc6cfbc64ffdde6 $std::rt::lang_start::__closure__::h1c0e9359e1a4f911 $core::ops::function::FnOnce::call_once__vtable.shim__::hf8647b93951115d6 $hw_rust::main::ha1617da2eac3802e $<core::fmt::Arguments_as_core::fmt::Display>::fmt::h7bbd73d0f30c8175 $<&T_as_core::fmt::Debug>::fmt::h090d81485cc71665 $core::fmt::num::imp::<impl_core::fmt::Display_for_i32>::fmt::hb257468ac8698236 $<alloc::string::String_as_core::fmt::Display>::fmt::h1c3337deac11fc7c $<&T_as_core::fmt::Display>::fmt::h251750df0dd9a432 $<std::io::error::Error_as_core::fmt::Display>::fmt::h98edb03f11a4710e $std::alloc::default_alloc_error_hook::h4c4aa82eea9626e8 $std::panicking::try::do_call::h7511ee0f6f4a8e2a $<std::sys_common::thread_local::Key_as_core::ops::drop::Drop>::drop::ha98c40f1657718ec $<&mut_W_as_core::fmt::Write>::write_str::h292f3bef30be5ae9 $<&mut_W_as_core::fmt::Write>::write_char::h29fafe67e786b5e9 $<&mut_W_as_core::fmt::Write>::write_fmt::h2b2a24f11dbb5e86 $<&mut_W_as_core::fmt::Write>::write_str::h6cf1ca1d364d8309 $<&mut_W_as_core::fmt::Write>::write_char::hdd43bb9ce4ee3aa1 $<&mut_W_as_core::fmt::Write>::write_fmt::ha9d8918552803d5f $<&T_as_core::fmt::Debug>::fmt::h1874024e098cc5f9 $core::ptr::real_drop_in_place::h08b326c460981070 $<T_as_core::any::Any>::type_id::h047c16fec401b221 $core::ptr::real_drop_in_place::hed899f15376b0cdc $<std::ffi::c_str::NulError_as_core::fmt::Debug>::fmt::h66d28a634ebb9809 $<core::cell::BorrowError_as_core::fmt::Debug>::fmt::hcf9a3603fd81bdae $<core::cell::BorrowMutError_as_core::fmt::Debug>::fmt::h41a576614c12f53d $core::ptr::real_drop_in_place::h6f9e84b48a4387b2 $<std::sys_common::poison::PoisonError<T>_as_core::fmt::Debug>::fmt::h2aeb25b601b8e9c6 $core::ptr::real_drop_in_place::h481a15a182dcb798 $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_std::error::Error>::description::h9ca06f358a7fd8af $std::error::Error::cause::h0a468326a46f05ba $std::error::Error::type_id::h3398eed2c8189c49 $std::error::Error::backtrace::h4da4cddfeacc4aca $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_core::fmt::Display>::fmt::hb37607c61a21babc $<std::error::<impl_core::convert::From<alloc::string::String>_for_alloc::boxed::Box<dyn_std::error::Error+core::marker::Send+core::marker::Sync>>::from::StringError_as_core::fmt::Debug>::fmt::h5fd9380e7b2f9c51 $core::ptr::real_drop_in_place::h1c5271a22a4bbc56 $<std::io::Write::write_fmt::Adaptor<T>_as_core::fmt::Write>::write_str::h190ca40cd38a8fb4 $core::fmt::Write::write_char::h2ef624cf48b9cdf4 $core::fmt::Write::write_fmt::h87ff8cf34eea5f27 $std::sync::once::Once::call_once::__closure__::h0dd633692ce85f20 $core::ops::function::FnOnce::call_once__vtable.shim__::h567968cd02331d13 $core::ops::function::FnOnce::call_once__vtable.shim__::hb430745993719cf4 $core::ptr::real_drop_in_place::hff6df1afa53ab3b9 $<std::panicking::continue_panic_fmt::PanicPayload_as_core::panic::BoxMeUp>::box_me_up::ha93a5fbf0ceb0d85 $<std::panicking::continue_panic_fmt::PanicPayload_as_core::panic::BoxMeUp>::get::h57815b869d589859 $<T_as_core::any::Any>::type_id::h2d4d17f20cb15612 $core::ptr::real_drop_in_place::h2621533cc62ca9fc $<std::panicking::begin_panic::PanicPayload<A>_as_core::panic::BoxMeUp>::box_me_up::h78e2a7abf0003e1e $<std::panicking::begin_panic::PanicPayload<A>_as_core::panic::BoxMeUp>::get::h9b1c7408f888c50f $<T_as_core::any::Any>::type_id::h8b736fbe99f2e981 $<T_as_core::any::Any>::type_id::had50064618b8bd18 $<&T_as_core::fmt::Debug>::fmt::hdf6f72ae8da72177 $core::fmt::num::imp::<impl_core::fmt::Display_for_u32>::fmt::h3518dbff2fc7fe22 $<&T_as_core::fmt::Display>::fmt::hbdb54b8c793ef0af $<core::ops::range::Range<Idx>_as_core::fmt::Debug>::fmt::h7eaf6892c126f203 $<char_as_core::fmt::Debug>::fmt::h50a7482d13f3c4e4 $core::fmt::ArgumentV1::show_usize::h9435cf789a0efc8c $<&T_as_core::fmt::Debug>::fmt::h33f1a352ef5b670f $core::ptr::real_drop_in_place::he0f5620a77bcc8c4 $<T_as_core::any::Any>::type_id::h40a48bfc40f5283f $core::ptr::real_drop_in_place::h219a4dd4886f8f7b $<core::fmt::builders::PadAdapter_as_core::fmt::Write>::write_str::h1176366f9b2ebdfc $core::fmt::Write::write_char::hd305b6bb33c3ac9a $core::fmt::Write::write_fmt::h55cece1dd4fdc74f $<core::ffi::VaListImpl_as_core::ops::drop::Drop>::drop::ha1b90773b9afe6ad $<&mut_W_as_core::fmt::Write>::write_str::h8a22eefaed8494fa $<&mut_W_as_core::fmt::Write>::write_char::ha316834a26c2dea1 $<&mut_W_as_core::fmt::Write>::write_fmt::h71629794b2677f0c)
  (data (;0;) (i32.const 1048576) "\01\00\00\00\04\00\00\00\04\00\00\00\02\00\00\00\02\00\00\00\03\00\00\00Hello, world!\0a\00\00\18\00\10\00\0e\00\00\00\0d\00\00\00\04\00\00\00\04\00\00\00\0e\00\00\00\0f\00\00\00\10\00\00\00\0d\00\00\00\04\00\00\00\04\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00\0d\00\00\00\04\00\00\00\04\00\00\00\14\00\00\00already borrowedalready mutably borrowedassertion failed: `(left == right)`\0a  left: ``,\0a right: ``\00\00\98\00\10\00-\00\00\00\c5\00\10\00\0c\00\00\00\d1\00\10\00\01\00\00\00\15\00\00\00\00\00\00\00\01\00\00\00\16\00\00\00`: \00\98\00\10\00-\00\00\00\c5\00\10\00\0c\00\00\00\fc\00\10\00\03\00\00\00called `Option::unwrap()` on a `None` valuesrc/libcore/option.rs\18\01\10\00+\00\00\00C\01\10\00\15\00\00\00z\01\00\00\15\00\00\00\17\00\00\00\10\00\00\00\04\00\00\00\18\00\00\00\15\00\00\00\00\00\00\00\01\00\00\00\19\00\00\00\15\00\00\00\00\00\00\00\01\00\00\00\1a\00\00\00called `Result::unwrap()` on an `Err` value\00\1b\00\00\00\08\00\00\00\04\00\00\00\1c\00\00\00src/liballoc/raw_vec.rsTried to shrink to a larger capacity\00\f3\01\10\00$\00\00\00\dc\01\10\00\17\00\00\00]\02\00\00\09\00\00\00\0d\00\00\00\04\00\00\00\04\00\00\00\06\00\00\00src/libstd/thread/mod.rs@\02\10\00\18\00\00\00\8c\03\00\00\13\00\00\00inconsistent park state\00\02\00\00\00park state changed unexpectedly\00\84\02\10\00\1f\00\00\00@\02\10\00\18\00\00\00\89\03\00\00\0d\00\00\00@\02\10\00\18\00\00\00\22\04\00\00\11\00\00\00failed to generate unique thread ID: bitspace exhaustedthread name may not contain interior null bytes\00\00@\02\10\00\18\00\00\00\97\04\00\00\12\00\00\00inconsistent state in unpark\1d\00\00\00\0c\00\00\00\04\00\00\00\1e\00\00\00\1f\00\00\00\1f\00\00\00 \00\00\00!\00\00\00\22\00\00\00#\00\00\00unexpected end of fileother os erroroperation interruptedwrite zerotimed outinvalid datainvalid input parameteroperation would blockentity already existsbroken pipeaddress not availableaddress in usenot connectedconnection abortedconnection resetconnection refusedpermission deniedentity not found\00\00\00\88\03\10\00\00\00\00\00 (os error )\88\03\10\00\00\00\00\00\bc\04\10\00\0b\00\00\00\c7\04\10\00\01\00\00\00cannot access stdout during shutdownfailed printing to : \00\00\00\04\05\10\00\13\00\00\00\17\05\10\00\02\00\00\00src/libstd/io/stdio.rs\00\00,\05\10\00\16\00\00\00\18\03\00\00\09\00\00\00stdoutfailed to write whole buffer\00\00$\00\00\00\0c\00\00\00\04\00\00\00%\00\00\00&\00\00\00'\00\00\00formatter errorsrc/libstd/sync/condvar.rs\00\00\00\9f\05\10\00\1a\00\00\00H\02\00\00\12\00\00\00attempted to use a condition variable with two mutexes\00\00\0d\00\00\00\04\00\00\00\04\00\00\00(\00\00\00)\00\00\00src/libstd/sync/once.rs\00\18\06\10\00\17\00\00\00\93\01\00\00\15\00\00\00assertion failed: state & STATE_MASK == RUNNING\00\18\06\10\00\17\00\00\00o\01\00\00\15\00\00\00Once instance has previously been poisoned\00\00\18\06\10\00\17\00\00\00\c5\01\00\00\09\00\00\00src/libstd/sys_common/at_exit_imp.rs\bc\06\10\00$\00\00\001\00\00\00\0d\00\00\00assertion failed: queue != DONEPoisonError { inner: .. }src/libstd/sys_common/thread_info.rs(\07\10\00$\00\00\00%\00\00\00\1a\00\00\00assertion failed: c.borrow().is_none()\00\00\0d\00\00\00\04\00\00\00\04\00\00\00*\00\00\00+\00\00\00\10\00\00\00\04\00\00\00,\00\00\00-\00\00\00\1d\00\00\00\0c\00\00\00\04\00\00\00.\00\00\00/\00\00\00\08\00\00\00\04\00\00\000\00\00\001\00\00\00/\00\00\00\08\00\00\00\04\00\00\002\00\00\00\15\00\00\00\00\00\00\00\01\00\00\003\00\00\00NulError\0d\00\00\00\04\00\00\00\04\00\00\004\00\00\00operation successfulsrc/libstd/sys/wasm/condvar.rs\00\00\18\08\10\00\1e\00\00\00\17\00\00\00\09\00\00\00can't block with web assemblysrc/libstd/sys/wasm/mutex.rs\00\00\00e\08\10\00\1c\00\00\00\16\00\00\00\09\00\00\00cannot recursively acquire mutexsrc/liballoc/raw_vec.rscapacity overflow\cb\08\10\00\11\00\00\00\b4\08\10\00\17\00\00\00\09\03\00\00\05\00\00\00`..\00\f5\08\10\00\02\00\00\00BorrowErrorBorrowMutError\00\00\00;\00\00\00\00\00\00\00\01\00\00\00<\00\00\00index out of bounds: the len is  but the index is \00\00,\09\10\00 \00\00\00L\09\10\00\12\00\00\00called `Option::unwrap()` on a `None` valuesrc/libcore/option.rsp\09\10\00+\00\00\00\9b\09\10\00\15\00\00\00z\01\00\00\15\00\00\00\f5\08\10\00\00\00\00\00\9b\09\10\00\15\00\00\00\a6\04\00\00\05\00\00\00: \00\00\f5\08\10\00\00\00\00\00\e0\09\10\00\02\00\00\00src/libcore/result.rs\00\00\00\f4\09\10\00\15\00\00\00\8d\04\00\00\05\00\00\00src/libcore/slice/mod.rsindex  out of range for slice of length 4\0a\10\00\06\00\00\00:\0a\10\00\22\00\00\00\1c\0a\10\00\18\00\00\00\19\0a\00\00\05\00\00\00slice index starts at  but ends at \00|\0a\10\00\16\00\00\00\92\0a\10\00\0d\00\00\00\1c\0a\10\00\18\00\00\00\1f\0a\00\00\05\00\00\00attempted to index slice up to maximum usize\c0\0a\10\00,\00\00\00\1c\0a\10\00\18\00\00\00%\0a\00\00\05\00\00\00)src/libcore/str/mod.rs[...]byte index  is out of bounds of `\00\00\00 \0b\10\00\0b\00\00\00+\0b\10\00\16\00\00\00\f4\08\10\00\01\00\00\00\05\0b\10\00\16\00\00\00\03\08\00\00\09\00\00\00begin <= end ( <= ) when slicing `\00\00l\0b\10\00\0e\00\00\00z\0b\10\00\04\00\00\00~\0b\10\00\10\00\00\00\f4\08\10\00\01\00\00\00\05\0b\10\00\16\00\00\00\07\08\00\00\05\00\00\00 is not a char boundary; it is inside  (bytes ) of ` \0b\10\00\0b\00\00\00\c0\0b\10\00&\00\00\00\e6\0b\10\00\08\00\00\00\ee\0b\10\00\06\00\00\00\f4\08\10\00\01\00\00\00\05\0b\10\00\16\00\00\00\14\08\00\00\05\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00=\00\00\00\0c\00\00\00\04\00\00\00>\00\00\00?\00\00\00@\00\00\00    ,\0a, (\0a(,\0a[]\00A\00\00\00\04\00\00\00\04\00\00\00B\00\00\00C\00\00\00D\00\00\00\00\00\00\00\00\00\00\00src/libcore/fmt/mod.rs\00\00@\0d\10\00\16\00\00\00V\04\00\00(\00\00\00@\0d\10\00\16\00\00\00b\04\00\00\11\00\00\00\00\00\00\00\00\00\00\00src/libcore/unicode/bool_trie.rs\80\0d\10\00 \00\00\00'\00\00\00\19\00\00\00\80\0d\10\00 \00\00\00(\00\00\00 \00\00\00\80\0d\10\00 \00\00\00*\00\00\00\19\00\00\00\80\0d\10\00 \00\00\00+\00\00\00\18\00\00\00\80\0d\10\00 \00\00\00,\00\00\00 \00\00\00\00\01\03\05\05\06\06\03\07\06\08\08\09\11\0a\1c\0b\19\0c\14\0d\12\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\05\18\02\19\03\1a\07\1c\02\1d\01\1f\16 \03+\04,\02-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\04\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\1d\dd\0e\0fKL\fb\fc./?\5c]_\b5\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11)EIWde\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\04\0d\11EIde\80\81\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80\0dmq\de\df\0e\0f\1fno\1c\1d_}~\ae\af\bb\bc\fa\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96\97/_&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\c0\c1\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91\fe\ffSgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab5\1e\15\80\e0\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\08\02\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05:\03\11\07\06\05\10\07W\07\02\07\15\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\15\0b\17\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06\1fAL\04-\03t\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03 \10!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\b00t\0c\80\d6\1a\0c\05\80\ff\05\80\b6\05$\0c\9b\c6\0a\d20\10\84\8d\037\09\81\5c\14\80\b8\08\80\c705\04\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09H\08\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a5\11\81m\10x(*\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\08\08\09\02\0a\05\0b\02\10\01\11\04\12\05\13\11\14\02\15\02\17\02\19\04\1c\05\1d\08$\01j\03k\02\bc\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e8\02\ee \f0\04\f9\06\fa\02\0c';>NO\8f\9e\9e\9f\06\07\096=>V\f3\d0\d1\04\14\1867VW\bd5\ce\cf\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOdeZ\5c\b6\b7\1b\1c\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\ee\efZb\9a\9b'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\0cr\a3\a4\cb\ccno^\22{\05\03\04-\03e\04\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0b\01\80\90\817\09\16\0a\08\80\989\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&\1f\80\81(\08*\80\86\17\09N\04\1e\0fC\0e\19\07\0a\06G\09'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8b` H\08\0a\80\a6^\22E\0b\0a\06\0d\139\07\0a6,\04\10\80\c0<dS\0c\01\80\a0E\1bH\08S\1d9\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\c72\0d\83\9bfu\0b\80\c4\8a\bc\84/\8f\d1\82G\a1\b9\829\07*\04\02`&\0aF\0a(\05\13\82\b0[eK\049\07\11@\04\1c\97\f8\08\82\f3\a5\0d\81\1f1\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\93`\80\f6\0as\08n\17F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\85P+\80\d5-\03\1a\04\02\81p:\05\01\85\00\80\d7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\0d\03]\03=9\1d\0d,\04\09\07\02\0e\06\80\9a\83\d6\0a\0d\03\0b\05t\0cY\07\0c\14\0c\048\08\0a\06(\08\1eRw\031\03\80\a6\0c\14\04\03\05\03\0d\06\85j\00\00\00\00\00\c0\fb\ef>\00\00\00\00\00\0e\00\00\00\00\00\00\00\00\00\00\00\00\00\00\f8\ff\fb\ff\ff\ff\07\00\00\00\00\00\00\14\fe!\fe\00\0c\00\00\00\02\00\00\00\00\00\00P\1e \80\00\0c\00\00@\06\00\00\00\00\00\00\10\869\02\00\00\00#\00\be!\00\00\0c\00\00\fc\02\00\00\00\00\00\00\d0\1e \c0\00\0c\00\00\00\04\00\00\00\00\00\00@\01 \80\00\00\00\00\00\11\00\00\00\00\00\00\c0\c1=`\00\0c\00\00\00\02\00\00\00\00\00\00\90D0`\00\0c\00\00\00\03\00\00\00\00\00\00X\1e \80\00\0c\00\00\00\00\84\5c\80\00\00\00\00\00\00\00\00\00\00\f2\07\80\7f\00\00\00\00\00\00\00\00\00\00\00\00\f2\1f\00?\00\00\00\00\00\00\00\00\00\03\00\00\a0\02\00\00\00\00\00\00\fe\7f\df\e0\ff\fe\ff\ff\ff\1f@\00\00\00\00\00\00\00\00\00\00\00\00\e0\fdf\00\00\00\c3\01\00\1e\00d \00 \00\00\00\00\00\00\00\e0\00\00\00\00\00\00\1c\00\00\00\1c\00\00\00\0c\00\00\00\0c\00\00\00\00\00\00\00\b0?@\fe\0f \00\00\00\00\008\00\00\00\00\00\00`\00\00\00\00\02\00\00\00\00\00\00\87\01\04\0e\00\00\80\09\00\00\00\00\00\00@\7f\e5\1f\f8\9f\00\00\00\00\00\00\ff\7f\0f\00\00\00\00\00\f0\17\04\00\00\00\00\f8\0f\00\03\00\00\00<;\00\00\00\00\00\00@\a3\03\00\00\00\00\00\00\f0\cf\00\00\00\f7\ff\fd!\10\03\ff\ff\ff\ff\ff\ff\ff\fb\00\10\00\00\00\00\00\00\00\00\ff\ff\ff\ff\01\00\00\00\00\00\00\80\03\00\00\00\00\00\00\00\00\80\00\00\00\00\ff\ff\ff\ff\00\00\00\00\00\fc\00\00\00\00\00\06\00\00\00\00\00\00\00\00\00\80\f7?\00\00\00\c0\00\00\00\00\00\00\00\00\00\00\03\00D\08\00\00`\00\00\000\00\00\00\ff\ff\03\80\00\00\00\00\c0?\00\00\80\ff\03\00\00\00\00\00\07\00\00\00\00\00\c83\00\00\00\00 \00\00\00\00\00\00\00\00~f\00\08\10\00\00\00\00\00\10\00\00\00\00\00\00\9d\c1\02\00\00\00\000@\00\00\00\00\00 !\00\00\00\00\00@\00\00\00\00\ff\ff\00\00\ff\ff\00\00\00\00\00\00\00\00\00\01\00\00\00\02\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\04\00\00\05\00\00\00\00\00\00\00\00\06\00\00\00\00\00\00\00\00\07\00\00\08\09\0a\00\0b\0c\0d\0e\0f\00\00\10\11\12\00\00\13\14\15\16\00\00\17\18\19\1a\1b\00\1c\00\00\00\1d\00\00\00\00\00\00\1e\1f !\00\00\00\00\00\22\00#\00$%&\00\00\00\00'\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00()\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00*+\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00,\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00-.\00\00/\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00012\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\003\00\00\00)\00\00\00\00\00\004\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\005\006\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0078\00\008889\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \00\00\00\00\01\00\00\00\00\00\00\00\00\00\c0\07n\f0\00\00\00\00\00\87\00\00\00\00`\00\00\00\00\00\00\00\f0\00\00\00\c0\ff\01\00\00\00\00\00\02\00\00\00\00\00\00\ff\7f\00\00\00\00\00\00\80\03\00\00\00\00\00x\06\07\00\00\00\80\ef\1f\00\00\00\00\00\00\00\08\00\03\00\00\00\00\00\c0\7f\00\1e\00\00\00\00\00\00\00\00\00\00\00\80\d3@\00\00\00\80\f8\07\00\00\03\00\00\00\00\00\00X\01\00\80\00\c0\1f\1f\00\00\00\00\00\00\00\00\ff\5c\00\00@\00\00\00\00\00\00\00\00\00\00\f9\a5\0d\00\00\00\00\00\00\00\00\00\00\00\00\80<\b0\01\00\000\00\00\00\00\00\00\00\00\00\00\f8\a7\01\00\00\00\00\00\00\00\00\00\00\00\00(\bf\00\00\00\00\e0\bc\0f\00\00\00\00\00\00\00\80\ff\06\00\00\f0\0c\01\00\00\00\fe\07\00\00\00\00\f8y\80\00~\0e\00\00\00\00\00\fc\7f\03\00\00\00\00\00\00\00\00\00\00\7f\bf\00\00\fc\ff\ff\fcm\00\00\00\00\00\00\00~\b4\bf\00\00\00\00\00\00\00\00\00\a3\00\00\00\00\00\00\00\00\00\00\00\18\00\00\00\00\00\00\00\1f\00\00\00\00\00\00\00\7f\00\00\80\00\00\00\00\00\00\00\80\07\00\00\00\00\00\00\00\00`\00\00\00\00\00\00\00\00\a0\c3\07\f8\e7\0f\00\00\00<\00\00\1c\00\00\00\00\00\00\00\ff\ff\ff\ff\ff\ff\7f\f8\ff\ff\ff\ff\ff\1f \00\10\00\00\f8\fe\ff\00\00\7f\ff\ff\f9\db\07\00\00\00\00\00\00\00\f0\00\00\00\00\7f\00\00\00\00\00\f0\07\00\00\00\00\00\00\00\00\00\00\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\f8\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\fe\ff\ff\ff\ff\bf\b6\00\00\00\00\00\00\00\00\00\ff\07\00\00\00\00\00\f8\ff\ff\00\00\01\00\00\00\00\00\00\00\00\00\00\00\c0\9f\9f=\00\00\00\00\02\00\00\00\ff\ff\ff\07\00\00\00\00\00\00\00\00\00\00\c0\ff\01\00\00\00\00\00\00\f8\0f (\13\10\00J\00\00\00x\15\10\00\00\02\00\00x\17\10\00:\00\00\00\00\01\02\03\04\05\06\07\08\09\08\0a\0b\0c\0d\0e\0f\10\11\12\13\14\02\15\16\17\18\19\1a\1b\1c\1d\1e\1f \02\02\02\02\02\02\02\02\02\02!\02\02\02\02\02\02\02\02\02\02\02\02\02\02\22#$%&\02'\02(\02\02\02)*+\02,-./0\02\021\02\02\022\02\02\02\02\02\02\02\023\02\024\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\025\026\027\02\02\02\02\02\02\02\028\029\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02:;<\02\02\02\02=\02\02>?@ABCDEF\02\02\02G\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02H\02\02\02\02\02\02\02\02\02\02\02I\02\02\02\02\02;\02\00\01\02\02\02\02\03\02\02\02\02\04\02\05\06\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\07\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02")
  (data (;1;) (i32.const 1056576) "\01gdb_load_rust_pretty_printers.py\00")
  (data (;2;) (i32.const 1056616) "\01\00\00\00\00\00\00\00")
  (data (;3;) (i32.const 1056624) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
