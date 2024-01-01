use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct ChoiceItem {
    content: String,
    uuid: Uuid,
    style: Style,
}

impl ChoiceItem {
    pub fn new(content: String, uuid: Uuid) -> Self {
        Self {
            content,
            uuid,
            style: Style::default(),
        }
    }
}

impl Styled for ChoiceItem {
    type Item = ChoiceItem;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}

#[derive(Debug, Default)]
pub struct ChoiceSelectorState {
    row: usize,
    col: usize,
    items: Vec<Vec<ChoiceItem>>,
    selected: HashSet<Uuid>,
}

impl ChoiceSelectorState {
    pub fn new(items: Vec<Vec<ChoiceItem>>) -> Self {
        Self {
            row: 0,
            col: 0,
            items,
            selected: HashSet::new(),
        }
    }

    // place the cursor to the last item in the row if it is out of bounds
    // useful when moving up/down and the rows dont have the same ammount of items
    fn normalize_cursor(&mut self) {
        let row_len = self.items[self.row].len();
        if self.col >= row_len {
            self.col = row_len - 1
        }
    }

    pub fn move_up(&mut self) {
        if self.row == 0 {
            self.row = self.items.len() - 1;
        } else {
            self.row -= 1;
        }

        self.normalize_cursor();
    }

    pub fn move_down(&mut self) {
        self.row = (self.row + 1) % self.items.len();

        self.normalize_cursor();
    }

    pub fn move_left(&mut self) {
        let row_len = self.items[self.row].len();
        if self.col == 0 {
            self.col = row_len - 1;
        } else {
            self.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        let row_len = self.items[self.row].len();
        self.col = (self.col + 1) % row_len;
    }

    // get selected answers as vector
    pub fn selected(&self) -> Vec<Uuid> {
        self.selected.clone().into_iter().collect()
    }

    pub fn toggle_selection(&mut self) {
        let item = self.items[self.row][self.col].uuid;

        if self.selected.contains(&item) {
            self.selected.remove(&item);
        } else {
            self.selected.insert(item);
        }
    }
}

#[derive(Default)]
pub struct ChoiceSelector<'a> {
    // the items that are gonna be displayed are part of the ChoiceSelectorState
    // we can handle input and moving around the grind more easily that way
    pub block: Option<Block<'a>>,
    pub current_item_style: Style,
    pub selected_item_style: Style,
}

impl<'a> ChoiceSelector<'a> {
    pub fn new() -> Self {
        Self {
            block: None,
            current_item_style: Style::default().italic(),
            selected_item_style: Style::default().bold(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn current_item_style(mut self, style: Style) -> Self {
        self.current_item_style = style;
        self
    }

    pub fn selected_item_style(mut self, style: Style) -> Self {
        self.selected_item_style = style;
        self
    }
}

impl<'a> StatefulWidget for ChoiceSelector<'a> {
    type State = ChoiceSelectorState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let choice_selector_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let items = &mut state.items;

        let item_height = choice_selector_area.height / items.len() as u16;
        let (x, y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in items.iter_mut().enumerate() {
            let item_width = choice_selector_area.width / row.len() as u16;

            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    x + j as u16 * item_width,
                    y + i as u16 * item_height,
                    item_width,
                    item_height,
                );

                let mut style = item.style;
                if state.row == i && state.col == j {
                    style = style.patch(self.current_item_style);
                }
                if state.selected.contains(&item.uuid) {
                    style = style.patch(self.selected_item_style);
                }

                Paragraph::new(item.content.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .style(style)
                    .wrap(Wrap { trim: true })
                    .render(area, buf);
            }
        }
    }
}