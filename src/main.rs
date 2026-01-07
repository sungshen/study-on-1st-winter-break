fn find<T:PartialEq>(n:T,array:&Vec<T>) -> bool{
    for s in array{
        if *s == n{
            return true;
        }
    }
    return false;
    
}

fn main() {
    let number = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{}",find(1,&number));
    println!("{}",find(11,&number));
}
