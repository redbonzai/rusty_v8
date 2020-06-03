// Copyright 2019-2020 the Deno authors. All rights reserved. MIT license.
use rusty_v8 as v8;

pub fn main() {
  let mut isolate = v8::Isolate::new(mock());
  let mut scope1 = v8::HandleScope::new(&mut isolate);
  let context = v8::Context::new(&mut scope1);
  let mut scope2 = v8::ContextScope::new(&mut scope1, context);

  let mut try_catch = v8::TryCatch::new(&mut scope2);
  let try_catch = try_catch.enter();

  let _message = {
    let mut scope3 = v8::HandleScope::new(&mut scope2);
    let mut scope4 = v8::HandleScope::new(&mut scope3);
    try_catch.message(&mut scope4).unwrap()
  };
}

fn mock<T>() -> T {
  unimplemented!()
}
