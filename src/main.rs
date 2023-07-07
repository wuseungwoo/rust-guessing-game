use rand::Rng;
use std::io;

fn main() {
    println!("开始猜数字辣 ==");

    //获取随机可变字符串
    let _rand_num = rand::thread_rng().gen_range(1..=100);

    //给出范围提示
    loop {
        println!("请输入你的数字吧：");
        //创建字符串接收前端输入
        let mut _guess = String::new();
        //前端读取输入赋值给已创建的字符串
        io::stdin().read_line(&mut _guess).expect("读取输入失败！");
        println!("你的数字就是：{}", _guess);
        //将字符串转型为32位数字
        let _guess: u32 = _guess.trim().parse().expect("看看你摁的是数字么？？？");

        //过小提示
        if _rand_num > _guess {
            println!("太小了吧你？")
        }

        //过大提示
        if _rand_num < _guess {
            println!("太大了吧你？")
        }

        //猜中提示
        if _rand_num == _guess {
            println!("实际数字是：{}", _rand_num);
            println!("猜中了，恭喜获得$500兰博基尼优惠券一张！");
            break;
        } else {
            println!("很遗憾没有猜中，安慰奖程睿kiss一枚～！")
        }
    }
}
