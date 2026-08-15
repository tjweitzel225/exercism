use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;
type Callback<'a, T> = Box<dyn FnMut(T) + 'a>;
struct ComputeCell<'a, T> {
    value: Option<T>,
    dependencies: Vec<CellId>,
    compute_func: ComputeFunc<'a, T>,
    callbacks: HashMap<usize, Callback<'a, T>>,
}
pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    input_cells: Vec<T>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.input_cells.push(initial);
        InputCellId(self.input_cells.len() - 1)
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
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(unsatisfied) = dependencies.iter().find(|&&dep| match dep {
            CellId::Input(InputCellId(id)) => id >= self.input_cells.len(),
            CellId::Compute(ComputeCellId(id)) => id >= self.compute_cells.len(),
        }) {
            return Err(*unsatisfied);
        }
        let cell = ComputeCell {
            value: None,
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: HashMap::new(),
        };
        self.compute_cells.push(cell);
        self.update();
        Ok(ComputeCellId(self.compute_cells.len() - 1))
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
            CellId::Input(InputCellId(i)) => self.input_cells.get(i).copied(),
            CellId::Compute(ComputeCellId(i)) => {
                self.compute_cells.get(i).and_then(|cell| cell.value)
            }
        }
    }

    fn update(&mut self) {
        for i in 0..self.compute_cells.len() {
            let dep_vals: Vec<T> = self.compute_cells[i]
                .dependencies
                .iter()
                .map(|&dep| self.value(dep))
                .collect::<Option<_>>()
                .unwrap();
            let new_value = (self.compute_cells[i].compute_func)(&dep_vals);
            if Some(new_value) != self.compute_cells[i].value {
                for cb in &mut self.compute_cells[i].callbacks.values_mut() {
                    cb(new_value)
                }
            }
            self.compute_cells[i].value = Some(new_value);
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
        let ok = match self.input_cells.get_mut(id.0) {
            Some(v) => {
                *v = new_value;
                true
            }
            None => false,
        };
        self.update();
        ok
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
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        self.compute_cells.get_mut(id.0).map(|v| {
            let i = v.callbacks.keys().copied().max().unwrap_or(0) + 1;
            v.callbacks.insert(i, Box::new(callback));
            CallbackId(i)
        })
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
        match self.compute_cells.get_mut(cell.0) {
            Some(cell) => {
                let i = callback.0;
                if i < cell.callbacks.len() {
                    let _ = cell.callbacks.remove(&i);
                    Ok(())
                } else {
                    Err(RemoveCallbackError::NonexistentCallback)
                }
            }
            None => Err(RemoveCallbackError::NonexistentCell),
        }
    }
}
