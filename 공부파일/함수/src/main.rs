fn main() 
{
    another_function();  //함수호출
    another_function2(5,6);
    let x=five();
    println!("{}",x);
    let result=add(5,6);
    println!("{}",result);
}

//함수정의
//Rust에서는 함수정의의 위치는 중요하지않다.어딘가에 정의 된것만 중요.
fn another_function()
{
    println!("Another function.");
}

//매개변수의 유형을 선언해야한다.
//여러가지 매개변수를 가지려면 쉼표로 구분 가능
fn another_function2(x:i32,y:i32)
{
    println!("The value of x is : {}",x);
    println!("The value of y is : {}",y);
}

//반환값이 있는 함수
//


fn five()->i32
{
    5
    //five()는 매개 변수가없고 반환 값의 유형을 정의하지만
    //함수의본문은 값을 반환하려는 식이므로 ; 이 없다.
    //;이 없으면 반환문
    //return 5;  //로도 사용할수있다
}

//return 을 사용한모습
fn add(x:i32, y:i32)->i32
{
    return x+y;
    // x+y  //로도 사용가능
}
