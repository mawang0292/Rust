use std::io; //사용자 입력을 받은 다음 출력을하기 위해 io 표준 라이브러리가 필요하므로 use 해서 직접가져옴
use rand::Rng; //thread_rng()
use std::cmp::Ordering; //범위를 가져오는 use문 

fn main()
 {
    println!("업다운게임!");
    let mut round = 1;
    let answer = rand::thread_rng().gen_range(1,101);
    //thread_rng : 특정 난수 생성기 
    //get_range : 두 개의 숫자를 인수로 사용하고 그 사이에 난수를 생성 하한은 포함되지만 상한은 포함되지않으므로 1 , 101을 입력
    println!("정답 : {}",answer);

    loop  //while
    {
        println!("{}번째 회차",round);
        println!("숫자를 입력하세요 : ");

        let mut number = String::new();  //let 불변  let mut  가변
        //string::new 새로운 인스턴스를 반환하는 함수를 호출
        //:: ::new를 나타냄
        //new 새로운 빈 문자열을 생성
        //요약 : 비어있는 새 빈 String 인스턴스에 바인딩 된 가변변수를 생성

        //std::io를 사용하여 표준라이브러리의 입출력기능을 포함했으므로 stdin함수 사용가능
        io::stdin().read_line(&mut number)
        .expect("readline 에러");
        //stdin : 표준입력에 대한 핸들을 나타내는 인스턴스를 반환
        // read_line : 메서드를 호출하여 사용자로부터 입력을 받음
        //&mut guess : read_line에 인수 하나를 전달
        //**** &는 이 인수가 참조임을 나타냄, 참조는 기본적으로 변경 불가능하므로 변경 가능하게 하려면 &guess 대신 &mut guess를 작성해야함. ****


        //expect()
        //Rust는 표준 라이브러리에 Result(io::Result)를 가지고있고 Result의 인스턴스에는 호출 할 수있는 expect메소드가 있다.
        //io::Result가 에러값이면 expect는 프로그램을 중단시키고 예상 한 인수로 전달한 메세지를 표시한다.
        //요약 : read_line 이 정상적인 값을 반환하면 사용자가 입력한 값이 반환되고 에러가 반환될시 입력한 메세지 출력.

        let number : u32 = match number.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        //shadowing
        //u32 : Rust에는 몇가지 내장숫자 유형이 있는데 u32는 부호없는 32비트 정수
        //trim() : 시작과 끝의 공백제거 u32는 숫자만 포함할수 있지만 사용자는 enter를 누르므로 공백문자열 제거
        //Parse() : 문자열을 어떤 종류의 숫자로 구문분석함. Parse는 쉽게 오류가 발생할수 있고, parse는 Result유형을 리턴하므로 expect로 에러메세지를 반환할수도 있음
        //Paser()에서 문자열을 숫자로 변환할수 있으면 num에 기록하고 Ok값이 반환됨, 변환할수 없으면 Err값이 반환됨. (_)은 포괄적인 값

        println!("입력한 숫자 : {}", number);
    
        match number.cmp(&answer)
        {
            Ordering::Less=>println!("업!!!"),   //작으면
            Ordering::Greater=>println!("다운!!!"),    //크면
            Ordering::Equal=>    //같으면
            {
                println!("정답!!!");
                break;  //종료
            }
        }

        round=round+1;
    }  //while end
    //Rust는 표준라이브러리에 난수기능을 포함하지 않으므로 Cargo.toml -> [dependencies] 아래에 rand = "0.3.14" 를 추가하고 cargo build
}  //main end
