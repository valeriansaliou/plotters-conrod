// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core as conrod;

const BACKEND_GRAPH_RESIZE_CHUNK: usize = 100;

/// The re-usable graph of Conrod widget IDs, to be re-used for each plot draw (building it is expensive, re-using it is cheap; so build it once and re-use it across loop calls)
pub struct ConrodBackendReusableGraph {
    pub line: ConrodBackendReusableGraphAtom,
    pub rect: ConrodBackendReusableGraphAtom,
    pub path: ConrodBackendReusableGraphAtom,
    pub circle: ConrodBackendReusableGraphAtom,
    pub text: ConrodBackendReusableGraphAtom,
    pub fill: ConrodBackendReusableGraphAtom,
}

pub struct ConrodBackendReusableGraphAtom(conrod::widget::id::List, usize);

impl ConrodBackendReusableGraph {
    /// Build a new Conrod backend re-usable graph of widget identifiers
    ///
    /// **This should be put outside of your drawer loop and built once; failing to do so will result in heavy CPU usage due to the graph being rebuilt for every frame!**
    pub fn build() -> Self {
        Self {
            line: ConrodBackendReusableGraphAtom::new(),
            rect: ConrodBackendReusableGraphAtom::new(),
            path: ConrodBackendReusableGraphAtom::new(),
            circle: ConrodBackendReusableGraphAtom::new(),
            text: ConrodBackendReusableGraphAtom::new(),
            fill: ConrodBackendReusableGraphAtom::new(),
        }
    }

    #[inline(always)]
    pub fn prepare(&mut self) {
        // Notice: destructuring is used there as a safety measure, so that no field is \
        //   forgotten, which could be dangerous (ie. risk of memory leak).
        let Self {
            line,
            rect,
            path,
            circle,
            text,
            fill,
        } = self;

        // Proceed all resets
        line.reset();
        rect.reset();
        path.reset();
        circle.reset();
        text.reset();
        fill.reset();
    }
}

impl ConrodBackendReusableGraphAtom {
    fn new() -> Self {
        Self(conrod::widget::id::List::new(), 0)
    }

    #[inline(always)]
    pub fn next(&mut self, ui: &mut conrod::UiCell) -> conrod::widget::Id {
        // Acquire current index (ie. last 'next index')
        let current_index = self.1;

        // IDs list has not a large-enough capacity for all dynamically-allocated IDs? Enlarge it \
        //   by a pre-defined chunk size (this prevents enlarging the list one by one, requiring \
        //   frequent re-allocations)
        // Notice: this upsizes the graph list to allow current ID to be stored. This is always \
        //   called on the very first call, and may be periodically called whenever the last \
        //   chunked upsize was not enough to store all IDs. This is a trade-off between memory \
        //   and performances.
        if current_index >= self.0.len() {
            self.0.resize(
                self.0.len() + BACKEND_GRAPH_RESIZE_CHUNK,
                &mut ui.widget_id_generator(),
            );
        }

        // Mutate state for next index
        self.1 += 1;

        self.0[current_index]
    }

    #[inline(always)]
    fn reset(&mut self) {
        // Rollback the incremented IDs counter back to zero
        self.1 = 0;
    }
}
