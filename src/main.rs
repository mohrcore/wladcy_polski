extern crate rand;

use std::io::BufRead;
use rand::distributions::{Range,  IndependentSample};

fn main() {

    let wladcy = vec![
        "Popiel",
        "Piast",
        "Siemowit",
        "Lestek",
        "Siemomysł",
        "Mieszko I",
        "Bolesław Chrobry",
        "Mieszko II",
        "Kazimierz Odnowiciel",
        "Bolesław Śmiały",
        "Władysław Herman",
        "Bolesław Krzywousty",
        "ROZBICIE",
        "Władysław Łokietek",
        "Kazimierz Wielki",
        "Ludwik Andegaweński",
        "Św Jadwiga",
        "Władysław Jagiełło",
        "Władysław Warneńczyk",
        "Kazimierz Jagiellończyk",
        "Jan Olbracht",
        "Aleksander Jagiellończyk",
        "Zygmunt Stary",
        "Zygmunt August",
        "Henryk Walezy",
        "Stefan Batory",
        "Zygmunt III Waza",
        "Władysław IV",
        "Jan Kazimierz",
        "Michał Korybut Wiśniowiecki",
        "Jan III Sobieski",
        "August II Mocny",
        "Stanisław Leszczyński",
        "August III Sas",
        "Stanisław August Poniatowski",
        "ZABORY"
    ];

    let mut stdin = std::io::stdin();
    let mut rng = rand::thread_rng();
    let mut  range = Range::new(0, wladcy.len() - 2);
    
    let mut i: f64 = 0.0;
    let mut p: f64 = 0.0;
    loop {
        let id = range.ind_sample(&mut rng);
        println!("{}", wladcy[id]);
        let line = read_line(&mut stdin);
        match line.as_str() {
            "[quit]" => {
                println!("Pa pa");
                break;
            }
            "[range]" => {
                match get_range(&mut stdin, &wladcy) {
                    Ok(r) => range = Range::new(r.0, r.1),
                    Err(e) => {
                        println!("An error has occured: {}", e);
                        continue;
                    }
                }
                continue;
            },
            _ => (),
        }
        i += 1.0;
        if(line == wladcy[id + 1]) {
            p += 1.0;
            println!("Zajebiście! Twój performace wynosi {}%", 100.0 * p / i);
        } else {
            println!("{} KURWA, {} CHUJU, NAUCZ SIĘ!!! Twój performace wynosi: {}%", wladcy[id + 1], wladcy[id + 1], 100.0 * p / i);
        }
    }
}

fn read_line(stdin: &mut std::io::Stdin) -> String {
    stdin.lock().lines().next().unwrap().unwrap()
}

fn get_range(stdin: &mut std::io::Stdin, v: &Vec<&str>) -> Result<(usize, usize), String> {

    println!("Od: ");
    let input = read_line(stdin);
    let mut i1 = 0;
    loop {
        if i1 >= v.len() { return Err("Item not found".to_string())}
        if input == v[i1] { break; }
        i1 += 1;
    }
    println!("Do: ");
    let input = read_line(stdin);
    let mut i2 = 0;
    loop {
        if i2 >= v.len() { return Err("Item not found".to_string())}
        if input == v[i2] { break; }
        i2 += 1;
    }

    if i1 >= i2 {
        return Err("Bad range".to_string());
    }

    return Ok((i1, i2));
}