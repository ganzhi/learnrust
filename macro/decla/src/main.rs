macro_rules! ok_or_return{
    // match something(q,r,t,6,7,8) etc
    // compiler extracts function name and arguments. It injects the values in respective varibles.
        ($a:ident($($b:tt)*))=>{
           {
            match $a($($b)*) {
                Ok(value)=>value,
                Err(err)=>{
                    return Err(err);
                }
            }
            }
        };
    }
    
    fn some_work(i:i64,j:i64)->Result<(i64,i64),String>{
        if i+j>2 {
            Ok((i,j))
        } else {
            Err("error".to_owned())
        }
    }
    
    fn main()->Result<(),String>{
        ok_or_return!(some_work(1,4));
        ok_or_return!(some_work(1,0));
        Ok(())
    }