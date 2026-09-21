pub fn factors(n: u64) -> Vec<u64> {
    /*let mut local_n = n;
    let mut vec_res = Vec::<u64>::new();
    let mut cand = 2;
    while cand * cand <= local_n{
        if local_n%cand==0
        {
            local_n/=cand;
            vec_res.push(cand);
        }
        else
        {
            cand+=1;
        }
    }
    if local_n !=1{
        vec_res.push(local_n);
    }
    
    vec_res*/

    let mut local_n = n;
    let mut cand: u64 = 2;
    std::iter::from_fn(
        || 
            {
                loop
                    {
                        if local_n==1
                        {
                            return None;
                        }
                        if cand * cand > local_n
                        {
                            return Some(std::mem::replace(&mut local_n, 1));
                        }
                        if local_n%cand==0
                        {
                            local_n/=cand; 
                            return Some(cand);
                        }
                        cand+=1;
                    }
            }
        ).collect()
}
