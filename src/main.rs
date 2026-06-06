mod array;
use array::Array;

fn main(){
    let a = Array::<i32>::new(1, 2, vec![1, 2]).unwrap();

    let shape: Vec<usize> = a.shape();
    let mut formatted_shape: String = Default::default();
    for (index, s) in shape.clone().into_iter().enumerate(){
        formatted_shape += &(s.to_string());
        if index != shape.len() - 1 {
            formatted_shape += ", ";
        }
    }

    println!("{}", formatted_shape);
}