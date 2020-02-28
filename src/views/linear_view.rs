use crate::{
    event::{
        EventLike,
        KeyEventLike,
        MouseEventLike,
    },
    orientation::Orientation,
    printer::Printer,
    state::RedrawState,
    vec2::Vec2,
    view::View,
    view_wrappers::SizeCacher,
};

pub struct LinearView<S, E, M> {
    children:    Vec<SizeCacher<Box<dyn View<S, E, Message = M> + 'static>>>,
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

    #[inline]
    pub fn orientation(
        mut self,
        orientation: Orientation,
    ) -> Self {
        self.set_orientation(orientation);
        self
    }

    #[inline]
    pub fn set_orientation(
        &mut self,
        orientation: Orientation,
    ) {
        self.orientation = orientation;
    }

    #[inline]
    pub fn child(mut self, 
                 v: impl View<S, E, Message = M> + 'static,) -> Self {
        self.add_child(v);
        self
    }

    #[inline]
    pub fn add_child(
        &mut self,
        v: impl View<S, E, Message = M> + 'static,
    ) {
        self.children.push(SizeCacher::new(Box::new(v)));
    }
}

impl<S: RedrawState, E: EventLike, M> View<S, E> for LinearView<S, E, M> {
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
        mut event: E,
    ) -> Option<Self::Message> {
        if let Some(me) = event.try_mouse_mut() {
            match self.orientation {
                Orientation::Horizontal => {
                    for child in self.children.iter_mut() {
                        let contains = !me.filter_map_pos(|pos| {
                            let size = child.prev_size();
                            if size.x >= pos.x {
                                None
                            } else {
                                Some(pos.sub_x(size.x))
                            }
                        });
                        if contains {
                            return child.on_event(state, event);
                        }
                    }
                }
                Orientation::Vertical => {
                    for child in self.children.iter_mut() {
                        let contains = !me.filter_map_pos(|pos| {
                            let size = child.prev_size();
                            if size.y >= pos.y {
                                None
                            } else {
                                Some(pos.sub_y(size.y))
                            }
                        });
                        if contains {
                            return child.on_event(state, event);
                        }
                    }
                }
            }

            None
        } else if let Some(ke) = event.try_key() {
            if !self.children.is_empty() {
                if self.orientation == Orientation::Horizontal {
                    if ke.try_left() {
                        self.focus = match self.focus {
                            Some(0) => Some(self.children.len() - 1),

                            Some(x) => Some(x - 1),

                            None => Some(0),
                        };
                        state.set_need_redraw(true);
                        return None;
                    } else if ke.try_right() {
                        self.focus = match self.focus {
                            Some(x) if x == self.children.len() - 1 => Some(0),

                            Some(x) => Some(x + 1),

                            None => Some(0),
                        };
                        state.set_need_redraw(true);
                        return None;
                    }
                } else if self.orientation == Orientation::Vertical {
                    if ke.try_up() {
                        self.focus = match self.focus {
                            Some(0) => Some(self.children.len() - 1),

                            Some(x) => Some(x - 1),

                            None => Some(0),
                        };
                        state.set_need_redraw(true);
                        return None;
                    } else if ke.try_down() {
                        self.focus = match self.focus {
                            Some(x) if x == self.children.len() - 1 => Some(0),

                            Some(x) => Some(x + 1),

                            None => Some(0),
                        };
                        state.set_need_redraw(true);
                        return None;
                    }
                }
            }

            if let Some(focus) = self.focus {
                self.children[focus].on_event(state, event)
            } else {
                None
            }
        } else {
            None
        }
    }
}
