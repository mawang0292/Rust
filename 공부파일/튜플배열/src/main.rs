fn main() 
{
    //튜플
    let tup : (i32,f64,u8) = (500,6.4,1);
    //값들을 집합시켜 튜플화
    //튜플 : 다양한 타입의 몇개의 숫자를 집합시켜 하나의 복합타입을 만드는것
    //타입이 동일하지 않아도 상관없음.
    let (x,y,z)=tup;
    //tup에는 튜플전체가 bind된다.
    //개별값을 빼오기 위해선 ,패턴매칭을 통하여 튜플의값을 구조해체 시켜야한다.

    println!("x의 값은 : {}",x);
    println!("y의 값은 : {}",y);
    println!("z의 값은 : {}",z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // . (마침표)를 통해서 원하는 값의 인덱스를 넣는것을 통해 직접적으로 접근할수 있다.
    println!("five_hundred의 값은 : {}",five_hundred);
    println!("six_point_four의 값은 : {}",six_point_four);
    println!("one의 값은 : {}",one);

    //배열
    //튜플과 다르게 배열의 모든 요소는 같은 타입이여야 한다.
    //Rust의 배열은 모두 고정된 값을 가진다.불변성
    //배열이 유용할때는 heap보다 stack 에 할당하는것을 원하거나 고정된 숫자의 요소를 갖는다고 확실할때.
    //배열이나 vector 중 뭘 사용할지 확실하지 않을땐 vector를 사용하자.

    let a =[1,2,3,4,5];
    let first =a[0];
    let second=a[1];

    println!("first의 값은 : {}",first);
    println!("second의 값은 : {}",second);
}
