use integration_testing::operatons::subtraction::sub;

#[test]
fn a_b(){
    let result = sub(10, 5);
    assert_eq!(result, 5);
}

#[test]
fn b_a(){
    let result = sub(10, 20);
    assert_eq!(result, 10);
}

#[test]
fn eq(){
    let result = sub(10, 10);
    assert_eq!(result, 0);
}

#[test]
fn ueq(){
    let result = sub(100, 50);
    assert_ne!(result, 100);
}


