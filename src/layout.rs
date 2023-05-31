use penrose::{
    builtin::layout::{messages::IncMain, MainAndStack},
    core::layout::{Layout, Message},
    pure::{geometry::Rect, Stack},
    Xid,
};

#[derive(Debug, Clone, Copy)]
pub struct ThreeColumn {}

impl ThreeColumn {
    pub fn new() -> Self {
        Self {}
    }
    pub fn boxed() -> Box<dyn Layout> {
        Box::new(Self::new())
    }
}

impl Layout for ThreeColumn {
    fn name(&self) -> String {
        String::from("Three Column")
    }

    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(*self)
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        let n = s.len() as u32;

        // evenly distribute remaining windows across left and right sectiosn
        let windows = if n == 0 {
            vec![]
        } else if n == 1 {
            vec![r]
                .iter()
                .zip(s)
                .map(|(r, c)| (*c, *r))
                .collect::<Vec<_>>()
        } else if n == 2 {
            // behave like two column layout
            let cols = r.as_columns(2);
            cols.into_iter()
                .zip(s)
                .map(|(r, c)| (*c, r))
                .collect::<Vec<_>>()
        } else {
            let cols = r.as_columns(3);
            let left = *cols.get(0).expect("Could not get left area");
            let main = *cols.get(1).expect("Could not get main area");
            let right = *cols.get(2).expect("Could not get left area");

            let left_n = (n.saturating_sub(1) + 1) / 2;
            let right_n = n.saturating_sub(1) / 2;

            vec![main]
                .into_iter()
                .chain(left.as_rows(left_n))
                .chain(right.as_rows(right_n))
                .zip(s)
                .map(|(r, c)| (*c, r))
                .collect::<Vec<_>>()
        };

        (None, windows)
    }

    fn handle_message(&mut self, m: &Message) -> Option<Box<dyn Layout>> {
        /*
        if let Some(&ExpandMain) = m.downcast_ref() {
            todo!()
        } else if let Some(&ShrinkMain) = m.downcast_ref() {
            todo!()
        } else if let Some(&IncMain(n)) = m.downcast_ref() {
            todo!()
        }
        */
        None
    }
}
