use std::hash::{Hash, Hasher};

use crate::ops::OperationImpl;

#[derive(Debug)]
pub struct GraphImpl {
    ops: Vec<Box<OperationImpl>>,
}

impl GraphImpl {
    pub fn add_op(&mut self, op_box: Box<OperationImpl>) {
        println!("Add Op id={}", op_box.get_id());
        self.ops.push(op_box);
    }

    pub fn connect(&mut self, src_op_id: usize, dst_op_id: usize) {
        println!("Connect {} to {}", src_op_id, dst_op_id);
    }
}

pub fn create_graph() -> GraphImpl {
    let ops = Vec::new();
    GraphImpl { ops }
}
