
fn main() {

}
#[test]
fn slice_reference() {
    let array : [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let slice1 : &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2 : &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3 : &[i32] = &array[5..];
    println!("{:?}", slice3);
}