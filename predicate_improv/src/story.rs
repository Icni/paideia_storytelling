use std::{collections::HashMap, hash::Hash};

use derive_deref::{Deref, DerefMut};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct TypeName(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atom(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSignature {
    pub name: Symbol,
    pub supertype: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedSymbol {
    pub name: Symbol,
    pub r#type: TypeName,
}

impl Hash for TypedSymbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableOrConstant {
    Variable(TypedSymbol),
    Constant(Symbol),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PredicateSignature {
    /// The function symbol distinguishes different predicates, thus it should be unique.
    pub function: Symbol,
    /// Number of variables in the predicate (f(a,b) != f(a), but f(a) == f(b)).
    pub arity: u32,
}

impl PredicateSignature {
    pub fn new(function: impl Into<Symbol>, arity: u32) -> Self {
        Self {
            function: function.into(),
            arity,
        }
    }
}

#[derive(Debug, Default)]
pub struct PredicateDomain {
    /// Maps the predicate signature to the variable names.
    pub predicates: HashMap<PredicateSignature, Vec<TypedSymbol>>,
    /// Maps the action names to the action data.
    pub actions: HashMap<Symbol, Action>,
    /// Maps constant names to constant types
    pub constants: HashMap<Symbol, TypeName>,
    /// Maps type names to supertypes
    pub types: HashMap<TypeName, Vec<TypeName>>,
}

impl PredicateDomain {
    pub fn generate_story(&self) -> Option<PredicateStory> {
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct StoryState {
    /// All atoms that exist in the story state
    pub atoms: Vec<TypedSymbol>,
    /// All predicates bound here are true, all predicates omitted are false.
    pub bound_predicates: HashMap<PredicateSignature, Vec<Atom>>,
}

#[derive(Debug, Default, Clone)]
pub struct InitialState {
    /// All predicates bound here are true, all predicates omitted are false.
    pub bound_predicates: HashMap<PredicateSignature, Vec<Symbol>>,
}

impl StoryState {
    pub fn get_atom(&self, name: &Symbol) -> Option<Atom> {
        let idx = self.atoms.iter().position(|obj| obj.name == name.clone())?;
        Some(Atom(idx))
    }

    pub fn get_or_insert_atom(&mut self, name: &Symbol, r#type: &TypeName) -> Atom {
        if let Some(idx) = self.atoms.iter().position(|obj| obj.name == name.clone()) {
            Atom(idx)
        } else {
            self.atoms.push(TypedSymbol {
                name: name.clone(),
                r#type: r#type.clone(),
            });
            Atom(self.atoms.len() - 1)
        }
    }

    pub fn get_atom_name(&mut self, atom: Atom) -> Option<&str> {
        self.atoms.get(atom.0).map(|x| x.name.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub enum LogicExpr {
    #[default]
    /// Always true
    True,
    /// The contained predicate is true for the given variables or constants.
    Predicate(PredicateSignature, Vec<Symbol>),
    /// ¬x
    Not(Box<LogicExpr>),
    /// x ∧ y
    And(Box<LogicExpr>, Box<LogicExpr>),
    /// x ∨ y
    Or(Box<LogicExpr>, Box<LogicExpr>),
}

#[derive(Debug, Default, Clone)]
pub struct Action {
    pub parameters: Vec<TypedSymbol>,
    pub precondition: LogicExpr,
    pub effect: LogicExpr,
}

#[derive(Debug, Default, Clone)]
pub struct PredicateProblem {
    /// The maximum number of actions allowed in a story sequence.
    pub max_story_length: u32,
    pub objects: Vec<TypedSymbol>,
    pub initial_state: InitialState,
}

#[derive(Debug, Default)]
pub struct PredicateStory {
    pub text: String,
}
