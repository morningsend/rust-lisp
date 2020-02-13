trait Variable {

}


pub trait Function {
  fn bind(&self, args: Args) -> Box<BoundFunction>;
}

pub trait BoundFunction {

}

pub trait Symbol {
  fn name(&self) -> &str;
}

pub trait List {
   
}