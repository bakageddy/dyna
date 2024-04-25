use std::io::{BufReader, Read};
use std::path::Path;

use docx_rs::{self, read_docx, TableCell, TableRowChild};

use crate::lexer::IntoText;

pub struct DocxFile<P> {
    pub filename: P,
}

impl<P: AsRef<Path>> DocxFile<P> {
    pub fn new(filename: P) -> Self {
        Self { filename }
    }
}

fn parse_table(table: &docx_rs::Table) -> String {
    let mut content = String::new();
    for row in &table.rows {
        let cells = match row {
            docx_rs::TableChild::TableRow(ref x) => x,
        };
        for cell in &cells.cells {
            match cell {
                TableRowChild::TableCell(TableCell { children: xs, .. }) => {
                    for x in xs {
                        match x {
                            docx_rs::TableCellContent::Paragraph(p) => {
                                content.push_str(&p.raw_text());
                                content.push(' ');
                            }
                            docx_rs::TableCellContent::Table(t) => {
                                content.push_str(&parse_table(&t));
                                content.push(' ');
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    content
}

impl<P: AsRef<Path>> IntoText for DocxFile<P> {
    fn into_text(&mut self) -> Option<String> {
        let mut buf = vec![];
        let mut content = String::new();
        if let Ok(file) = std::fs::File::open(self.get_path()) {
            if let Ok(_) = BufReader::new(file).read_to_end(&mut buf) {
                if let Ok(doc) = read_docx(&buf) {
                    for node in doc.document.children {
                        match node {
                            docx_rs::DocumentChild::Paragraph(ref p) => {
                                content.push_str(&p.raw_text());
                                content.push(' ');
                            }
                            docx_rs::DocumentChild::Table(ref t) => {
                                content.push_str(&parse_table(t));
                                content.push(' ');
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                    return Some(content);
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_path(&self) -> &str {
        self.filename.as_ref().to_str().unwrap()
    }
}
