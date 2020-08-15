// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub trait OperationProgressListener {
    fn on_progress(&self, desc: &str, cur: usize, total: usize);
}
