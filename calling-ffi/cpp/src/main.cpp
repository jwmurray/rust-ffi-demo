#include "ffi.h"
#include <iostream>

using namespace std;

int main() {
    //
    // Call FFI functions
    //

    const char *first_name = "Murray";
    const char *last_name = "John";
    const char *street_address = "`son's street_address here";
    const char *city = "John's city here";
    const char *state = "John's state here";
    const char *country = "John's country here";
    person_t *John = create_new_person(
        first_name,
        last_name,
        1,
        88,
        street_address,
        city,
        state,
        country
    );

    print_person_info(John);

    char *person_info_ptr = get_person_info(John);
    cout << "\n>>> C++ caller print >>>\n" << person_info_ptr << "\n\n";
    release_get_person_info(person_info_ptr);
    
    release_person_pointer(John);
    
    return 0;
}
