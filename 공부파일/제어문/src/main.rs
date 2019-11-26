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
        //"six"  //컴파일에러 변수는 단일형식을 가져야한다.
    };

    println!("The value of number is: {}", number);

    //loop 반복문
    let mut counter = 0;
    let result = loop
    {
        counter+=1;
        if counter == 10 
        {
            break counter * 2;  //rust에서 break는 반환값과 같이사용
        }
    };

    println!("{}",result);

    //조건부 루프 while
    let mut number = 3;
    while number != 0
    {
        println!("{}",number);

        number-=1;
    }

    println!("LIFTOFF");

    //for
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index <5
    {
        println!("{}",a[index]);
        index +=1;
    }

    //for문을 이용해 간결하게
    for element in a.iter()
    //iter() 이터레이터 : 순환을 표현할때 사용하는 디자인패턴중 하나
    //컬렉션같은 타입의 요소를 순서대로 접근할 때 사용.
    {
        println!("{}",element);
    }
}
