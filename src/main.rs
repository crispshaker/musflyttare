fn main() -> ! {
    let enigo: enigo::Enigo = enigo::Enigo::new();
    const TIME_FOR_CHANGE: std::time::Duration = std::time::Duration::from_secs(30);
    const MEAN_MOVMENT: i32 = 5;

    loop {
        if !mouse_postion_changed(TIME_FOR_CHANGE, &enigo) {
            let current_mouse_position: (i32, i32) =
                enigo::MouseControllable::mouse_location(&enigo);
            std::thread::spawn(move || {
                drag_mouse(current_mouse_position, MEAN_MOVMENT).expect("Failed to move mouse.");
            });
        }
    }
}

fn drag_mouse(
    current_mouse_position: (i32, i32),
    mean_movment: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    fn move_mouse(
        enigo: &mut enigo::Enigo,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        duration_ms: i32,
        speed: i32,
    ) {
        let steps: i32 = duration_ms / speed;
        for step in 0..=steps {
            let current_x: i32 = start_x + ((end_x - start_x) * step) / steps;
            let current_y: i32 = start_y + ((end_y - start_y) * step) / steps;

            enigo::MouseControllable::mouse_move_to(enigo, current_x, current_y);
            std::thread::sleep(std::time::Duration::from_millis(speed as u64));
        }
    }

    let mut enigo: enigo::Enigo = enigo::Enigo::new();
    let normal_distribution: rand_distr::Normal<f64> =
        rand_distr::Normal::new(0.0, mean_movment as f64)?;

    let start_x: i32 = current_mouse_position.0;
    let start_y: i32 = current_mouse_position.1;

    let duration_ms: i32 = 200;
    let speed: i32 = 5;

    let change1: i32 =
        rand_distr::Distribution::sample(&normal_distribution, &mut rand::thread_rng()) as i32;
    let change2: i32 =
        rand_distr::Distribution::sample(&normal_distribution, &mut rand::thread_rng()) as i32;

    let end_x: i32 = current_mouse_position.0 + change1;
    let end_y: i32 = current_mouse_position.1 + change2;

    move_mouse(
        &mut enigo,
        start_x,
        start_y,
        end_x,
        end_y,
        duration_ms,
        speed,
    );

    std::thread::sleep(std::time::Duration::from_millis(
        rand_distr::Distribution::sample(
            &rand_distr::Normal::new(0.0, mean_movment as f64)?,
            &mut rand::thread_rng(),
        ) as u64,
    ));

    move_mouse(
        &mut enigo,
        end_x,
        end_y,
        start_x,
        start_y,
        duration_ms,
        speed,
    );
    Ok(())
}

fn mouse_postion_changed(time: std::time::Duration, enigo: &enigo::Enigo) -> bool {
    let pos_1: (i32, i32) = enigo::MouseControllable::mouse_location(enigo);
    std::thread::sleep(time);
    pos_1 != enigo::MouseControllable::mouse_location(enigo)
}
