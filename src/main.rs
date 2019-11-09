fn main(){
  let r = calculate(1, 1, 5, 8);

  for i in r {
    println!("{}", i);
    
  }
  
  let p = permutate(1,1,5,8);
  for i in p {
    println!("{:?}", i);
    
  }
  let r2 = calc_all_permutatiton(1,1,5,8);
  for i in r2 {
    println!("{}", i);
    
  }
  
}

#[derive(Debug)]
enum Operator {
  ADD,
  SUB,
  TSUB,
  MUL,
  DIV,
  TDIV,
}

fn calc_all_permutatiton(a: u32, b: u32, c: u32, d: u32) -> Vec<f64> {
  let mut vec = Vec::new();
  let p = permutate(a, b, c, d);
  for terms in p {
    let mut vals = calculate(terms.0, terms.1, terms.2, terms.3);
    vec.append(&mut vals);
  }
  vec
}

fn calculate(a: u32, b: u32, c: u32, d: u32) -> Vec<f64> {
  let mut temp_vec = Vec::new();
  let all_operator = [Operator::ADD, 
                      Operator::SUB, 
                      Operator::MUL, 
                      Operator::DIV, 
                      Operator::TSUB, 
                      Operator::TDIV 
                     ];
  for i in all_operator.iter() {
    for j in all_operator.iter() {
      for k in all_operator.iter() {
        let t = operate(k, 
                  operate(i, a as f64, b as f64), 
                  operate(j, c as f64, d as f64));
        if t == 10.0 {
          println!("Great! : ({} {:?} {}) {:?} ({} {:?} {})", a, i, b, k, c, j, d);
        } 
        temp_vec.push(t);
      }
    }
  }
  for i in all_operator.iter() {
    for j in all_operator.iter() {
      for k in all_operator.iter() {
        let t = operate(k, 
                  operate(j, 
                    operate(i, 
                      a as f64, 
                      b as f64), 
                    c as f64), 
                  d as f64);
        if t == 10.0 {
          println!("Great! : (({} {:?} {}) {:?} {}) {:?} {}", a, i, b, j, c, k, d);
        } 
        temp_vec.push(t);
      }
    }
  }
  return temp_vec;
}

fn operate(op: &Operator, term1: f64, term2: f64) -> f64{
  match op {
    Operator::ADD => term1 + term2,
    Operator::SUB => term1 - term2,
    Operator::MUL => term1 * term2,
    Operator::DIV => term1 / term2,
    Operator::TSUB => term2 - term1,
    Operator::TDIV => term2 / term1,
  }
}

fn permutate(a: u32, b: u32, c: u32, d: u32) -> Vec<(u32, u32, u32, u32)> 
{
  let mut temp_vec = Vec::new();
  for i in 1..5 { //a place
    for j in 1..5 { // b place
      if j == i
        {continue;}
      for k in 1..5 { //c place
        if k == i || k == j {
          continue;
        }
        for l in 1..5 { //d place
          if l == i || l == j || l == k {
            continue;
          }
          let mut p1 : u32 = 0;
          let mut p2 : u32 = 0;
          let mut p3 : u32 = 0;
          let mut p4 : u32 = 0;

          match i {
            1 => p1 = a,
            2 => p2 = a,
            3 => p3 = a,
            4 => p4 = a,
            _ => {},
          } 
          
          match j {
            1 => p1 = b,
            2 => p2 = b,
            3 => p3 = b,
            4 => p4 = b,
            _ => {},
          } 
          
          match k {
            1 => p1 = c,
            2 => p2 = c,
            3 => p3 = c,
            4 => p4 = c,
            _ => {},
          } 
          
          match l {
            1 => p1 = d,
            2 => p2 = d,
            3 => p3 = d,
            4 => p4 = d,
            _ => {},
          } 
          
          temp_vec.push((p1, p2, p3, p4));
        }     
      }
    }
    
  }   
  temp_vec
}


