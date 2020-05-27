use serde::{Deserialize, Serialize};

#[path = "CourseSection.rs"]
mod course_section;
#[path = "GeneralEducation.rs"]
mod general_education;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub class_sections: Vec<course_section::CourseSection>,
    pub college: String,
    pub contact_hours: Option<f32>,
    pub course_id: String,
    pub delayed_sectioning: Option<String>,
    pub dept_code: String,
    pub description: String,
    pub general_education: Vec<general_education::GeneralEducation>,
    pub grading_option: Option<String>,
    pub in_progress_course: Option<String>,
    pub instruction_type: String,
    pub obj_level_code: String,
    pub on_line_course: bool,
    pub quarter: String,
    pub subject_area: String,
    pub title: String,
    pub units_fixed: Option<f32>,
    pub units_variable_high: Option<f32>,
    pub units_variable_low: Option<f32>,
}
