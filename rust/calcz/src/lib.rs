use std::error::Error;
use std::io::Read;
use std::process;

#[derive(PartialEq, Debug)]
enum Operation {
    SUM,
    SUB,
    MLT,
    DIV
}

trait Operable {
    fn compute(&self) -> f64;
    fn display(&self) -> String;
}

impl Operable for Input {//somehow feels weird to implement this on the struct instead of the operation...
    fn compute(&self) -> f64 {
        match self.op {
            Operation::SUM =>  self.n1 + self.n2,
            Operation::SUB =>  self.n1 - self.n2,
            Operation::MLT =>  self.n1 * self.n2,
            Operation::DIV =>  self.n1 / self.n2,
        }
    }

    fn display(&self) -> String {
        match self.op {
            Operation::SUM => format!("{} + {}", self.n1,  self.n2),
            Operation::SUB => format!("{} - {}", self.n1,  self.n2),
            Operation::MLT => format!("{} * {}", self.n1,  self.n2),
            Operation::DIV => format!("{} / {}", self.n1,  self.n2),
        }
    }
}

struct Input {
    op: Operation,
    n1: f64,
    n2: f64
}

impl Input {
    fn new(op: Operation, n1: f64, n2: f64) -> Input {
        Input { op, n1, n2 }
    }
}


//I want to refactor this into 
//equation calculator instead :)
//however will save it for later (for now..)
pub fn run () -> Result<(), Box<Error>> {

    loop { 
        println!("hint: type q to exit");
        println!("Choose an operation");
        println!("+ , - , * or /");

        let input = 'q';   

        /*
            Will continue later, 

            TODO:
                1. Read one char at a time
                2. Match operation
                3. read number input
                4. display result
                5. refactor into equation

        */

        let input: Option<char> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char);

        println!("{:?}", input);

        if input == Some('q') {
            break
        }
    }


    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_input () {

        let i = Input::new(Operation::SUM, 10f64, 15f64);
        assert_eq!(i.op, Operation::SUM);
        assert_eq!(i.n1, 10f64);
        assert_eq!(i.n2, 15f64);
    }

    #[test]
    fn sum_operation () {

        let i = Input::new(Operation::SUM, 10f64, 15f64);
        assert_eq!(i.op, Operation::SUM);
        assert_eq!(i.n1, 10f64);
        assert_eq!(i.n2, 15f64);

        assert_eq!(25f64, i.compute());
        assert_eq!("10 + 15", i.display());

    }

    #[test]
    fn sub_operation () {

        let i = Input::new(Operation::SUB, 10f64, 15f64);
        assert_eq!(i.op, Operation::SUB);
        assert_eq!(i.n1, 10f64);
        assert_eq!(i.n2, 15f64);

        assert_eq!(-5f64, i.compute());
        assert_eq!("10 - 15", i.display());

    }


    #[test]
    fn mlt_operation () {

        let i = Input::new(Operation::MLT, 10f64, 15f64);
        assert_eq!(i.op, Operation::MLT);
        assert_eq!(i.n1, 10f64);
        assert_eq!(i.n2, 15f64);

        assert_eq!(150f64, i.compute());
        assert_eq!("10 * 15", i.display());
    }


    #[test]
    fn div_operation () {

        let i = Input::new(Operation::DIV, 10f64, 15f64);
        assert_eq!(i.op, Operation::DIV);
        assert_eq!(i.n1, 10f64);
        assert_eq!(i.n2, 15f64);

        assert_eq!(10f64/15f64, i.compute());
        assert_eq!("10 / 15", i.display());

    }

}

