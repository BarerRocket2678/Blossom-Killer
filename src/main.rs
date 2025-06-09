use std;

fn input() -> String{
  

  let mut temp = String::new();
  std::io::stdin()
  .read_line(&mut temp)
  .expect("Failed to read line");


  if let Some('\n')=temp.chars().next_back() {
      temp.pop();
  }
  if let Some('\r')=temp.chars().next_back() {
    temp.pop();
  }

  return temp;
  
}

fn main(){
  
  let mut used: Vec<String> = vec![];
  let mut letters: Vec<String> = vec![];

  let mut count:u8 = 0;


  loop{

    count += 1;

    println!("Letter {}: ", count);
    let temp = input();
    
    letters.push(temp);

    println!("{:?}", letters);

    if count >= 7 {
      break;
    }
  
  }

  count = 0;
  let mut gold_leaf = String::new();
  let mut required_leaf = String::new();
  let mut nonspecial_leafs: Vec<String> = vec![];

  println!("Required leaf: ");
  required_leaf = input();

  
  loop{

    count += 1;

    println!("Gold leaf: ");
    gold_leaf = input();
    nonspecial_leafs = letters.clone();
    nonspecial_leafs.remove(nonspecial_leafs.iter()
    .position(|x| *x == gold_leaf).expect("Not found"));

    println!("{:?}", nonspecial_leafs);

    let mut score = 0;
    let mut high_score = 0;
    let mut works = true;
    let mut best_string = String::new();
    let mut bonus = true;
    let mut scores: Vec<usize> = vec![];
    let mut output: Vec<String> = vec![];
    for line in std::fs::read_to_string("dictionary.txt").unwrap().lines() {
      score = 0;

      
      works = true;
      
      for char in line.chars() {
        if letters.contains(&char.to_string()) {

          if char.to_string() == gold_leaf.to_string() {
            score += 5;
          }
  
        } else {
          works = false;
  
        }
      }
      
      match line.len() {
        1 => works = false,
        2 => works = false,
        3 => works = false,
        4 => score += 2,
        5 => score += 4,
        6 => score += 6,
        7 => score += 12,
        _ => score += 12 + (line.len() - 7)*3
        
      }
      

      if used.contains(&line.to_string()) {
        works = false;
      }
      
      if line.contains(&required_leaf) == false {
        works = false;
      }

      bonus = true;
      for char in &letters {
        if line.contains(char) == false {
          bonus = false;
          
        }
      }

      if bonus == true {
        score += 7;
      }

        

      if works == true {
        scores.push(score);
        output.push(line.to_string() + ":" + &score.to_string());
        if score > high_score {
          high_score = score;
          best_string = line.to_string();
        
        }
      }


    }

    scores.sort_unstable();

    let mut printed: Vec<String> = vec![];
    for x in &scores {
      
      for y in &output {
        let parts: Vec<&str> = y.split(":").collect();
        if parts[1].parse::<usize>().unwrap() == *x {
          if printed.contains(y) == false {
            println!("{}\n", y);
            printed.push(y.to_string());
          }
        }
      }
    }
    
    println!("Enter used word: ");
    used.push(input());
    
    
    if count >= 12 {
      break;
    
    }
  }
}