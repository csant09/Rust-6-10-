fn main() {

    // //Enum
    // //An enum value can be only one of its variants.
    // //i.e. an animal can either be a carn, a herb or an omn; not more than one.
    // #[derive(Debug)]
    // enum Animal{
    //     Carnivore,
    //     Herbivore,
    //     Omnivore,
    // }
    // let tiger = Animal::Carnivore;
    // let cow = Animal::Herbivore;
    // let dog = Animal::Omnivore;
    // println!("{:?} {:?} {:?}",tiger,cow,dog);

    // #[derive(Debug)]
    // enum Faculty{
    //     CSIT(String,i32),
    //     BCA(String,i32),
    //     BBS(String,i32),
    // }
    // let student1 = Faculty::BBS(String::from("Samriddhi College"),3);
    // let student2 = Faculty::BCA(String::from("Deerwalk College"), 2);
    // let student3 = Faculty::CSIT(String::from("Vedas College"), 4);
    // println!("{:?} {:?} {:?}",student1,student2,student3)

    // MATCH, An Enum
    enum Grade {
        Distinction(i32),
        FirstDiv(i32),
        SecondDiv(i32),
        ThirdDiv(i32),
        Fail(i32),
    }
    impl Grade{
        fn calc_grade(&self)->f32{
            match self{
                Grade::Distinction(per) => {
                    if *per>95{
                        println!("WHOA! THAT'S SOME SERIOUS SCORE!!!");
                    }
                    else {
                        println!("YOU GOT A+. CONGRATULATIONS!!!");
                    }
                    4.0
                },
                Grade::FirstDiv(_) => 3.6,
                Grade::SecondDiv(_) => 3.0,
                Grade::ThirdDiv(_) => 2.8,
                Grade::Fail(_) => 1.0,
            }
        }
    }
    let result1 = Grade::Distinction(89);
    let result2 = Grade::FirstDiv(65);
    let result3 = Grade::SecondDiv(54);
    let result4 = Grade::ThirdDiv(49);
    let result5 = Grade::Fail(25);
    println!("{} {} {} {} {}",result1.calc_grade(),result2.calc_grade(),result3.calc_grade(),result4.calc_grade(),result5.calc_grade());


    //OPTION, An Enum
    fn plus_one(number:Option<i32>)->Option<i32>{
        match number{
            Some(i) => Some(i+1),
            None => None,
        }
    }
    let instance1 = plus_one(Some(5));
    let instance2 = plus_one(None);
    println!("{:?} {:?}",instance1,instance2);


    //Problem of match: Exhaustive checking i.e the requirement to define extra cases: None, _
    let a = Some(5);
    match a{
        Some(i)=>println!("{}",i),
        _ =>println!(""),
    }

    //If let solves this problem
    //Note: Don't use == and don't revert the comparision aka a = Some() because compiler might confuse it with assignment
    if let Some(n) = a{
        println!("{}",n);
    }


}
