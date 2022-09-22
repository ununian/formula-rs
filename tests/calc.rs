use formula;


#[test]
fn it_adds_two() {
    let result = formula::parse("COUNT(relationship;relationship=CHILD)").unwrap();
    println!("{:?}", result);
    // println!("{:?}", formula::eval(result));
}