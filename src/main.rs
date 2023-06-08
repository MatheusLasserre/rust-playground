

fn main() {
    let sup = vec![1,2,3];
    let mut list = sup
    .iter()
    .map(|x| {
         x + 1
    });
  
    let mut new_vector = vec![];

    while let Some(x) = list.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);

    
    
}
