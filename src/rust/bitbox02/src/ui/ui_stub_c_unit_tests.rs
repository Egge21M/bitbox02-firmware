// Copyright 2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Stubs for the Bitbox02 simulator and also C unit-tests.

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

use crate::input::SafeInputString;

use core::marker::PhantomData;

extern crate alloc;

pub struct Component<'a> {
    is_pushed: bool,
    _p: PhantomData<&'a ()>,
}

impl<'a> Component<'a> {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        self.is_pushed = true;
    }
}

impl<'a> Drop for Component<'a> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
    }
}

pub fn trinary_input_string_create<'a, F>(
    _params: &TrinaryInputStringParams,
    mut confirm_callback: F,
    _cancel_callback: Option<ContinueCancelCb<'a>>,
) -> Component<'a>
where
    F: FnMut(SafeInputString) + 'a,
{
    confirm_callback(SafeInputString::new());
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn confirm_create<'a, F>(_params: &ConfirmParams, mut result_callback: F) -> Component<'a>
where
    F: FnMut(bool) + 'a,
{
    result_callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn screen_process() {}

pub fn status_create<'a, F>(_text: &str, _status_success: bool, mut callback: F) -> Component<'a>
where
    F: FnMut() + 'a,
{
    callback();
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn sdcard_create<'a, F>(_insert: bool, mut continue_callback: F) -> Component<'a>
where
    F: FnMut() + 'a,
{
    continue_callback();
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn menu_create(_params: MenuParams<'_>) -> Component<'_> {
    panic!("not implemented");
}

pub fn trinary_choice_create<'a>(
    _message: &'a str,
    _label_left: &'a str,
    _label_middle: &'a str,
    _label_right: &'a str,
    _chosen_callback: TrinaryChoiceCb,
) -> Component<'a> {
    panic!("not implemented")
}

pub fn confirm_transaction_address_create<'a, 'b>(
    _amount: &'a str,
    _address: &'a str,
    mut callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn confirm_transaction_fee_create<'a, 'b>(
    _amount: &'a str,
    _fee: &'a str,
    _longtouch: bool,
    mut callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn trinary_input_string_set_input(_component: &mut Component, _word: &str) {
    panic!("not implemented")
}

pub fn with_lock_animation<F: Fn()>(f: F) {
    f()
}

pub fn screen_stack_pop_all() {}

pub fn progress_create<'a>(_title: &str) -> Component<'a> {
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn progress_set(_component: &mut Component, _progress: f32) {}

pub fn empty_create<'a>() -> Component<'a> {
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}