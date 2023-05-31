use penrose::{
    builtin::layout::messages::IncMain,
    core::layout::{Layout, Message},
    pure::{geometry::Rect, Stack},
    Xid,
};

#[derive(Debug, Clone, Copy)]
pub struct ThreeColumn {}

impl Layout for ThreeColumn {
    fn name(&self) -> String {
        String::from("Three Column")
    }

    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(*self)
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        let n = s.len() as u32;

        let cols = r.as_columns(3);
        let left = *cols.get(0).expect("Could not get left area");
        let main = *cols.get(1).expect("Could not get main area");
        let right = *cols.get(2).expect("Could not get left area");

        // evenly distribute remaining windows across left and right sectiosn
        if n == 0 {
            todo!()
        } else if n == 1 {
            todo!()
        } else if n == 2 {
            todo!()
        } else {
            let left_n = n.saturating_sub(1) / 2;
            let right_n = (n.saturating_sub(1) + 1) / 2;

            let res: Vec<(Xid, Rect)> = vec![main]
                .into_iter()
                .chain(left.as_rows(left_n))
                .chain(right.as_rows(right_n))
                .zip(s)
                .map(|(r, c)| (*c, r))
                .collect();
        }

        todo!()
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
