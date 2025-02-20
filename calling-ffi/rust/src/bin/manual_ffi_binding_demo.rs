#[path = "../manual_bindings.rs"]
mod manual_bindings;

use manual_bindings::root::Demo::{
    create_new_person, get_person_info, print_helloworld, print_person_info, Gender_Male, Location,
    Person,
};
use std::ffi::{CStr, CString};

use crate::manual_bindings::root::Demo::release_person_pointer;

///
fn main() -> () {
    println!("[ Manual FFI bindgins call demo ]\n");

    // helloworld
    unsafe {
        print_helloworld();
    }

    // `Person` related
    let john_first_name = CString::new("John").unwrap();
    let john_last_name = CString::new("Ye").unwrap();
    let john_street = CString::new("My street").unwrap();
    let john_city = CString::new("My city").unwrap();
    let john_state = CString::new("My state").unwrap();
    let john_country = CString::new("My country").unwrap();
    let john_location = Location {
        street_address: john_street.as_ptr(),
        city: john_city.as_ptr(),
        state: john_state.as_ptr(),
        country: john_country.as_ptr(),
    };

    unsafe {
        // Get back `Person` raw pointer
        let john: *mut Person = create_new_person(
            john_first_name.as_ptr(),
            john_last_name.as_ptr(),
            Gender_Male,
            18,
            john_location,
        );

        let temp_first_name: String = CStr::from_ptr((*john).first_name)
            .to_string_lossy()
            .into_owned();
        let temp_last_name: String = CStr::from_ptr((*john).last_name)
            .to_string_lossy()
            .into_owned();
        let temp_street: String = CStr::from_ptr((*john).location.street_address)
            .to_string_lossy()
            .into_owned();
        let temp_country: String = CStr::from_ptr((*john).location.country)
            .to_string_lossy()
            .into_owned();

        // Customize print `Person` instance
        let john_info = format!(
        "\n>>> Customized print >>>\n[ John Info ]:\nFirst name: {}\nLast name: {}\nLocation:\n\tStreet: {}\n\tCountry: {}\n",
        temp_first_name, temp_last_name, temp_street, temp_country
        );
        println!("{}", john_info);

        // `print_person_info` print
        println!(">>> 'print_person_info' print >>>");
        print_person_info(john);

        // `get_person_info` print
        let john_info_from_c_string: String = CStr::from_ptr(get_person_info(john))
            .to_string_lossy()
            .into_owned();
        println!(">>> 'get_person_info' print >>>{}", john_info_from_c_string);

        // Remember to free the instance
        release_person_pointer(john);

        // Want to try double free? :)
        // release_person_pointer(john);
    }
}
