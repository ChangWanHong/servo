/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{DOMString, ErrorResult};
use dom::htmlelement::HTMLElement;

pub struct HTMLTableColElement {
    parent: HTMLElement,
}

impl HTMLTableColElement {
    pub fn Span(&self) -> u32 {
        0
    }

    pub fn SetSpan(&mut self, _span: u32, _rv: &mut ErrorResult) {
    }

    pub fn Align(&self) -> DOMString {
        None
    }

    pub fn SetAlign(&mut self, _align: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Ch(&self) -> DOMString {
        None
    }

    pub fn SetCh(&mut self, _ch: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn ChOff(&self) -> DOMString {
        None
    }

    pub fn SetChOff(&mut self, _ch_off: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn VAlign(&self) -> DOMString {
        None
    }

    pub fn SetVAlign(&mut self, _v_align: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Width(&self) -> DOMString {
        None
    }

    pub fn SetWidth(&mut self, _width: &DOMString, _rv: &mut ErrorResult) {
    }
}
