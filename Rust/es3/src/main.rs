

fn main() {
   let expr = Sum{
       left: Box::new(Product{
           left: Box::new(Literal{value: 2.0}),
           right: Box::new(Literal{value: 3.0})
       }),
       right: Box::new(Literal{value: 4.0})
   };
   println!("{}", expr.eval());
}

/* 
    Reprise this exercise
        https://tomamic.github.io/p10-collezioni.html#/14
        https://tomamic.github.io/pyodide/?p10_expression.py
    Solve it using a Rust enum
        Represent the expression in the picture as a tree
        Evaluate it
    Optionally, parse a user's string as a prefix expression
        https://tomamic.github.io/pyodide/?p10_parse.py
 */

/* pub enum Expression{
    Literal(f64),
    Product(Box<Expression>, Box<Expression>),
    Sum(Box<Expression>, Box<Expression>)
}
impl Expression{
    pub fn eval(&self)->f64{
        match self{
            Expression::Literal(x) => *x,
            Expression::Product(a, b) => a.eval() * b.eval(),
            Expression::Sum(a, b) => a.eval() + b.eval()
        }
    }
} */
/* 
    Reprise this exercise
        https://tomamic.github.io/p10-collezioni.html#/14
        https://tomamic.github.io/pyodide/?p10_expression.py
    Solve it using a Rust trait
        Represent the expression in the picture as a tree
        Evaluate it
    Optionally, parse a user's string as a prefix expression
        https://tomamic.github.io/pyodide/?p10_parse.py
 */
pub trait Expression{
    fn eval(&self)->f64;
}
pub struct Literal{
    value: f64
}
impl Expression for Literal{
    fn eval(&self)->f64{
        self.value
    }
}
pub struct Product{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>
}
impl Expression for Product{
    fn eval(&self)->f64{
        self.left.eval() * self.right.eval()
    }
}

pub struct Sum{
    left: Box<dyn Expression>,
    right: Box<dyn Expression>
}

impl Expression for Sum{
    fn eval(&self)->f64{
        self.left.eval() + self.right.eval()
    }
}



    
