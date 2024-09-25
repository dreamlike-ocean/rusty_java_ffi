use dynamic_library::Person;

#[test]
fn test_work() {
    println!("hello test!");
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_add() {
    use dynamic_library::cal::adder::int_add::add;
    assert_eq!(add(2, 2), 5);
}

#[test]
fn test_sub() {
    use dynamic_library::callback;
    let res = callback(1, 2, return_two);
    assert_eq!(res, 5);
}


extern fn return_two(_: i32, _: i32) -> i32 {
    2
}

#[test]
fn test_person() {
    let person = dynamic_library::fillPerson(1, 2);
    unsafe {
        assert_eq!((*person).a, 1);
        assert_eq!((*person).n, 2);

        assert_eq!(dynamic_library::getA(person), 1);
        assert_eq!(dynamic_library::getN(person), 2);
    }

}

#[test]
fn test_container() {
  unsafe {
      let mut original_person_struct = dynamic_library::Person {
          a: 1,
          n: 2,
      };
      let container = dynamic_library::initContainer(3, &mut original_person_struct as *mut dynamic_library::Person, 2);

      assert_eq!((*container).size, 3);
      original_person_struct.a = 10;

      let x = (*container).ptr;
      assert_eq!((*x).a, 10);

      assert_eq!((*container).union_field.long_value, 2);

      assert_eq!((*container).person_array[2].a, 1);

      assert_eq!( (*(*container).array_but_pointer.add(2)).a, 1);

      dynamic_library::setSize(container, 4);
      assert_eq!((*container).size, 4);
      assert_eq!(dynamic_library::getSize(container), 4);

      let new_single = Person {
          a: 10,
          n: 20,
      };

      dynamic_library::setSingle(container, new_single);
      assert_eq!((*container).single.a, 10);

      let new_single = dynamic_library::getSingle(container);
      let x1 = &mut (*container).single;
      x1.a = 20;
      assert_eq!(new_single.a, 10)
  }
}

