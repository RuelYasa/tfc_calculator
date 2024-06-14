use std::{collections::VecDeque, io::{stdin, stdout, Read}, process::Output};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeStep {
    HitLight,
    HitMedium,
    HitHard,
    Draw,
    Punch,
    Bend,
    Upset,
    Shrink,
    None,
}
impl ForgeStep {
    pub fn get_effect(&self) -> i32 {
        match self {
            Self::HitLight => -3,
            Self::HitMedium => -6,
            Self::HitHard => -9,
            Self::Draw => -15,
            Self::Punch => 2,
            Self::Bend => 7,
            Self::Upset => 13,
            Self::Shrink => 16,
            Self::None => 0,
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            Self::HitLight => "轻击",
            Self::HitMedium => "击打",
            Self::HitHard => "重击",
            Self::Draw => "牵拉",
            Self::Punch => "冲压",
            Self::Bend => "弯曲",
            Self::Upset => "镦锻",
            Self::Shrink => "收缩",
            Self::None => "无",
        }
    }
}
static OPS: [ForgeStep; 8] = [
    ForgeStep::HitLight,
    ForgeStep::HitMedium,
    ForgeStep::HitHard,
    ForgeStep::Draw,
    ForgeStep::Punch,
    ForgeStep::Bend,
    ForgeStep::Upset,
    ForgeStep::Shrink,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeOrder {
    Last,
    SecLast,
    ThiLast,
    NotLast,
    Any,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForgeRule {
    order: ForgeOrder,
    typ: ForgeStep,
}

pub fn init_optimal_steps_to_target() -> Vec<Vec<ForgeStep>> {
    //广度优先搜索

    let mut pat = vec![vec![]; 151];
    let mut vis = [false; 151];
    vis[0] = true;
    let mut q = VecDeque::new();
    q.push_back((0, vec![]));
    while !q.is_empty() {
        let (front, step) = q.pop_front().unwrap().clone();
        pat[front] = step.clone();
        for op in OPS {
            let next = front as i32 + op.get_effect();
            if next >= 0 && next <= 150 {
                let next: usize = next as usize;
                if !vis[next] {
                    vis[next] = true;
                    pat[next] = step.clone();
                    pat[next].push(op);
                    q.push_back((next, pat[next].clone()));
                }
            }
        }
    }
    pat
}

#[inline]
pub fn get_optimal_steps_to_target(pat: &Vec<Vec<ForgeStep>>, target: i32) -> Vec<ForgeStep> {
    return if target < 0 || target > 150 {
        vec![]
    } else {
        pat[target as usize].clone()
    };
}

pub fn calculate_optimal_steps_to_target(mut target: i32, rules: Vec<ForgeRule>) -> Vec<ForgeStep> {
    let pat = init_optimal_steps_to_target();
    let mut last_steps: Vec<ForgeStep> = Vec::new();

    'outer: for last3 in OPS {
        for last2 in OPS {
            for last1 in OPS {
                let mut legal = 0;
                let output = [last3, last2, last1];
                for rule in &rules {
                    //遍历所有后三步
                    let rule = *rule;
                    legal += if match rule.order {
                        //合法性
                        ForgeOrder::Any => output.contains(&rule.typ),
                        ForgeOrder::NotLast => output.contains(&rule.typ) && output[2] != rule.typ,
                        ForgeOrder::Last => output[2] == rule.typ,
                        ForgeOrder::SecLast => output[1] == rule.typ,
                        ForgeOrder::ThiLast => output[0] == rule.typ,
                    } {
                        1
                    } else {
                        0
                    }
                }
                if legal == 3 {
                    output.iter().for_each(|x|{last_steps.push(*x)});
                    break 'outer;
                }
            }
        }
    }

    let mut required_hits = 0;
    for step in &last_steps {
            target -= step.get_effect();
            if *step == ForgeStep::HitLight {
                required_hits += 1;
            }
    }
    let mut best_power = 0;
    let mut minimum_steps = get_optimal_steps_to_target(&pat, target);
    for power in 1..=required_hits * 2 {
        target -= ForgeStep::HitLight.get_effect();
        let t = get_optimal_steps_to_target(&pat, target);
        if t.len() < minimum_steps.len() {
            minimum_steps = t;
            best_power = power;
        }
    }
    for x in last_steps.iter_mut() {
        if best_power == 0 {
            break;
        }
        if *x == ForgeStep::HitLight {
            if best_power >= 2 {
                best_power -= 2;
                *x = ForgeStep::HitHard;
            } else {
                best_power -= 1;
                *x = ForgeStep::HitMedium;
            }
        }
        if best_power == 0 {
            break;
        }
        if *x == ForgeStep::HitMedium {
            best_power -= 1;
            *x = ForgeStep::HitHard;
        }
    }
    minimum_steps.append(&mut last_steps);
    return minimum_steps;
}

fn read_num() -> usize {
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();
    let num = str.trim().parse().unwrap();
    num
}

fn main() {
    println!("所需锻造点数：");
    let target = read_num();
    if target > 150 {
        println!("输入错误")
    }
    let mut rules = vec![];
    println!("分三行输入三个要求。");
    println!("每个要求的十位为操作要求：  1:击打  2:牵拉  3:冲压  4:弯曲  5:镦锻  6:收缩");
    println!("每个要求的个位为位置要求：  1:末尾  2:倒二  3:倒三  4:非末  5:任意");
    println!("若无要求则输入0");
    for _ in 0..3 {
        let op = read_num();
        if op == 0 {
            continue;
        }
        rules.push(ForgeRule {
            order: match op % 10 {
                1 => ForgeOrder::Last,
                2 => ForgeOrder::SecLast,
                3 => ForgeOrder::ThiLast,
                4 => ForgeOrder::NotLast,
                5 => ForgeOrder::Any,
                _ => panic!("输入错误"),
            },
            typ: match op / 10 {
                1 => ForgeStep::HitLight,
                2 => ForgeStep::Draw,
                3 => ForgeStep::Punch,
                4 => ForgeStep::Bend,
                5 => ForgeStep::Upset,
                6 => ForgeStep::Shrink,
                _ => panic!("输入错误"),
            },
        });
    }

    let output = calculate_optimal_steps_to_target(target as i32, rules);
    println!("操作序列为：");
    output.iter().for_each(|x| {
        print!("{} ", x.get_name());
    });
    println!("\n按任意键继续...");
    let _ = stdin().read(&mut [0u8]);
}
