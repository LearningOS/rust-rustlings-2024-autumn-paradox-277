// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


// 定义一个枚举类型来表示成绩
#[derive(Debug)]
pub enum Grade {
    Numeric(f32), // 数字成绩
    Alphabetic(String), // 字母成绩
}

// 修改 ReportCard 结构体
pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        match &self.grade {
            Grade::Numeric(value) => format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, value
            ),
            Grade::Alphabetic(letter) => format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, letter
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // 这里改变成绩为字母成绩
        let report_card = ReportCard {
            grade: Grade::Alphabetic("A+".to_string()), // 使用字母成绩
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
