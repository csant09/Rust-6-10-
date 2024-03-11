fn main() {

    //Enum
    //An enum value can be only one of its variants.
    //i.e. an animal can either be a carn, a herb or an omn; not more than one.
    #[derive(Debug)]
    enum Animal{
        Carnivore,
        Herbivore,
        Omnivore,
    }
    let tiger = Animal::Carnivore;
    let cow = Animal::Herbivore;
    let dog = Animal::Omnivore;
    println!("{:?} {:?} {:?}",tiger,cow,dog);

    #[derive(Debug)]
    enum Faculty{
        CSIT(String,i32),
        BCA(String,i32),
        BBS(String,i32),
    }
    let student1 = Faculty::BBS(String::from("Samriddhi College"),3);
    let student2 = Faculty::BCA(String::from("Deerwalk College"), 2);
    let student3 = Faculty::CSIT(String::from("Vedas College"), 4);
    println!("{:?} {:?} {:?}",student1,student2,student3)


}
