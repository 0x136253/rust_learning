fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [usize; 5] = [1,2,3,4,5];
    for elem in a {
        print!("the No.{} month is {}\n",elem,months[elem-1]);
    }
}
