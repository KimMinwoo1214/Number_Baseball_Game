extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut random_num = Vec::new(); //벡터만들기

    for i in 0..4{ //이걸로 나중에 원하는 숫자 만큼 수정 가능 지금은 4개
        let mut ran_num = rand::thread_rng().gen_range(0..10); //랜덤하게 숫자 뽑기
        loop {
        if random_num.contains(&ran_num) { //중복된 숫자가 있으면 다시 숫자 뽑기
            ran_num = rand::thread_rng().gen_range(0..10); 
        }
        else {
            break;
        }
        }
        random_num.push(ran_num); //숫자 집어넣기
    }
    
    let mut trial = 0; //시도한 횟수 저장 변수

    loop {
        let mut numbers = Vec::new(); //벡터 만들고 초기화까지
        println!("Enter the numbers, separated by enter:"); //숫자치고 엔터치기
        let mut strike = 0;
        let mut ball =0;

        for i in 0..4 { //4번 반복
            let mut number = String::new(); 
            io::stdin().read_line(&mut number).expect("Failed to read line");
            let number: i32 = number.trim().parse().expect("Invalid input");
            numbers.push(number); //입력한 숫자 집어넣기
        }

        let guess = Vec::clone(&numbers); //복사하고 원래 있는것은 지워짐

        for i in 0..4{
            if random_num.contains(&guess[i]) { //그 숫자가 포함되면 실행
                if guess[i] == random_num[i]{ //위치까지 같으면 스트라이크
                    strike += 1;
                }
                else {
                    ball += 1; //아니면 볼
                }
            }
        }

        println!("You enter: {:?}", guess);
        println!("Strike : {}", strike);
        println!("Ball: {}", ball);
        trial += 1;    
        
        if strike == 4{ //정답 맞추기
            println!("Homerun!!!!!");
            break;
        }
          
    }
    
    println!("You Win! The scret number is {:?}", random_num);
    println!("You try {} times.", trial);
    }