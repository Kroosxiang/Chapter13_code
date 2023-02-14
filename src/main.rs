use std::collections::hash_map::Values;

#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}


fn main(){
    let mut list = [
        Rectangle{ width: 10, height: 1},
        Rectangle{ width: 3 , height: 5},
        Rectangle{ width: 7 , height: 12},
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("By key called");
    let mut num_sort_opertaion = 0;

    list.sort_by_key(
        |r|{
            // sort_operations.push(value);
            num_sort_opertaion += 1;
            r.width
        });

    println!("{:#?}, sorted in {num_sort_opertaion} opration",list);
}
