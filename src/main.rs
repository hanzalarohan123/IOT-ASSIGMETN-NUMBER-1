// fn main()
// {
 
// let x = 3;
// match x 
// {
//  3 => println!("ndbf {}",x),
//  _ => println!("not found"),
// }

// }

// fn main( )
// { //Generate counting till 100. Hint you can break loop
// let mut x = 0;
// loop{
// println!("Hello {}",x);
//  x+=1;
// if x == 100
// {
//     break;
// }
// }
// }

// fn main()
//  {
// let x = 3;
// loop {
// println!("I love Stranger things");
// if x==3
// {
// println!("I don’t love it anymore!");
// break;
// }
// }
// }

// fn main()
// {
// let _number1:i32 = 12;
// let _number2:i32 = 12;
// if _number1 == _number1
//  {
// println!("Number1 and Number2 are equal\n");
// }
// else
// {
// println!("Number1 and Number2 are not equal\n");
// }
// }

//fn main()
// {
// let mut  x = 5;
// loop
// {
// x += x -- 3;
// println!("{}", x);
// if x % 5 == 0
// {
//       break;
// }
// }
// }
// use std::io;
// fn main() 
// {
// let mut sum=0;
// for i in 0..10
// {
// let mut data = String::new();
// println!("Enter integer {}",i+1);
// io::stdin().read_line(&mut data);
// let mut data  :i64 = data.trim().parse().unwrap();
// sum = sum + data;
// }
// println!("The sum is: {}",sum);
// println!("The average is: {}",sum/10);
// }

// fn main()
// {
// let x = 3;
// for i in 0..x{
// println!("Number is :{} and cube is :{}",i,(i as i32).pow(3) ); //pow is method for
// //power
// // (num as datatype).pow(power)
// }
// }


// fn main() 
// {
// let names = ["Ali", "Zain", "Naufil"];
// for name in names.iter() 
// {
// match name {
// &"Ali" => println!("There is a rockstar among us!"),
// _ => println!("Hello {}", name),
// }
// }
// }

// fn main()
// {
// let array: [i32; 5] = [8, 9, 3, 4, 5];
// let mut sum = 0;
// println!("Find sum of all elements of array:");
// println!("----------------------------------");
// for n in  0..5
// {
// sum += array[n];
// }
// println!("Sum of all elements stored in the array is : {}", sum);
// }

// fn main()
// {
// let numbers = [20, 30, 25, 35, 16, 60, 100];
// //calculate sum of all array elements
//  let mut sum :i32  = 0;
// for a in 0..numbers.len() {
// sum = sum + numbers[a];
// }
// //calculate average value
// let len=numbers.len();
// let average :f32 = sum as f32 / len as f32;
// println!("Average value of the array elements is : {}" , average);
// }

//This program will calculate profit and loss on a transaction.
// fn main()
// {
// let cost_price = 12;
// let sale_price = 13;
// let mut profit_lost = 0;

// if sale_price>cost_price //calculate profit
// {
// profit_lost = sale_price-cost_price;
// println!("You can booked your profit amount : {}", profit_lost);
// }
// else if (cost_price>sale_price) //calculate loss
// {
// profit_lost = cost_price-sale_price;
// println!("You got a loss of amount : {}", profit_lost);
// }
// else //No Profit No Loss
// {
// println!("You are running in no profit no loss condition.");
// }
// }

