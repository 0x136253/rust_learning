fn main() {
    // let ans = build_nxt("abcab");
    // // for i in ans {
    // //     print!("{} ",i)
    // // }
    // search("abcabdddabcabc", "abcab", &ans);

    let s = String::from("barfoofoobarthefoobarman");
    let words:Vec<String> = vec!["bar".to_string(),"foo".to_string(),"the".to_string()];
    find_substring(s, words);
}

fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut search_ans:Vec<Vec<i32>> = Vec::new();
    let mut nxts:Vec<Vec<i32>> = Vec::new();
    let mut ans:Vec<i32> = Vec::new();
    let mut ans_array:Vec<i32> = Vec::new();
    for word in words.iter(){
        nxts.push(build_nxt(word));
    }
    for (pos,word) in words.iter().enumerate(){
        search_ans.push(search(&s,word,&nxts[pos]))
    }
    for item in search_ans.iter(){
        for item2 in item.iter() {
            ans_array
        }
    }
    ans
}

fn search(search_str:&str,patt:&str,nxt:&Vec<i32>) -> Vec<i32>{
    let mut tar:usize = 0;
    let mut pos:usize = 0;
    let s = search_str.as_bytes();
    let p = patt.as_bytes();
    let mut ans:Vec<i32> = Vec::new();
    while tar < s.len(){
        if s[tar] == p[pos] {
            tar+=1;
            pos+=1;
        }
        else if pos != 0 {
            pos = nxt[pos-1] as usize;
        }
        else{
            tar += 1;
        }
        if pos == p.len() {
            ans.push((tar-pos) as i32);
            pos = nxt[pos-1] as usize;
        }
    }
    ans
}

fn build_nxt(patt:& str) -> Vec<i32>{
    let mut nxt:Vec<i32> = Vec::new();
    let mut x:usize = 1;
    let mut now:usize = 0;
    let p = patt.as_bytes();
    nxt.push(0);
    while x < p.len(){
        if p[now] == p[x] {
            now+=1;
            x+=1;
            nxt.push(now as i32);
        }
        else if now != 0{
            now=nxt[now-1] as usize;
        }
        else{
            nxt.push(0);
            x+=1;
        }
    }
    nxt
}
