fn fizzbuzz(n:usize) {
    fn do(v:usize) {
        if v == 1 {
            break
        }else{
            let s =  match (v % 3, v % 5){
                (0,0) => "fizzbuzz",
                (_,0) => "buzz",
                (0,_) => "fizz",
                 _ => "no match"
                }
        println!("{}",s);
        do(v-1)
        }
    }
    do(n)
}
