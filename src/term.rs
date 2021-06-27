pub trait Term {
    type Substitution;
    type Environment;

    fn has_redex(&self, env: Self::Environment) -> bool;
    fn substitute(sub: &Self::Substitution, args: &[Box<Self>]) -> Self;
    fn reduce(&mut self, env: Self::Environment) -> ();
}
