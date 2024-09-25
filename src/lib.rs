
pub mod cal;
#[no_mangle]
pub extern "C" fn add(i: i32, j: i32) -> i32 {
    i + j
}

#[no_mangle]
pub extern "C" fn callback(i: i32, j: i32, f: extern fn(i32, i32) -> i32) -> i32 {
    i + j + f(i, j)
}

#[repr(C)]
pub struct Person {
    pub a: i32,
    pub n: i64,
}

#[repr(C)]
pub union int_or_long {
    pub int_value: i32,
    pub long_value: i64,
}

#[repr(C)]
pub struct TestContainer {
    pub size: i32,
    pub single: Person,
    pub ptr: *mut Person,
    pub union_field: int_or_long,
    pub person_array: [Person; 3],
    pub array_but_pointer: *mut Person,
}

#[no_mangle]
pub extern "C" fn fillPerson(a: i32, n: i64) -> *mut Person {
    Box::leak(Box::new(Person { a, n }))
}

#[no_mangle]
pub unsafe extern "C" fn getA(p: *mut Person) -> i32 {
    (*p).a
}

#[no_mangle]
pub unsafe extern "C" fn getN(p: *mut Person) -> i64 {
    (*p).n
}

#[no_mangle]
pub unsafe extern "C" fn initContainer(size: i32, ptr: *mut Person, union_value: i64) -> *mut TestContainer {
    use std::ptr::{null_mut, read};
    let container_ptr = Box::leak(Box::new(TestContainer {
        size,
        single: read(ptr),
        ptr,
        union_field: int_or_long { long_value: union_value },
        person_array: [read(ptr), read(ptr), read(ptr)],
        array_but_pointer: null_mut(),
    }));
    let size = size as usize;
    let array_ptr = Box::leak(Box::new(Vec::with_capacity(size)));

    for _ in 0..size {
        array_ptr.push(read(ptr));
    }

    container_ptr.array_but_pointer = array_ptr.as_mut_ptr();
    container_ptr
}

//     void setUnionA(TestContainer* container, int value);
//     int getUnionA(TestContainer* container);
//     void setUnionB(TestContainer* container, long value);
//     long getUnionB(TestContainer* container);
//     void setPersonArray(TestContainer* container, Person value, int index);
//     Person getPersonArray(TestContainer* container, int index);
//     void setArrayButPointer(TestContainer* container, Person* value);
//     Person* getArrayButPointer(TestContainer* container);

#[no_mangle]
pub unsafe extern "C" fn setSize(container: *mut TestContainer, value: i32) -> () {
    *(&mut (*container).size) = value;
}

#[no_mangle]
pub unsafe extern "C" fn getSize(container: *mut TestContainer) -> i32 {
    (*container).size
}

#[no_mangle]
pub unsafe extern "C" fn setSingle(container: *mut TestContainer, value: Person) -> () {
    *(&mut (*container).single) = value;
}

#[no_mangle]
pub unsafe extern "C" fn getSingle(container: *mut TestContainer) -> Person {
    std::ptr::read(&(*container).single)
}

