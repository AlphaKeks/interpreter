use {
	super::{builtins, builtins::BuiltinFunction},
	crate::Value,
	std::{cell::RefCell, collections::HashMap, rc::Rc},
};

pub type VariableStore = Rc<RefCell<HashMap<String, Value>>>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Environment {
	pub(super) variables: VariableStore,
	pub(super) outer: Option<Rc<Environment>>,
}

impl Environment {
	pub fn new_global() -> Self {
		let variables = HashMap::from_iter([
			(builtins::Print.name(), Value::BuiltinFunction(Rc::new(builtins::Print))),
			(builtins::Measure.name(), Value::BuiltinFunction(Rc::new(builtins::Measure))),
			(builtins::First.name(), Value::BuiltinFunction(Rc::new(builtins::First))),
		]);

		Self { variables: Rc::new(RefCell::new(variables)), ..Default::default() }
	}

	pub fn with_outer(outer: &Rc<Self>) -> Self {
		Self { outer: Some(Rc::clone(&outer)), ..Default::default() }
	}

	#[tracing::instrument(level = "DEBUG", fields(var = var.as_ref()), ret)]
	pub fn get(&self, var: impl AsRef<str>) -> Value {
		let var = var.as_ref();
		self.variables.borrow().get(var).map_or_else(
			|| match &self.outer {
				None => Value::Null,
				Some(outer) => outer.get(var),
			},
			ToOwned::to_owned,
		)
	}

	pub fn set(&self, name: impl Into<String>, value: impl Into<Value>) -> Value {
		let value = value.into();
		self.variables
			.borrow_mut()
			.entry(name.into())
			.and_modify(|current| *current = value.clone())
			.or_insert(value)
			.to_owned()
	}
}
