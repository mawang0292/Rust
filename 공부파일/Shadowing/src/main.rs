fn main() 
{
//변수 선언
   let x = 5;
   println!("x의 값은 : {}",x);
//    x=6;  //컴파일 에러 기본 변수는 불변성이고 x = 6은 불변성재할당에 해당하기 때문.
//    println!("{}",x);

//mut
    let mut y = 5;
    println!("y의 값은 : {}",y);
    y = 6;
    println!("y의 값은 : {}",y);
    //mut 는 bind된 값을  변경할수 있다. 가변성 변수.

//shadowing
   let z = 5;
   let z = z + 1;
   let z = z * 2;
   println!("z의 값은 : {}",z);
   //mut 과 shadowing의 차이점 
   //mut 은 가변성 변수라서 언제든 다시 변수값을 변경하수 있다.
   // shadowing은 let 키워드를 사용하여 언제든 값을 변경할 수 있지만 let 키워드를 사용하지 않고 값을 대입시 컴파일에러 발생( 불변성변수)
   //그럼에도 shadowing을 사용하는 이유는 let 키워드를 사용하여 효과적으로  새 변수를 선언하고
   //값을 변경할수 있으면서도 동일 이름을 사용할수 있다.

   let spaces = "   ";
   let spaces = spaces.len();
   println!("공백 숫자 : {}",spaces);
   //위와 같이 첫 spaces 는 문자열 유형변수이고 두번째 spaces는 숫자 유형변수이지만 
   //shadowing은 다른이름의 변수를사용 하는 것 대신 동일한 이름으로 사용할수 있게 해줌.

   let mut spaces2 = "    ";
//    spaces = spaces.len();  //len() 컴파일 에러

}
