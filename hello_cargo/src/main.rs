use hello_cargo::get_process_selected;
fn main() {
    //two way to do the same thing
    //using 'use' to import a function
    get_process_selected();
    //not using 'use', passing the relative path would be:
    //(hello_cargo -> project name in Cargo.toml + function name -> greet())
    //hello_cargo::get_process_selected();
}
