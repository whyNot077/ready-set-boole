use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::ex03::ast::{get_ast, ASTNode};
use crate::ex03::boolean_evaluation::calculate;
use std::collections::HashSet;

/// 모든 변수 조합에 대해 수식을 평가하여 진리표를 생성하는 함수
fn generate_truth_table(formula: &str) -> Result<String> {
    let ast = get_ast(formula).context("Failed to create AST from formula")?;
    let vars = extract_and_sort_vars_from_ast(&ast);
    let mut output = create_truth_table_header(&vars);
    output.push_str(&evaluate_combinations(&ast, &vars)?);
    Ok(output)
}


fn extract_and_sort_vars_from_ast(ast: &ASTNode) -> Vec<char> {
    let mut vars = HashSet::new();
    extract_vars(ast, &mut vars);
    
    let mut vars: Vec<char> = vars.into_iter().collect();
    vars.sort();
    vars
}

/// 재귀적으로 AST를 순회하며 변수를 추출하는 헬퍼 함수
fn extract_vars(node: &ASTNode, vars: &mut HashSet<char>) {
    match node {
        ASTNode::Operand(c) => {
            if c.is_alphabetic() {
                vars.insert(*c);
            }
        }
        ASTNode::Operator(_, left, right_opt) => {
            extract_vars(left, vars);
            if let Some(right) = right_opt {
                extract_vars(right, vars);
            }
        }
    }
}

/// 진리표의 헤더와 구분선을 생성하는 함수
fn create_truth_table_header(vars: &[char]) -> String {
    let mut header = String::new();
    header.push('|');
    for &var in vars {
        header.push_str(&format!(" {} |", var));
    }
    header.push_str(" = |\n|");
    header.push_str(&format!("{}\n", "---|".repeat(vars.len() + 1)));
    header
}


fn evaluate_combinations(ast: &ASTNode, vars: &[char]) -> Result<String> {
    let mut output = String::new();
    let num_vars = vars.len();

    for i in 0..(1 << num_vars) {
        output.push('|');
        let mut eval_map = HashMap::new();
        
        // 각 변수에 대해 현재 조합에 해당하는 값을 할당
        for (j, &var) in vars.iter().enumerate() {
            let value = (i >> (num_vars - j - 1)) & 1 == 1;
            eval_map.insert(var, value);
            output.push_str(&format!(" {} |", if value { 1 } else { 0 }));
        }

        // AST를 사용하여 수식 평가
        let result = evaluate_ast(ast, &eval_map);
        output.push_str(&format!(" {} |\n", if result { 1 } else { 0 }));
    }

    Ok(output)
}

fn evaluate_ast(node: &ASTNode, var_map: &HashMap<char, bool>) -> bool {
    match node {
        ASTNode::Operand(c) => {
            match c {
                '0' => false,  // '0'은 false를 나타냄
                '1' => true,   // '1'은 true를 나타냄
                _ if c.is_alphabetic() => *var_map.get(c).unwrap_or_else(|| panic!("Variable {} not found in map", c)),
                _ => panic!("Unexpected operand: {}", c),  // 예상하지 못한 피연산자는 패닉 발생
            }
        }
        ASTNode::Operator('!', left, _) => {
            let val = evaluate_ast(left, var_map);
            !val
        }
        ASTNode::Operator(op, left, right_opt) => {
            let left_val = evaluate_ast(left, var_map);

            // `right`가 `Some`이면 그 값을 평가하고, `None`이면 기본값으로 처리
            let right_val = match right_opt {
                Some(right) => evaluate_ast(right, var_map), // `Some`이면 평가
                None => false, // 단항 연산자는 여기서 처리되지 않으므로 기본값은 사용되지 않음
            };

            calculate(*op, left_val, right_val)
        }
    }
}

/// 진리표를 출력하는 함수
pub fn print_truth_table(formula: &str) {
    match generate_truth_table(formula) {
        Ok(output) => print!("{}", output),
        Err(e) => eprintln!("Error generating truth table: {}", e),
    }
}



#[test]
fn test_truth_table() {
    // 기존 테스트 케이스

    // AB&C| 테스트 케이스
    let res = generate_truth_table("AB&C|").unwrap();
    assert_eq!(
        res,
        "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n"
    );

    // 잘못된 입력 처리
    assert!(generate_truth_table("AB&C|&").is_err());

    // A! 테스트 케이스
    let res = generate_truth_table("A!").unwrap();
    assert_eq!(
        res,
        "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 0 |\n"
    );

    // A 테스트 케이스
    let res = generate_truth_table("A").unwrap();
    assert_eq!(
        res,
        "| A | = |\n|---|---|\n| 0 | 0 |\n| 1 | 1 |\n"
    );

    // AB| 테스트 케이스
    let res = generate_truth_table("AB|").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 0 |\n| 0 | 1 | 1 |\n| 1 | 0 | 1 |\n| 1 | 1 | 1 |\n"
    );

    // AB& 테스트 케이스
    let res = generate_truth_table("AB&").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 0 |\n| 0 | 1 | 0 |\n| 1 | 0 | 0 |\n| 1 | 1 | 1 |\n"
    );

    // AB^ 테스트 케이스
    let res = generate_truth_table("AB^").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 0 |\n| 0 | 1 | 1 |\n| 1 | 0 | 1 |\n| 1 | 1 | 0 |\n"
    );

    // AB> 테스트 케이스
    let res = generate_truth_table("AB>").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 1 |\n| 0 | 1 | 1 |\n| 1 | 0 | 0 |\n| 1 | 1 | 1 |\n"
    );

    // AB= 테스트 케이스
    let res = generate_truth_table("AB=").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 1 |\n| 0 | 1 | 0 |\n| 1 | 0 | 0 |\n| 1 | 1 | 1 |\n"
    );

    // AA= 테스트 케이스
    let res = generate_truth_table("AA=").unwrap();
    assert_eq!(
        res,
        "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 1 |\n"
    );

    // 추가된 새로운 테스트 케이스

    // ABC== 테스트 케이스
    let res = generate_truth_table("ABC==").unwrap();
    assert_eq!(
        res,
        "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 1 |\n| 0 | 1 | 1 | 0 |\n| 1 | 0 | 0 | 1 |\n| 1 | 0 | 1 | 0 |\n| 1 | 1 | 0 | 0 |\n| 1 | 1 | 1 | 1 |\n"
    );

    // AB>C> 테스트 케이스
    let res = generate_truth_table("AB>C>").unwrap();
    assert_eq!(
        res,
        "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 1 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 0 |\n| 1 | 1 | 1 | 1 |\n"
    );

    // AB>A>A> 테스트 케이스
    let res = generate_truth_table("AB>A>A>").unwrap();
    assert_eq!(
        res,
        "| A | B | = |\n|---|---|---|\n| 0 | 0 | 1 |\n| 0 | 1 | 1 |\n| 1 | 0 | 1 |\n| 1 | 1 | 1 |\n"
    );
}
