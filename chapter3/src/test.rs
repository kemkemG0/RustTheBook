fn  main(){
    let mut cnt =0;
    let result = loop{
        if cnt==10{
            break cnt*100
        }
        cnt+=1;
    };
    println!("{result}");
}