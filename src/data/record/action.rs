use super::super::Editable;
use std::mem;

#[derive(Clone, Debug)]
pub enum Action {
    Insert(String),
    InsertForward(String),
    Delete(String),
    Move(isize),
    DeleteForward(String),
}

impl Action {
    pub fn apply<T: Editable>(&self, content: &mut T) {
        match *self {
            Action::Insert(ref s) => {
                for c in s.chars() {
                    content.insert(c);
                }
            }
            Action::Delete(ref s) => {
                for _ in s.chars() {
                    content.delete();
                }
            }
            Action::Move(rel) => {
                let new_position = rel + content.pos() as isize;
                content.move_to(new_position as usize);
            }
            Action::DeleteForward(ref s) => {
                for _ in s.chars() {
                    content.delete_forward();
                }
            }
            Action::InsertForward(ref s) => {
                for c in s.chars() {
                    content.insert_forward(c);
                }
            }
        };
    }

    pub fn invert(&self) -> Action {
        match *self {
            Action::Insert(ref s) => Action::Delete(s.clone()),
            Action::Delete(ref s) => Action::Insert(s.clone()),
            Action::Move(ref rel) => Action::Move(-rel),
            Action::InsertForward(ref s) => Action::DeleteForward(s.clone()),
            Action::DeleteForward(ref s) => Action::InsertForward(s.clone()),
        }
    }

    pub fn join(&mut self, act: Action) {
        assert!(self.same_variant(&act));
        match *self {
            Action::Insert(ref mut s) => {
                let act_string = match act {
                    Action::Insert(a) => a,
                    _ => panic!("Trying to join dissimilar Actions"),
                };
                s.push_str(&act_string);
            }
            Action::InsertForward(ref mut s) => {
                let act_string = match act {
                    Action::InsertForward(a) => a,
                    _ => panic!("Trying to join dissimilar Actions"),
                };
                s.push_str(&act_string);
            }
            Action::Delete(ref mut s) => {
                let mut act_string = match act {
                    Action::Delete(a) => a,
                    _ => panic!("Trying to join dissimilar Actions"),
                };
                act_string.push_str(s);
                *s = act_string;
            }
            Action::DeleteForward(ref mut s) => {
                let mut act_string = match act {
                    Action::DeleteForward(a) => a,
                    _ => panic!("Trying to join dissimilar Actions"),
                };
                act_string.push_str(s);
                *s = act_string;
            }
            Action::Move(ref mut rel) => {
                let act_rel = match act {
                    Action::Move(a) => a,
                    _ => panic!("Trying to join dissimilar Actions"),
                };
                *rel += act_rel;
            }
        }
    }

    pub fn same_variant(&self, other: &Action) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
