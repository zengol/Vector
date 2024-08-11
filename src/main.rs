fn main() {
    // construct mutable vector
    let mut vector = vec![1, 2, 3];
    //{:?} to row
    println!("vector = {:?}", vector);
    
     // add element to
     // end of vector
     vector.push(100);
     //{:#?} to columns
     println!("vector = {:#?}", vector);

     // remove element from
     // end of element

     vector.pop();
     println!("vector = {:?}", vector);

}
