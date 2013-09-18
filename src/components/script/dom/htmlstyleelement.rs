/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{DOMString, ErrorResult};
use dom::htmlelement::HTMLElement;

pub struct HTMLStyleElement {
    parent: HTMLElement,
}

impl HTMLStyleElement {
    pub fn Disabled(&self) -> bool {
        false
    }

    pub fn SetDisabled(&self, _disabled: bool) {
    }

    pub fn Media(&self) -> DOMString {
        None
    }

    pub fn SetMedia(&mut self, _media: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Type(&self) -> DOMString {
        None
    }

    pub fn SetType(&mut self, _type: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Scoped(&self) -> bool {
        false
    }

    pub fn SetScoped(&self, _scoped: bool, _rv: &mut ErrorResult) {
    }
}
