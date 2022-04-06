use std::collections::HashSet;

fn prefers(prefer: Vec<Vec<i32>>, w: i32, m: i32, m1: i32, n: u32) -> bool {
    for i in 0..n {

        // m1 comes before m in list of w,
        // then w prefers her current engagement,
        // dont do anything
        if(prefer.get(w).get(i) == m1) {
            return true;
        }

        // if m comes before m1 in w's list,
        // then free her current engagement
        // and engage her with m
        if (prefer.get(w).get(i) == m) {
            return false;
        }
    }
    return false;
}

fn gs(prefer: Vec<Vec<i32>>, n: i32) {
    let wParter: Vec<i32> = Vec::with_capacity(n);


}
