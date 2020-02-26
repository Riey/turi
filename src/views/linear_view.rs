use crate::{
    event::{
        EventLike,
    },
    orientation::Orientation,
    printer::Printer,
    vec2::Vec2,
    view::{
        View,
    },
    view_wrappers::BoundChecker,
};

pub struct LinearView<S, E, M> {
    children:    Vec<BoundChecker<Box<dyn View<S, E, Message = M> + 'static>>>,
    orientation: Orientation,
    focus:       Option<usize>,
}

impl<S, E, M> LinearView<S, E, M> {
    pub fn new() -> Self {
        Self {
            children:    Vec::with_capacity(10),
            orientation: Orientation::Horizontal,
            focus:       None,
        }
    }

    pub fn with_orientation(
        mut self,
        orientation: Orientation,
    ) -> Self {
        self.set_orientation(orientation);
        self
    }

    pub fn set_orientation(
        &mut self,
        orientation: Orientation,
    ) {
        self.orientation = orientation;
    }

    pub fn add_child(
        &mut self,
        v: impl View<S, E, Message = M> + 'static,
    ) {
        self.children.push(BoundChecker::new(Box::new(v)));
    }
}

impl<S, E: EventLike, M> View<S, E> for LinearView<S, E, M> {
    type Message = M;

    fn render(
        &self,
        printer: &mut Printer,
    ) {
        match self.orientation {
            Orientation::Horizontal => {
                let mut x = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((x, 0)), |printer| {
                        child.render(printer)
                    });
                    x += child.prev_size().x;
                }
            }
            Orientation::Vertical => {
                let mut y = 0;
                for child in self.children.iter() {
                    printer.with_bound(printer.bound().add_start((0, y)), |printer| {
                        child.render(printer);
                    });
                    y += child.prev_size().y;
                }
            }
        }
    }

    fn desired_size(&self) -> Vec2 {
        match self.orientation {
            Orientation::Vertical => {
                self.children
                    .iter()
                    .map(|c| c.desired_size())
                    .fold(Vec2::new(0, 0), |acc, x| {
                        Vec2::new(acc.x.max(x.x), acc.y + x.y)
                    })
            }
            Orientation::Horizontal => {
                self.children
                    .iter()
                    .map(|c| c.desired_size())
                    .fold(Vec2::new(0, 0), |acc, x| {
                        Vec2::new(acc.x + x.x, acc.y.max(x.y))
                    })
            }
        }
    }

    fn layout(
        &mut self,
        mut size: Vec2,
    ) {
        for child in self.children.iter_mut() {
            let child_size = child.desired_size();
            child.layout(size.min(child_size));

            match self.orientation {
                Orientation::Vertical => {
                    size = size.saturating_sub_y(child_size.y);
                }
                Orientation::Horizontal => {
                    size = size.saturating_sub_x(child_size.x);
                }
            }
        }
    }

    fn on_event(
        &mut self,
        state: &mut S,
        event: E,
    ) -> Option<Self::Message> {
        if let Some(pos) = event.try_mouse() {
            for child in self.children.iter_mut() {
                if child.contains(pos) {
                    return child.on_event(state, event);
                }
            }

            None
        } else {
            if let Some(focus) = self.focus {
                self.children[focus].on_event(state, event)
            } else {
                None
            }
        }
    }
}
