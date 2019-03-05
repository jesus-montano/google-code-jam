use std::io;
use std::str::FromStr;

use std::cmp;
fn main() {

    let mut T=0;
    
     let mut input = String::new();
        
        let inp: u32 = u32::from_str(&input.trim()).unwrap();
        T=inp;
        for i in 1..T{
            let mut a =0;
            
             let mut input = String::new();
        
        let inp: u32 = u32::from_str(&input.trim()).unwrap();
        a=inp;
        let mut cols =cmp::max(3,(a+2)/3);
        
        
       
        let mut cnts =vec![0; (cols as usize)+2];
        let mut ptr = 1;
		let mut area = 0;
        

        let mut seen: Vec<Vec<bool>>=  Vec::new(); 
       
        for i in 0..4{
            let mut v: Vec<bool>= Vec::new();
            for i in 0..cols+5{
                v.push(false);
             }
             seen.push(v);
        }
         while area < cols*3 {
            
            while cnts[ptr] == 3{
                 
                 ptr+=1;
                
                let mut qi = ptr+1;
                qi =cmp::min(qi,(cols as usize)-1);
                let mut res:[i32;2]= prepare(2,qi as i32); 
                if res[0] == 0 && res[1] == 0{
					break;}
                    if !seen[res[0] as usize][res[1] as usize] {
					cnts[res[1] as usize]+=1;
					seen[res[0] as usize][res[1] as usize] = true;
					area+=1;
                  
				}
                           
                   
             }
        }
        }
 }
fn prepare( i:i32,  j:i32)->[i32; 2] {
	let mut xs: [i32; 2] = [1, 2];	
		println!("{} {}", i, j);
        xs[0]=i;
        xs[1]=j;
	xs}