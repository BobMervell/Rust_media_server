use crate::domain::person::credits::Cast;

pub fn is_credited(cast: &Cast) -> bool {
    !cast.character().contains("uncredited")
}

pub fn is_main_crew(crew: &crate::domain::person::credits::Crew) -> bool {
    match crew.department() {
        "Directing" => is_important_directing(crew.job()),
        "Production" => is_important_production(crew.job()),
        "Camera" => is_important_camera(crew.job()),
        "Sound" => is_important_sound(crew.job()),
        "Visual Effects" => is_important_vfx(crew.job()),
        "Writing" => is_important_writing(crew.job()),
        "Art" => is_important_art(crew.job()),
        "Costume & Make-Up" => is_important_costumes_makeup(crew.job()),
        _ => false,
    }
}

fn is_important_directing(job: &str) -> bool {
    matches!(job, "Director" | "Co-Director")
}

fn is_important_production(job: &str) -> bool {
    matches!(job, "Producer")
}

fn is_important_camera(job: &str) -> bool {
    matches!(job, "Director of Photography")
}

fn is_important_sound(job: &str) -> bool {
    matches!(job, "Original Music Composer" | "Sound Designer")
}

fn is_important_vfx(job: &str) -> bool {
    matches!(
        job,
        "VFX Supervisor" | "Visual Effects Supervisor" | "Visual Effects Art Director"
    )
}

fn is_important_writing(job: &str) -> bool {
    matches!(
        job,
        "VFX Supervisor" | "Visual Effects Supervisor" | "Visual Effects Art Director"
    )
}

fn is_important_art(job: &str) -> bool {
    matches!(
        job,
        "Writer"
            | "Original Film Writer"
            | "Co-Writer"
            | "Scenario Writer"
            | "Teleplay"
            | "Screenplay"
    )
}

fn is_important_costumes_makeup(job: &str) -> bool {
    matches!(
        job,
        "Writer"
            | "Original Film Writer"
            | "Co-Writer"
            | "Scenario Writer"
            | "Teleplay"
            | "Screenplay"
    )
}
