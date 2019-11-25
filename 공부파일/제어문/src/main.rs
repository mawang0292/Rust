fn main() 
{
    let number = 3;

    //rust의 if문은 boolean형식   
    if number < 5 
    {
        println!("true");
    } 
    else 
    {
        println!("false");
    }

    let number = 6;

    if number % 4 == 0 
    {
        println!("number is divisible by 4");
    } 
    else if number % 3 == 0 
    {
        println!("number is divisible by 3");
    } 
    else if number % 2 == 0 
    {
        println!("number is divisible by 2");
    } 
    else 
    {
        println!("number is not divisible by 4, 3, or 2");
    }
    //6은 2에도 나눠지지만 실행시키면 두번째 else if문만 출력이된다.
    //그 이유는 else if 문은 rust가  처음에 참이되는 블록만 찾아 실행하고,
    //찾게되면 더이상 검사를 하지 않기 때문.

    //if문이 표현식이기 때문에 let구문의 우측에 사용할수있다
    let condition = true;
    let number = if condition 
    {
        5
    } 
    else
    {
        6
    };

    println!("The value of number is: {}", number);
}
