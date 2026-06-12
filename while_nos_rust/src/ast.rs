// Variable names are strings
pub type VarName = String;

// Arithmetic Expressions (AExp)
#[derive(Debug, Clone)]
pub enum AExp {
    Num(i32),
    Var(VarName),
    Add(Box<AExp>, Box<AExp>),
    Mult(Box<AExp>, Box<AExp>),
    // Section 1
    Sub(Box<AExp>, Box<AExp>),
    Iand(Box<AExp>, Box<AExp>),

    // Section 3
    Shl(Box<AExp>, Box<AExp>),
    Shr(Box<AExp>, Box<AExp>),
}

// Boolean Expressions (BExp)
#[derive(Debug, Clone)]
pub enum BExp {
    True,
    False,
    Aeq(AExp, AExp),
    Beq(Box<BExp>, Box<BExp>),
    Gte(AExp, AExp),
    Neg(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
}

// Statements (Stm)
#[derive(Debug, Clone)]
pub enum Stm {
    Ass(VarName, AExp),
    Skip,
    Comp(Box<Stm>, Box<Stm>),
    If(BExp, Box<Stm>, Box<Stm>),
    While(BExp, Box<Stm>),
    DoWhile(Box<Stm>, BExp),
}







// ----------- Test Cases Functiond  --------
// let test1 = Skip;;
pub fn test1() -> Stm {
    Stm::Skip
}

// let test2 = Comp (Ass ("x", Num 3), Ass ("x", Add(Var "x", Num 1)));;
pub fn test2() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("x".to_string(), AExp::Num(3))),
        Box::new(Stm::Ass(
            "x".to_string(),
            AExp::Add(
                Box::new(AExp::Var("x".to_string())),
                Box::new(AExp::Num(1)),
            ),
        )),
    )
}

// let test3 = If(Neg(Aeq(Var "x", Num 1)),Ass ("x", Num 3),Ass ("x", Num 7));;
pub fn test3() -> Stm {
    Stm::If(
        BExp::Neg(Box::new(BExp::Aeq(
            AExp::Var("x".to_string()),
            AExp::Num(1),
        ))),
        Box::new(Stm::Ass("x".to_string(), AExp::Num(3))),
        Box::new(Stm::Ass("x".to_string(), AExp::Num(7))),
    )
}

/*
let test4 = Comp (Ass("y", Num 1), 
    While(Neg(Aeq(Var "x", Num 0)),
        Comp(Ass("y", Mult(Var "y", Var "x")),
            Ass("x", Sub(Var "x", Num 1))
        )
    )
);;
*/
pub fn test4() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("y".to_string(), AExp::Num(1))),
        Box::new(Stm::While(
            BExp::Neg(Box::new(BExp::Aeq(
                AExp::Var("x".to_string()),
                AExp::Num(0),
            ))),
            Box::new(Stm::Comp(
                Box::new(Stm::Ass(
                    "y".to_string(),
                    AExp::Mult(
                        Box::new(AExp::Var("y".to_string())),
                        Box::new(AExp::Var("x".to_string())),
                    ),
                )),
                Box::new(Stm::Ass(
                    "x".to_string(),
                    AExp::Sub(
                        Box::new(AExp::Var("x".to_string())),
                        Box::new(AExp::Num(1)),
                    ),
                )),
            )),
        )),
    )
}

// section 5

/*
a := 84 ; b := 22 ; c := 0 ; while b != 0 do (
    a := a << 1 ;
    b := b >> 1
)
*/

pub fn test5() -> Stm {
    Stm::Comp(
        // a := 84
        Box::new(Stm::Ass("a".to_string(), AExp::Num(84))),
        // ;
        Box::new(Stm::Comp(
        // b := 22
        Box::new(Stm::Ass("b".to_string(), AExp::Num(22))),
        // ;
        Box::new(Stm::Comp(
        // c := 0
        Box::new(Stm::Ass("c".to_string(), AExp::Num(0))),
        // while 
        Box::new(Stm::While(
            // b != 0
            BExp::Neg(
                Box::new(BExp::Aeq(
                    AExp::Var("b".to_string()),
                    AExp::Num(0),
                )),
            ),
            // body of the while
            Box::new(Stm::Comp(
                // a := a << 1
                Box::new(Stm::Ass(
                    "a".to_string(),
                    AExp::Shl(
                        Box::new(AExp::Var("a".to_string())),
                        Box::new(AExp::Num(1)),
                    ),
                )),
                // b := b >> 1
                Box::new(Stm::Ass(
                    "b".to_string(),
                    AExp::Shr(
                        Box::new(AExp::Var("b".to_string())),
                        Box::new(AExp::Num(1)),
                    ),
                )),
            )),
        )),
        )),
        )),
    )
}

// New TESTS

/*
i := 0 ; n := 5 ; r := 0
do
r := r + 10; i := i + 1;
while i != n
*/

pub fn test6() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("i".to_string(), AExp::Num(0))),
        Box::new(Stm::Comp(
            Box::new(Stm::Ass("n".to_string(), AExp::Num(5))),
            Box::new(Stm::Comp(
                Box::new(Stm::Ass("r".to_string(), AExp::Num(0))),
                Box::new(Stm::DoWhile(
                    Box::new(Stm::Comp(
                        Box::new(Stm::Ass(
                            "r".to_string(),
                            AExp::Add(
                                Box::new(AExp::Var("r".to_string())),
                                Box::new(AExp::Num(10)),
                            ),
                        )),
                        Box::new(Stm::Ass(
                            "i".to_string(),
                            AExp::Add(
                                Box::new(AExp::Var("i".to_string())),
                                Box::new(AExp::Num(1)),
                            ),
                        )),
                    )),
                    BExp::Neg(Box::new(BExp::Aeq(
                        AExp::Var("i".to_string()),
                        AExp::Var("n".to_string()),
                    ))),
                )),
            )),
        )),
    )
}

/*
a := 11 ; b := 0 ;
if (a << 1) >> 1 == 1
    b := b + 1
else
    Skip
*/

pub fn test7() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass("a".to_string(), AExp::Num(11))),
        Box::new(Stm::Comp(
            Box::new(Stm::Ass("b".to_string(), AExp::Num(0))),
            Box::new(Stm::If(
                BExp::Aeq(
                    AExp::Shr(
                        Box::new(AExp::Shl(
                            Box::new(AExp::Var("a".to_string())),
                            Box::new(AExp::Num(1)),
                        )),
                        Box::new(AExp::Num(1)),
                    ),
                    AExp::Num(1),
                ),
                Box::new(Stm::Ass(
                    "b".to_string(),
                    AExp::Add(
                        Box::new(AExp::Var("b".to_string())),
                        Box::new(AExp::Num(1)),
                        ),
                    )),
                Box::new(Stm::Skip),
                )),
        )),
    )
}


/*
a := ((1+3)*2-1) & 13
if a == 5
    b := 1
else
    b := 0
*/
pub fn test8() -> Stm {
    Stm::Comp(
        Box::new(Stm::Ass(
            "a".to_string(),
            AExp::Iand(
                Box::new(AExp::Sub(
                    Box::new(AExp::Mult(
                    Box::new(AExp::Add(
                        Box::new(AExp::Num(1)),
                        Box::new(AExp::Num(3)),
                    )),
                    Box::new(AExp::Num(2)),
                    )),
                Box::new(AExp::Num(1)),
                )),
                Box::new(AExp::Num(13)),
            )),
        ),
        Box::new(Stm::If(
            BExp::Aeq(
                    AExp::Var("a".to_string()),
                    AExp::Num(5),
            ),
            Box::new(Stm::Ass(
                "b".to_string(),
                AExp::Num(1),
            )),
            Box::new(Stm::Ass(
                "b".to_string(),
                AExp::Num(0),
            )),
        )),
    )
}