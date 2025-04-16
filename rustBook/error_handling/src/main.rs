/* ERROR HANDLING
Type Result<T, E> for recoverable errors 
Panic! macro that stops execution for unrecoverable error. 

This chapter covers calling panic! first and then talks about 
returning Result<T, E> values. 

Additionally, we’ll explore considerations when deciding whether to try 
to recover from an error or to stop execution.
*/

fn main() {
    println!("Hello, world!");
}
