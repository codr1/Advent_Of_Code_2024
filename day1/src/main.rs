//use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let mut startIndex = 0;
    let mut sum = 0;
    
    println!("Hello, world!");
    let mut file = File::open("data")?;
    let mut characters = Vec::new();
    file.read_to_end(&mut characters)?;

    let charsLen = characters.len();

    loop { 
        for index in startIndex..charsLen{
            //If we hit a ' ',  we try to parse out a number
            if characters[index] == b' ' { 
                //println!("Index: {}, Character: {}", index, characters[index]);
                let number = characters[startIndex..index]      
                    .iter()
                    .fold(0, |acc, &c| acc * 10 + (c as u8 - b'0') as i32);

                list1.push(number);
                startIndex = index + 3;
                break;
            }
        }
        
        for index in startIndex..charsLen {
            //If we hit a '\n',  we try to parse out a number
            if characters[index] == b'\n' {
                //println!("Index: {}, Character: {}", index, characters[index]);
                let number = characters[startIndex..index]      
                    .iter()
                    .fold(0, |acc, &c| acc * 10 + (c as u8 - b'0') as i32);

                list2.push(number);
                startIndex = index + 1;
                break;
            }
        }

        if startIndex == characters.len() || startIndex + 1 == characters.len() {
            break;
        }
    }

    list1.sort_unstable();
    list2.sort_unstable();

    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
        println!("{} {} {} {}", list1[i], list2[i], list2[i]-list1[i], sum );
    }
    println!("Sum:{}", sum);


    // Process unique numbers in list1 and multiply by occurrences in list2
    sum = 0;
    let mut prev = None;
    for &num in list1.iter() {
        if prev != Some(num) {
            // Count occurrences in list2
            let occurrences = list2.iter().filter(|&&x| x == num).count() as i32;
            println!("Number {} appears {} times in list2, result: {}", num, occurrences, num * occurrences);
            sum += num * occurrences;
            prev = Some(num);

        }
    }
    println!("Sum: {}", sum);


    Ok(())
}
