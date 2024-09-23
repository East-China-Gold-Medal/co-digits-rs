use num_traits::PrimInt;

pub fn increment_one<T>(number: T) -> T 
where T: PrimInt{
    number + T::from(1).expect("T from 1 panic")
}