use crate::{
    admin::AdminHandle, booster::BoosterHandle, brightspace::BrightspaceHandle, john::JohnHandle,
};

pub mod admin;
pub mod booster; // <<< WORK IN HERE
pub mod brightspace;
pub mod john;

#[tokio::main]
async fn main() {
    // Step 1: Construct (which also starts up all backends for) All Actors
    let john_handle = JohnHandle::new().await;
    let brightspace_handle = BrightspaceHandle::new().await;
    // let booster_handle = BoosterHandle::new().await;
    let admin_handle = AdminHandle::new().await;

    // Step 2: Orchestrate Actors
    john_handle
        .set_brightspace(brightspace_handle.clone())
        .await;
    brightspace_handle.set_admin(admin_handle.clone()).await;

    // Step 3: Use Actors
    john_handle
        .register_new_student("Aarya Patel".to_string())
        .await;
    john_handle
        .assign_grade_to_student("Aarya Patel".to_string(), 58.0)
        .await;
    john_handle
        .register_new_student("Dane Hindsley".to_string())
        .await;
    john_handle
        .assign_grade_to_student("Dane Hindsley".to_string(), 53.0)
        .await;
    john_handle
        .report_all_students_and_grades_to_brightspace()
        .await;

    brightspace_handle
        .generate_and_append_student_career_id()
        .await;
    brightspace_handle
        .report_all_students_and_grades_to_admin()
        .await;

    let booster_handle = BoosterHandle::new(admin_handle.clone()).await;
    booster_handle.grade_boost().await;

    let all_student_names: Vec<String> = admin_handle.get_all_student_names().await;
    let all_student_grades: Vec<f64> = admin_handle.get_all_student_grades().await;
    let num_failing_students: usize = admin_handle.count_number_of_failing_students().await;

    // Step 4: Print Results
    println!("names of students:  {:?}", all_student_names);
    println!("grades of students: {:?}", all_student_grades);
    println!("number of students failed: {}", num_failing_students);
}
