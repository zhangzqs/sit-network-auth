use tokio::time::{sleep, Duration};
use exitfailure::ExitFailure;
use sit_network_auth::{check_network, login};
use structopt::StructOpt;
use std::path::Path;
use rand::prelude::IteratorRandom;
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufRead};


#[derive(Debug)]
struct Student {
    username: String,
    password: String,
}


struct StudentManager {
    students: Vec<Student>,
    rng: rand::rngs::SmallRng,
}

impl StudentManager {
    pub fn new() -> Result<Self, ExitFailure> {
        let os_rng = rand::rngs::OsRng::default();
        let rng = rand::rngs::SmallRng::from_rng(os_rng)?;

        Ok(Self {
            students: vec![],
            rng,
        })
    }

    pub fn from_csv(csv_path: &str) -> Result<Self, ExitFailure> {
        let mut ret = StudentManager::new()?;

        ret.students = std::io::BufReader::new(File::open(Path::new(csv_path))
            .unwrap())
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut cols = line
                    .split(',')
                    .map(|x| x.trim());
                (cols.next().unwrap().to_string(),
                 cols.next().unwrap().to_string())
            })
            .map(|(username, password)| Student { username, password })
            .collect::<Vec<Student>>();

        Ok(ret)
    }

    pub fn choose_randomly(&mut self) -> &Student {
        self.students.iter().choose(&mut self.rng).unwrap()
    }
}

/// 保持网络连接
async fn keep_alive(student_manager: &mut StudentManager) -> Result<(), ExitFailure> {
    //记录是否已经打印过
    let mut has_printed = false;

    if check_network().await? {
        println!("Network has connected!!!");
    }

    loop {

        //如果验证失败了
        if !check_network().await? {
            has_printed = false;  //不打印
            println!("Network auth failure detected.");
            //选一个学号密码
            let stu = student_manager.choose_randomly();
            let (user_name, password) = (
                stu.username.as_str(),
                stu.password.as_str());
            println!("Login by user: {}", stu.username);
            login(user_name, password).await?;
        } else {
            //如果没打印过，那让它打印一次
            if !has_printed {
                has_printed = true;
            }
        }
        //如果联网成功，打印一次
        if !has_printed {
            println!("Network has connected!!!");
            has_printed = true;
        }
        sleep(Duration::from_secs(2)).await;
    }
}

#[derive(Debug, StructOpt)]
struct Input {
    csv_path: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let mut student_manager = StudentManager::from_csv(input.csv_path.as_str())?;

    println!("sit network keeper running!!!");
    loop {
        keep_alive(&mut student_manager).await.unwrap_or_else(|_| {
            println!("Network error!!!");
        });
        sleep(Duration::from_secs(2)).await;
    }
}
