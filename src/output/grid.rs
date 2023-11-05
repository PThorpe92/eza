use std::io::{self, Write};

use term_grid as tg;

use crate::fs::filter::FileFilter;
use crate::fs::File;
use crate::output::file_name::{Classify, Options as FileStyle};
use crate::output::file_name::{EmbedHyperlinks, ShowIcons};
use crate::output::{TextCell, TextCellContents};
use crate::theme::Theme;

use super::file_name::QuoteStyle;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Options {
    pub across: bool,
}

impl Options {
    pub fn direction(self) -> tg::Direction {
        if self.across {
            tg::Direction::LeftToRight
        } else {
            tg::Direction::TopToBottom
        }
    }
}

pub struct Render<'a> {
    pub files: Vec<File<'a>>,
    pub theme: &'a Theme,
    pub file_style: &'a FileStyle,
    pub opts: &'a Options,
    pub console_width: usize,
    pub filter: &'a FileFilter,
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(mut self, w: &mut W) -> io::Result<()> {
        self.filter.sort_files(&mut self.files);
        let mut cells: Vec<TextCell> = Vec::new();
        for file in &self.files {
            let filename = self.file_style.for_file(file, self.theme);
            let cell = filename.paint();
            let width = cell.width();
            cells.push(TextCell {
                contents: cell.strings().to_string().into(),
                width,
            });
        }

        let grid = tg::Grid::new(
            cells,
            tg::GridOptions {
                direction: self.opts.direction(),
                filling: tg::Filling::Spaces(2),
                width: self.console_width,
            },
        );
        write!(w, "{grid}")
    }
}
