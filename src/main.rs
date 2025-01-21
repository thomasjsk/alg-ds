use crate::two_crystal_balls::two_crystal_balls;

mod binary_search_list;
mod bubble_sort;
mod doubly_linked_list;
mod linear_search;
mod queue;
mod stack;
mod two_crystal_balls;

fn main() {
    // println!(
    //     ">>> binary_search_list: {}",
    //     binary_search_list::binary_search_list([1, 2, 3, 4, 5, 6, 7, 12, 13, 33], 50)
    // )
    println!(
        ">>>> {}",
        two_crystal_balls(&[false, false, false, false, false, false, false, true])
    )
}
