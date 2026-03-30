use crate::domain::person::credits::Cast;

pub fn is_credited(cast: &Cast) -> bool {
    return !cast.character().contains("uncredited");
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
    match job {
        "Director" => true,
        "Co-Director" => true,
        _ => false,
    }
}

fn is_important_production(job: &str) -> bool {
    match job {
        "Producer" => true,
        _ => false,
    }
}

fn is_important_camera(job: &str) -> bool {
    match job {
        "Director of Photography" => true,
        _ => false,
    }
}

fn is_important_sound(job: &str) -> bool {
    match job {
        "Original Music Composer" => true,
        "Sound Designer" => true,
        _ => false,
    }
}

fn is_important_vfx(job: &str) -> bool {
    match job {
        "VFX Supervisor" => true,
        "Visual Effects Supervisor" => true,
        "Visual Effects Art Director" => true,
        _ => false,
    }
}

fn is_important_writing(job: &str) -> bool {
    match job {
        "Writer" => true,
        "Original Film Writer" => true,
        "Co-Writer" => true,
        "Scenario Writer" => true,
        "Teleplay" => true,
        "Screenplay" => true,
        _ => false,
    }
}

fn is_important_art(job: &str) -> bool {
    match job {
        "Art Direction" => true,
        "Co-Art Director" => true,
        "Production Design" => true,
        "Art Designer" => true,
        "Set Designer" => true,
        "Property Master" => true,
        _ => false,
    }
}

fn is_important_costumes_makeup(job: &str) -> bool {
    match job {
        "Costume Designer" => true,
        "Makeup Designer" => true,
        _ => false,
    }
}
