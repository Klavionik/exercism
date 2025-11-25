use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug};
use std::hash::Hash;
use uuid::{Uuid};

/// Represents a cell and its dependencies/dependants.
#[derive(Debug)]
struct Node {
    id: CellId,
    parents: Vec<CellId>,
    children: Vec<CellId>,
}

impl Node {
    pub fn new(id: CellId) -> Self {
        Self { id, parents: vec![], children: vec![] }
    }

    pub fn add_parent(&mut self, node: &Node) {
        self.parents.push(node.id)
    }

    pub fn add_child(&mut self, node: &Node) {
        self.children.push(node.id)
    }
}

/// Performs topological sort on a dependencies tree starting at the given root.
fn topo(nodes: &HashMap<CellId, Node>, root: &Node) -> Vec<CellId> {
    let mut ordered = VecDeque::new();
    let mut visited = HashSet::new();
    let mut in_progress = HashSet::new();

    fn topo_step(nodes: &HashMap<CellId, Node>, root: &Node, o: &mut VecDeque<CellId>, v: &mut HashSet<CellId>, in_p: &mut HashSet<CellId>) {
        if v.contains(&root.id) {
            return
        }

        if in_p.contains(&root.id) {
            panic!("Cycle detected at {:?}", root.id)
        }

        for child in &root.children {
            let node = nodes.get(child).unwrap();
            topo_step(nodes, node, o, v, in_p)
        }

        in_p.remove(&root.id);
        v.insert(root.id);
        o.push_front(root.id);
    }

    topo_step(nodes, root, &mut ordered, &mut visited, &mut in_progress);
    Vec::from(ordered)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T: Copy + PartialEq + Debug> {
    id: InputCellId,
    value: T
}

impl<T: Copy + PartialEq + Debug> InputCell<T> {
    pub fn new(initial: T) -> Self {
        Self {
            id: InputCellId(Uuid::new_v4()),
            value: initial,
        }
    }

    pub fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }
}

struct ComputeCell<'a, T: Copy + PartialEq + Debug> {
    id: ComputeCellId,
    func: fn(&[T]) -> T,
    callbacks: HashMap<CallbackId, Callback<'a, T>>,
    value: T,
    last_value: T,
}

impl<'a, T: Copy + PartialEq + Debug + Debug> ComputeCell<'a, T> {
    pub fn new(func: fn(&[T]) -> T, initials: &[T]) -> Self {
        let value = func(initials);

        Self {
            id: ComputeCellId(Uuid::new_v4()),
            func,
            value,
            last_value: value,
            callbacks: HashMap::new(),
        }
    }

    pub fn add_callback(&mut self, callback: Callback<'a, T>) {
        self.callbacks.insert(callback.id, callback);
    }

    pub fn remove_callback(&mut self, callback_id: &CallbackId) -> bool {
        self.callbacks.remove(callback_id).is_some()
    }

    pub fn update(&mut self, values: &[T]) {
        self.last_value = self.value;
        self.value = (self.func)(values);
        
        if self.last_value != self.value {
            for cb in self.callbacks.values() {
                cb.call(self.value)
            }
        }
    }
}

struct Callback<'a, T: Copy + PartialEq + Debug> {
    id: CallbackId,
    func: Box<dyn Fn(T) + 'a>,
}

impl<'a, T: Copy + PartialEq + Debug> Callback<'a, T> {
    pub fn new<C: Fn(T) + 'a>(func: C) -> Self {
        Self {
            id: CallbackId(Uuid::new_v4()),
            func: Box::new(func),
        }
    }

    pub fn call(&self, value: T) {
        (self.func)(value)
    }
}

pub struct Reactor<'a, T: Copy + PartialEq + Debug> {
    nodes: HashMap<CellId, Node>,
    inputs: HashMap<InputCellId, InputCell<T>>,
    computes: HashMap<ComputeCellId, ComputeCell<'a, T>>
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            inputs: HashMap::new(),
            computes: HashMap::new()
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input = InputCell::new(initial);
        let input_id = input.id;
        let node_id = CellId::Input(input_id);
        let node = Node::new(node_id);
        
        self.nodes.insert(node_id, node);
        self.inputs.insert(input.id, input);
        
        input_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute(
        &mut self,
        dependencies: &[CellId],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellId, CellId> {
        let mut initials = vec![];

        for dep in dependencies {
            let maybe_value = self.value(*dep);
            
            if let Some(value) = maybe_value {
                initials.push(value);
            } else {
                return Err(*dep)
            }
        }

        let compute = ComputeCell::new(compute_func, &initials);
        let compute_id = compute.id;
        let node_id = CellId::Compute(compute_id);
        let mut node = Node::new(node_id);

        for dep in dependencies {
            match self.nodes.get_mut(dep) {
                Some(parent) => {
                    parent.add_child(&node);
                    node.add_parent(parent);
                },
                _ => unreachable!()
            };

        }

        self.computes.insert(compute_id, compute);
        self.nodes.insert(node_id, node);

        Ok(compute_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.inputs.get(&id).map(|cell| cell.value),
            CellId::Compute(id) => self
                .computes
                .get(&id)
                .map(|cell| cell.value)
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(cell) = self.inputs.get_mut(&id) {
            cell.set_value(new_value);
            
            let topo_order = topo(&self.nodes, &self.nodes[&CellId::Input(cell.id)]);

            for node_id in topo_order.iter() {
                let node = &self.nodes[node_id];
                let mut new_values = vec![];

                for parent in &node.parents {
                    let value = self.value(*parent).unwrap();
                    new_values.push(value);
                }

                match node_id {
                    CellId::Compute(id) => {
                        let cell = self.computes.get_mut(id).unwrap();
                        cell.update(&new_values);
                    },
                    CellId::Input(_) => ()
                }
            }
            
            return true
        }

        false
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<C: Fn(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: C,
    ) -> Option<CallbackId> {
        let compute = self.computes.get_mut(&id)?;
        
        let callback = Callback::new(callback);
        let callback_id = callback.id;
        
        compute.add_callback(callback);

        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if !self.computes.contains_key(&cell) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        let compute = self.computes.get_mut(&cell).unwrap();
        
        if !compute.remove_callback(&callback) {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        
        Ok(())
    }
}