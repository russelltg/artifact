/* artifact: the requirements tracking tool made for developers
 * Copyright (C) 2018  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */
#![allow(dead_code)]
pub use chrono::prelude::*;
pub use yew::prelude::*;
pub use yew::services::console::ConsoleService;
pub use artifact_lib::*;
pub use ergo_std::*;
pub use ergo_config::*;
pub use path_abs::*;

pub(crate) type HtmlApp = Html<Context, Model>;

pub(crate) struct Context {}

// http://basscss.com/

// Types
pub(crate) const H1: &str = "h1";
pub(crate) const H2: &str = "h2";
pub(crate) const H3: &str = "h3";
pub(crate) const BTN: &str = "btn";
pub(crate) const INPUT: &str = "input";

// Styles
pub(crate) const REGULAR: &str = "regular";
pub(crate) const BOLD: &str = "bold";
pub(crate) const ITALIC: &str = "italic";
pub(crate) const BORDER: &str = "border";

// Alignment
pub(crate) const LEFT: &str = "left";
pub(crate) const LEFT_ALIGN: &str = "left-align";
pub(crate) const RIGHT: &str = "right";

// Controlling columns
pub(crate) const CLEARFIX: &str = "clearfix";

// Padding: top/bottom/right/left + x/y axis
pub(crate) const P1: &str = "p1";
pub(crate) const PY1: &str = "py1";
pub(crate) const PX1: &str = "py1";

// margin right/left/top/bottom + x/y axis
pub(crate) const MT1: &str = "mt1";
pub(crate) const MR1: &str = "mr1";
pub(crate) const MB1: &str = "mb1";
pub(crate) const ML1: &str = "ml1";
pub(crate) const MX1: &str = "mx1";
pub(crate) const MY1: &str = "my1";

pub(crate) const MB2: &str = "mb2";


// Colors
pub(crate) const ACE_WHITE: &str = "white";
pub(crate) const ACE_GRAY: &str = "gray";
pub(crate) const ACE_BG_BLACK: &str = "bg-black";

pub(crate) const GRAY: &str = "#DCDEE2";
pub(crate) const OLIVE: &str = "#3DA03D";
pub(crate) const BLUE: &str = "#0074D9";
pub(crate) const ORANGE: &str = "#FF851B";
pub(crate) const RED: &str = "#FF4136";
pub(crate) const PURPLE: &str = "#B10DC9";

// Column controls == must add to 12 in view
pub(crate) const COL: &str = "col";
pub(crate) const COL_1: &str = "col-1";
pub(crate) const COL_2: &str = "col-2";
pub(crate) const COL_3: &str = "col-3";
pub(crate) const COL_4: &str = "col-4";
pub(crate) const COL_5: &str = "col-5";
pub(crate) const COL_6: &str = "col-6";
pub(crate) const COL_7: &str = "col-7";
pub(crate) const COL_10: &str = "col-10";

// Responsive Columns
pub(crate) const SM_COL: &str = "sm-col";
pub(crate) const SM_COL_2: &str = "sm-col-2";
pub(crate) const SM_COL_3: &str = "sm-col-3";
pub(crate) const SM_COL_6: &str = "sm-col-6";
pub(crate) const SM_COL_8: &str = "sm-col-8";
pub(crate) const SM_COL_11: &str = "sm-col-11";
pub(crate) const SM_COL_12: &str = "sm-col-12";

pub(crate) const MD_COL_3: &str = "md-col-3";
pub(crate) const MD_COL_4: &str = "md-col-4";
pub(crate) const MD_COL_5: &str = "md-col-5";
pub(crate) const MD_COL_6: &str = "md-col-6";
pub(crate) const MD_COL_7: &str = "md-col-7";
pub(crate) const MD_COL_8: &str = "md-col-8";
pub(crate) const MD_COL_12: &str = "md-col-12";

pub(crate) const LG_COL_2: &str = "lg-col-2";
pub(crate) const LG_COL_3: &str = "lg-col-3";
pub(crate) const LG_COL_4: &str = "lg-col-4";
pub(crate) const LG_COL_5: &str = "lg-col-5";
pub(crate) const LG_COL_6: &str = "lg-col-6";
pub(crate) const LG_COL_7: &str = "lg-col-7";
pub(crate) const LG_COL_8: &str = "lg-col-8";
pub(crate) const LG_COL_9: &str = "lg-col-9";
pub(crate) const LG_COL_10: &str = "lg-col-10";
pub(crate) const LG_COL_12: &str = "lg-col-12";

// Font Awesome
pub(crate) const FA: &str = "fas";
pub(crate) const FA_INFO_CIRCLE: &str = "fa-info-circle";
pub(crate) const FA_EYE: &str = "fa-eye";
pub(crate) const FA_FLOPPY_O: &str = "fa-floppy-o";
pub(crate) const FA_PLUS_SQUARE: &str = "fa-plus-square";
pub(crate) const FA_SEARCH: &str = "fa-search";
pub(crate) const FA_SEARCH_PLUS: &str = "fa-search-plus";
pub(crate) const FA_SEARCH_MINUS: &str = "fa-search-minus";
pub(crate) const FA_EXCLAMATION: &str = "fa-exclamation";
pub(crate) const FA_EXCLAMATION_CIRCLE: &str = "fa-exclamation-circle";
pub(crate) const FA_TRASH: &str = "fa-trash";
pub(crate) const FA_TIMES: &str = "fa-times";

// Custom
pub(crate) const ART_INFO: &str = "art-info";
pub(crate) const SELECT_TINY: &str = "select-tiny";

#[derive(Debug, Clone)]
pub(crate) enum View {
    Graph,
    Artifact(Name),
}

pub(crate) struct Model {
    pub(crate) shared: Arc<ProjectSer>,
    pub(crate) view: View,
    pub(crate) router: ::yew_router::RouterTask<Context, Model>,
    pub(crate) nav: Nav,
}

pub(crate) enum Msg {
    SetView(View),
    ToggleSearch,
    SetSearch(String),
    Ignore,
}

#[derive(Debug, Default, Clone)]
/// Navigation bar
pub(crate) struct Nav {
    pub(crate) search: Search,
}

#[derive(Debug, Default, Clone)]
/// Search settings
pub(crate) struct Search {
    pub(crate) on: bool,
    pub(crate) value: String,
}

impl Search {
    pub(crate) fn with_on(self, on: bool) -> Self {
        Self {
            on: on,
            value: self.value,
        }
    }
}

pub(crate) trait CompletedExt {
    fn spc_html(&self) -> HtmlApp;
    fn tst_html(&self) -> HtmlApp;
    fn name_color(&self) -> &'static str;
}
